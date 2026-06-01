"""SDXL LoRA training script for music score style transfer.

Trains a LoRA adapter on SDXL's UNet using music score images.
The base SDXL model weights are frozen; only the LoRA parameters
(~20-40M params, < 200MB) are trained.

Usage:
    uv run python models/train_lora.py --config models/configs/lora_boulez.yaml
"""

import logging
import sys
from pathlib import Path

import click
import torch
import torch.nn.functional as F
import yaml
from accelerate import Accelerator
from accelerate.utils import set_seed
from diffusers import AutoencoderKL, DDPMScheduler, UNet2DConditionModel
from diffusers.optimization import get_scheduler
from diffusers.training_utils import cast_training_params
from peft import LoraConfig
from peft.utils import get_peft_model_state_dict
from safetensors.torch import save_file
from torch.utils.data import DataLoader
from tqdm.auto import tqdm
from transformers import AutoTokenizer, CLIPTextModel, CLIPTextModelWithProjection

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from models.utils.caption import ScoreMetadata
from models.utils.dataset import ScoreDataset

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")


def load_models(base_model: str, variant: str = "fp16"):
    """Load frozen SDXL components."""
    dtype = torch.float16 if variant == "fp16" else torch.float32

    noise_scheduler = DDPMScheduler.from_pretrained(base_model, subfolder="scheduler")
    tokenizer_one = AutoTokenizer.from_pretrained(base_model, subfolder="tokenizer", use_fast=False)
    tokenizer_two = AutoTokenizer.from_pretrained(base_model, subfolder="tokenizer_2", use_fast=False)
    text_encoder_one = CLIPTextModel.from_pretrained(base_model, subfolder="text_encoder", torch_dtype=dtype, variant=variant)
    text_encoder_two = CLIPTextModelWithProjection.from_pretrained(base_model, subfolder="text_encoder_2", torch_dtype=dtype, variant=variant)
    vae = AutoencoderKL.from_pretrained(base_model, subfolder="vae", torch_dtype=dtype, variant=variant)
    unet = UNet2DConditionModel.from_pretrained(base_model, subfolder="unet", torch_dtype=dtype, variant=variant)

    for model in [vae, text_encoder_one, text_encoder_two, unet]:
        model.requires_grad_(False)

    return noise_scheduler, (tokenizer_one, tokenizer_two), (text_encoder_one, text_encoder_two), vae, unet


def encode_prompt_sdxl(tokenizers, text_encoders, prompt: str, device: torch.device):
    """Encode a prompt through both SDXL text encoders and concatenate.

    SDXL uses two text encoders:
      - CLIPTextModel (ViT-L): hidden_states[-2] for prompt embeds
      - CLIPTextModelWithProjection (ViT-G): hidden_states[-2] for prompt embeds,
        text_embeds for pooled output (used as add_time_ids conditioning)
    """
    prompt_embeds_list = []
    pooled_prompt_embeds = None

    for tokenizer, text_encoder in zip(tokenizers, text_encoders):
        text_inputs = tokenizer(
            prompt, padding="max_length", max_length=tokenizer.model_max_length,
            truncation=True, return_tensors="pt",
        )
        input_ids = text_inputs.input_ids.to(device)
        with torch.no_grad():
            outputs = text_encoder(input_ids, output_hidden_states=True)

        # Penultimate hidden states from each encoder get concatenated
        prompt_embeds_list.append(outputs.hidden_states[-2])

        # Pooled embeds come from CLIPTextModelWithProjection (second encoder)
        if hasattr(outputs, "text_embeds"):
            pooled_prompt_embeds = outputs.text_embeds

    prompt_embeds = torch.cat(prompt_embeds_list, dim=-1)
    return prompt_embeds, pooled_prompt_embeds


def compute_sdxl_add_time_ids(resolution: int, device: torch.device, dtype: torch.dtype):
    """Create the add_time_ids tensor for SDXL conditioning."""
    original_size = (resolution, resolution)
    crops_coords_top_left = (0, 0)
    target_size = (resolution, resolution)
    return torch.tensor([original_size + crops_coords_top_left + target_size],
                        device=device, dtype=dtype)


def save_lora_weights(unet, path: Path):
    """Save LoRA weights as safetensors with 'unet.' prefix for diffusers."""
    path.parent.mkdir(parents=True, exist_ok=True)
    state_dict = get_peft_model_state_dict(unet)
    # Add 'unet.' prefix so diffusers' load_lora_weights can find the keys
    prefixed = {f"unet.{k}": v for k, v in state_dict.items()}
    save_file(prefixed, str(path))
    logger.info(f"Saved LoRA weights -> {path}")


