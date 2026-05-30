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
    text_encoder_one = CLIPTextModel.from_pretrained(base_model, subfolder="text_encoder", torch_dtype=dtype)
    text_encoder_two = CLIPTextModelWithProjection.from_pretrained(base_model, subfolder="text_encoder_2", torch_dtype=dtype)
    vae = AutoencoderKL.from_pretrained(base_model, subfolder="vae", torch_dtype=dtype)
    unet = UNet2DConditionModel.from_pretrained(base_model, subfolder="unet", torch_dtype=dtype)

    for model in [vae, text_encoder_one, text_encoder_two, unet]:
        model.requires_grad_(False)

    return noise_scheduler, (tokenizer_one, tokenizer_two), (text_encoder_one, text_encoder_two), vae, unet


def encode_prompt_sdxl(tokenizers, text_encoders, prompt: str, device: torch.device):
    """Encode a prompt through both SDXL text encoders and concatenate."""
    embeds = []
    pooled = None
    for tokenizer, encoder in zip(tokenizers, text_encoders):
        inputs = tokenizer(prompt, padding="max_length", max_length=tokenizer.model_max_length,
                           truncation=True, return_tensors="pt")
        input_ids = inputs.input_ids.to(device)
        with torch.no_grad():
            outputs = encoder(input_ids, output_hidden_states=True)
        embeds.append(outputs.hidden_states[-2])
        if pooled is None:
            pooled = outputs[0]
    return torch.cat(embeds, dim=-1), pooled


def compute_sdxl_add_time_ids(resolution: int, device: torch.device, dtype: torch.dtype):
    """Create the add_time_ids tensor for SDXL conditioning."""
    original_size = (resolution, resolution)
    crops_coords_top_left = (0, 0)
    target_size = (resolution, resolution)
    return torch.tensor([original_size + crops_coords_top_left + target_size],
                        device=device, dtype=dtype)


def save_lora_weights(unet, path: Path):
    """Save LoRA weights as safetensors."""
    path.parent.mkdir(parents=True, exist_ok=True)
    state_dict = get_peft_model_state_dict(unet)
    save_file(state_dict, str(path))
    logger.info(f"Saved LoRA weights -> {path}")


def generate_validation(unet, vae, text_encoders, tokenizers, noise_scheduler,
                        prompts, resolution, steps, cfg, seed, output_dir, device, dtype):
    """Generate validation images at a training checkpoint."""
    from diffusers import StableDiffusionXLPipeline

    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    pipe = StableDiffusionXLPipeline.from_pretrained(
        "stabilityai/stable-diffusion-xl-base-1.0",
        unet=unet,
        vae=vae,
        text_encoder=text_encoders[0],
        text_encoder_2=text_encoders[1],
        tokenizer=tokenizers[0],
        tokenizer_2=tokenizers[1],
        torch_dtype=dtype,
    )
    pipe.enable_model_cpu_offload()
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
def main(config: str):
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

    logger.info(f"=== coda-score LoRA Training ===")
    logger.info(f"Base model: {model_cfg['base']}")
    logger.info(f"LoRA rank: {lora_cfg['rank']}, alpha: {lora_cfg['alpha']}")
    logger.info(f"Resolution: {resolution}x{resolution}")
    logger.info(f"Max steps: {train_cfg['max_steps']}")
    logger.info(f"Learning rate: {train_cfg['learning_rate']}")
    logger.info(f"Optimizer: {train_cfg['optimizer']}")

    if train_cfg.get("seed"):
        set_seed(train_cfg["seed"])

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

    weight_dtype = torch.float16 if train_cfg["mixed_precision"] == "fp16" else torch.float32
    vae.to(accelerator.device, dtype=weight_dtype)
    text_encoders[0].to(accelerator.device, dtype=weight_dtype)
    text_encoders[1].to(accelerator.device, dtype=weight_dtype)

    add_time_ids = compute_sdxl_add_time_ids(resolution, accelerator.device, weight_dtype)

    logger.info("Starting training...")
    global_step = 0
    output_dir = Path(output_cfg["dir"]) / output_cfg["name"]
    data_iter = iter(dataloader)

    progress_bar = tqdm(range(train_cfg["max_steps"]), disable=not accelerator.is_main_process, desc="Training")

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

    final_path = output_dir / f"{output_cfg['name']}.safetensors"
    save_lora_weights(accelerator.unwrap_model(unet), final_path)
    logger.info(f"\nTraining complete! Final model: {final_path}")
    logger.info(f"Load with: pipe.load_lora_weights('{final_path}')")


if __name__ == "__main__":
    main()