def generate_validation(unet, vae, text_encoders, tokenizers, noise_scheduler,
                        prompts, resolution, steps, cfg, seed, output_dir, device, dtype):
    """Generate validation images at a training checkpoint."""
    from diffusers import StableDiffusionXLPipeline

    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    pipe = StableDiffusionXLPipeline(
        vae=vae,
        text_encoder=text_encoders[0],
        text_encoder_2=text_encoders[1],
        tokenizer=tokenizers[0],
        tokenizer_2=tokenizers[1],
        unet=unet,
        scheduler=noise_scheduler,
        force_zeros_for_empty_prompt=True,
    )
    # Validation runs full fp32 — safe for VAE, fits 16GB at 1024²
    pipe.to(device=device, dtype=torch.float32)
    generator = torch.Generator(device=device).manual_seed(seed)

    for i, prompt in enumerate(prompts):
        image = pipe(prompt=prompt, num_inference_steps=steps, guidance_scale=cfg,
                     width=resolution, height=resolution, generator=generator).images[0]
        image.save(output_dir / f"val_{i:02d}.png")
        logger.info(f"  Validation: {prompt[:50]}... -> {output_dir / f'val_{i:02d}.png'}")

    del pipe
    torch.cuda.empty_cache()


@click.command()
@click.option("--config", required=True, type=click.Path(exists=True), help="YAML config file")
@click.option("--resume", default=None, type=click.Path(exists=True), help="Resume from checkpoint .safetensors")
def main(config: str, resume: str | None):
    """Train a LoRA adapter on SDXL for music score style transfer."""
    with open(config) as f:
        cfg = yaml.safe_load(f)

    model_cfg, lora_cfg, train_cfg = cfg["model"], cfg["lora"], cfg["training"]
    data_cfg, output_cfg, val_cfg = cfg["data"], cfg["output"], cfg.get("validation", {})
    resolution = train_cfg["resolution"]

    accelerator = Accelerator(
        mixed_precision=train_cfg["mixed_precision"],
        gradient_accumulation_steps=train_cfg["gradient_accumulation_steps"],
    )

    # Parse resume step from checkpoint filename
    resume_step = 0
    if resume:
        import re
        match = re.search(r"checkpoint-(\d+)", str(resume))
        if match:
            resume_step = int(match.group(1))
            logger.info(f"Resuming from step {resume_step}")

    logger.info(f"=== coda-score LoRA Training ===")
    logger.info(f"Base model: {model_cfg['base']}")
    logger.info(f"LoRA rank: {lora_cfg['rank']}, alpha: {lora_cfg['alpha']}")
    logger.info(f"Resolution: {resolution}x{resolution}")
    logger.info(f"Max steps: {train_cfg['max_steps']}")
    if resume_step:
        logger.info(f"Resume from step: {resume_step}")
    logger.info(f"Learning rate: {train_cfg['learning_rate']}")
    logger.info(f"Optimizer: {train_cfg['optimizer']}")

    if train_cfg.get("seed"):
        set_seed(train_cfg["seed"])

    # Model weights are always fp16 (SDXL doesn't publish bf16 weights)
    # mixed_precision in config controls training autocast, not weight dtype
    logger.info("Loading SDXL models...")
    noise_scheduler, tokenizers, text_encoders, vae, unet = load_models(
        model_cfg["base"], model_cfg.get("variant", "fp16")
    )

    lora_config = LoraConfig(
        r=lora_cfg["rank"],
        lora_alpha=lora_cfg["alpha"],
        target_modules=lora_cfg["target_modules"],
        lora_dropout=lora_cfg.get("dropout", 0.05),
    )
    unet.add_adapter(lora_config)

    # Load checkpoint weights if resuming
    if resume:
        from safetensors.torch import load_file
        from peft.utils import set_peft_model_state_dict
        state_dict = load_file(resume)
        missing, unexpected = set_peft_model_state_dict(unet, state_dict)
        if missing:
            logger.warning(f"Missing keys when loading checkpoint: {len(missing)}")
        logger.info(f"Loaded LoRA weights from {resume}")

    cast_training_params(unet, dtype=torch.float32)

    if train_cfg.get("gradient_checkpointing"):
        unet.enable_gradient_checkpointing()

    trainable = sum(p.numel() for p in unet.parameters() if p.requires_grad)
    total = sum(p.numel() for p in unet.parameters())
    logger.info(f"Trainable: {trainable / 1e6:.2f}M / {total / 1e6:.0f}M ({100 * trainable / total:.2f}%)")

    lora_params = [p for p in unet.parameters() if p.requires_grad]
    if train_cfg["optimizer"] == "adamw_8bit":
        import bitsandbytes as bnb
        optimizer = bnb.optim.AdamW8bit(lora_params, lr=train_cfg["learning_rate"])
    else:
        optimizer = torch.optim.AdamW(lora_params, lr=train_cfg["learning_rate"])

    lr_scheduler = get_scheduler(
        train_cfg.get("lr_scheduler", "constant_with_warmup"),
        optimizer=optimizer,
        num_warmup_steps=train_cfg.get("lr_warmup_steps", 100),
        num_training_steps=train_cfg["max_steps"],
    )

    logger.info(f"Loading dataset from {data_cfg['train_dir']}...")
    default_meta = ScoreMetadata(
        composer="Pierre Boulez",
        trigger_word=data_cfg.get("trigger_word", "in the style of Boulez"),
    )
    dataset = ScoreDataset(
        image_dir=data_cfg["train_dir"],
        metadata_file=data_cfg.get("metadata_file"),
        resolution=resolution,
        split="train",
        default_metadata=default_meta,
    )
    logger.info(f"Dataset: {len(dataset)} images")

    dataloader = DataLoader(dataset, batch_size=train_cfg["batch_size"], shuffle=True, num_workers=0)

    unet, optimizer, dataloader, lr_scheduler = accelerator.prepare(unet, optimizer, dataloader, lr_scheduler)

    # Fast-forward scheduler and dataloader if resuming
    if resume_step > 0:
        for _ in range(resume_step):
            lr_scheduler.step()
        # Fast-forward dataloader to get a different shuffle state
        data_iter = iter(dataloader)
        for _ in range(resume_step % len(dataloader)):
            try:
                next(data_iter)
            except StopIteration:
                data_iter = iter(dataloader)
                next(data_iter)

    mp = train_cfg["mixed_precision"]
    weight_dtype = torch.bfloat16 if mp == "bf16" else (torch.float16 if mp == "fp16" else torch.float32)
    vae.to(accelerator.device, dtype=weight_dtype)
    text_encoders[0].to(accelerator.device, dtype=weight_dtype)
    text_encoders[1].to(accelerator.device, dtype=weight_dtype)

    add_time_ids = compute_sdxl_add_time_ids(resolution, accelerator.device, weight_dtype)

    logger.info("Starting training...")
    global_step = resume_step
    output_dir = Path(output_cfg["dir"]) / output_cfg["name"]
    data_iter = iter(dataloader)

    progress_bar = tqdm(range(train_cfg["max_steps"]), disable=not accelerator.is_main_process, desc="Training",
                        initial=resume_step)

    while global_step < train_cfg["max_steps"]:
        try:
            batch = next(data_iter)
        except StopIteration:
            data_iter = iter(dataloader)
            batch = next(data_iter)

        with accelerator.accumulate(unet):
            pixel_values = batch["pixel_values"].to(accelerator.device, dtype=weight_dtype)
            with torch.no_grad():
                latents = vae.encode(pixel_values).latent_dist.sample() * vae.config.scaling_factor

            noise = torch.randn_like(latents)
            timesteps = torch.randint(
                0, noise_scheduler.config.num_train_timesteps,
                (latents.shape[0],), device=accelerator.device,
            ).long()
            noisy_latents = noise_scheduler.add_noise(latents, noise, timesteps)

            caption = batch["caption"][0] if isinstance(batch["caption"], list) else batch["caption"]
            prompt_embeds, pooled = encode_prompt_sdxl(tokenizers, text_encoders, caption, accelerator.device)

            added_cond_kwargs = {"text_embeds": pooled, "time_ids": add_time_ids}
            model_pred = unet(noisy_latents, timesteps, prompt_embeds,
                              added_cond_kwargs=added_cond_kwargs).sample

            loss = F.mse_loss(model_pred.float(), noise.float(), reduction="mean")

            accelerator.backward(loss)
            if accelerator.sync_gradients:
                accelerator.clip_grad_norm_(lora_params, train_cfg.get("max_grad_norm", 1.0))
            optimizer.step()
            lr_scheduler.step()
            optimizer.zero_grad()

        if accelerator.sync_gradients:
            global_step += 1
            progress_bar.update(1)
            progress_bar.set_postfix(loss=f"{loss.item():.4f}", lr=f"{lr_scheduler.get_last_lr()[0]:.2e}")

            if global_step % output_cfg["save_every_n_steps"] == 0:
                ckpt_path = output_dir / f"checkpoint-{global_step}.safetensors"
                save_lora_weights(accelerator.unwrap_model(unet), ckpt_path)

                if val_cfg.get("enabled"):
                    logger.info(f"Generating validation images at step {global_step}...")
                    generate_validation(
                        accelerator.unwrap_model(unet), vae, text_encoders, tokenizers,
                        noise_scheduler, val_cfg["prompts"], val_cfg.get("resolution", resolution),
                        val_cfg.get("steps", 30), val_cfg.get("cfg", 7.0), train_cfg.get("seed", 42),
                        output_dir / f"val-{global_step}", accelerator.device, weight_dtype,
                    )
                    # Restore VAE to training dtype (validation changed it to fp32)
                    vae.to(accelerator.device, dtype=weight_dtype)
                    torch.cuda.empty_cache()

    final_path = output_dir / f"{output_cfg['name']}.safetensors"
    save_lora_weights(accelerator.unwrap_model(unet), final_path)
    logger.info(f"\nTraining complete! Final model: {final_path}")
    logger.info(f"Load with: pipe.load_lora_weights('{final_path}')")


if __name__ == "__main__":
    main()
