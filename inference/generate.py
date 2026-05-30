"""CLI tool for SDXL score image generation — supports three modes.

Modes:
  txt2img   — Pure diffusion from text prompt + LoRA (default)
  img2img   — Style transfer from a reference score image + LoRA
  controlnet — Guided generation using a structural layout + LoRA

All modes are image-level operations. No symbolic music tools involved.
"""

import sys
from pathlib import Path

import click
import torch
from PIL import Image

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from utils.device import get_device, get_dtype, get_vram_gb, print_device_info


def load_pipeline(mode: str, device, dtype, vram_gb: float, lora_path=None, lora_weight=0.8):
    """Load the appropriate pipeline for the given mode."""
    if mode == "controlnet":
        from diffusers import StableDiffusionXLControlNetPipeline, ControlNetModel, AutoencoderKL

        controlnet = ControlNetModel.from_pretrained(
            "diffusers/controlnet-canny-sdxl-1.0", torch_dtype=dtype
        )
        vae = AutoencoderKL.from_pretrained("madebyollin/sdxl-vae-fp16-fix", torch_dtype=dtype)
        pipe = StableDiffusionXLControlNetPipeline.from_pretrained(
            "stabilityai/stable-diffusion-xl-base-1.0",
            controlnet=controlnet, vae=vae, torch_dtype=dtype,
        )
    elif mode == "img2img":
        from diffusers import StableDiffusionXLImg2ImgPipeline

        pipe = StableDiffusionXLImg2ImgPipeline.from_pretrained(
            "stabilityai/stable-diffusion-xl-base-1.0",
            torch_dtype=dtype, variant="fp16",
        )
    else:  # txt2img
        from diffusers import StableDiffusionXLPipeline

        pipe = StableDiffusionXLPipeline.from_pretrained(
            "stabilityai/stable-diffusion-xl-base-1.0",
            torch_dtype=dtype, variant="fp16",
        )

    # Memory optimizations
    if vram_gb < 12:
        print(f"  VRAM {vram_gb:.0f}GB < 12GB -> CPU offload + VAE tiling")
        pipe.enable_model_cpu_offload()
        pipe.enable_vae_tiling()
    else:
        pipe.to(device)

    # Load LoRA
    if lora_path and Path(lora_path).exists():
        print(f"  LoRA: {Path(lora_path).name} (weight: {lora_weight})")
        pipe.load_lora_weights(lora_path, adapter_name="style")
        pipe.set_adapters(["style"], adapter_weights=[lora_weight])

    return pipe


@click.command()
@click.option("--mode", type=click.Choice(["txt2img", "img2img", "controlnet"]), default="txt2img",
              help="Generation mode")
@click.option("--prompt", default="contemporary classical music score, in the style of Boulez, for piano, serial composition, black ink on white paper",
              help="Text prompt")
@click.option("--negative-prompt", default="blurry, low quality, colored, photograph, watermark",
              help="Negative prompt")
@click.option("--lora", default=None, help="Path to LoRA .safetensors")
@click.option("--lora-weight", default=0.8, help="LoRA weight (0.0-1.0)")
@click.option("--ref-image", default=None, help="Reference image for img2img mode")
@click.option("--strength", default=0.55, help="img2img denoising strength (0.0=keep original, 1.0=pure noise)")
@click.option("--layout-image", default=None, help="Layout/edge image for ControlNet mode")
@click.option("--controlnet-scale", default=0.8, help="ControlNet conditioning scale")
@click.option("--steps", default=30, help="Inference steps")
@click.option("--cfg", default=7.0, help="CFG guidance scale")
@click.option("--seed", default=42, help="Random seed (-1 for random)")
@click.option("--width", default=1024, help="Image width (multiple of 8)")
@click.option("--height", default=1024, help="Image height (multiple of 8)")
@click.option("--output", default="outputs/generated.png", help="Output path")
@click.option("--batch", default=1, help="Number of images")
def main(mode, prompt, negative_prompt, lora, lora_weight, ref_image, strength,
         layout_image, controlnet_scale, steps, cfg, seed, width, height, output, batch):
    """Generate music score images using SDXL with three modes.

    \b
    Examples:
      # Pure diffusion (graphic notation)
      coda-generate --mode txt2img --lora weights/lora/boulez.safetensors

      # img2img from IMSLP reference (traditional notation)
      coda-generate --mode img2img --ref-image references/scriabin_page.png --strength 0.55

      # ControlNet with procedural layout (mixed notation)
      coda-generate --mode controlnet --layout-image layouts/staff_layout.png
    """
    device = get_device()
    dtype = get_dtype(device)
    vram = get_vram_gb()
    print_device_info()

    print(f"\nMode: {mode}")
    pipe = load_pipeline(mode, device, dtype, vram, lora, lora_weight)

    for i in range(batch):
        current_seed = seed + i if seed >= 0 else torch.randint(0, 2**32, (1,)).item()
        generator = torch.Generator(device=device).manual_seed(current_seed)

        print(f"\nGenerating {i + 1}/{batch} - seed {current_seed}")
        print(f"  Prompt: {prompt[:80]}...")

        if mode == "img2img":
            if not ref_image:
                print("ERROR: --ref-image required for img2img mode")
                return
            init_image = Image.open(ref_image).convert("RGB").resize((width, height))
            print(f"  Reference: {ref_image} (strength: {strength})")
            image = pipe(
                prompt=prompt, negative_prompt=negative_prompt, image=init_image,
                strength=strength, num_inference_steps=steps, guidance_scale=cfg,
                generator=generator,
            ).images[0]

        elif mode == "controlnet":
            if not layout_image:
                print("ERROR: --layout-image required for controlnet mode")
                return
            import cv2
            import numpy as np
            layout = cv2.imread(layout_image, cv2.IMREAD_GRAYSCALE)
            layout = cv2.resize(layout, (width, height))
            edges = cv2.Canny(layout, 50, 150)
            edge_image = Image.fromarray(edges)
            print(f"  Layout: {layout_image} (scale: {controlnet_scale})")
            image = pipe(
                prompt=prompt, negative_prompt=negative_prompt, image=edge_image,
                controlnet_conditioning_scale=controlnet_scale,
                num_inference_steps=steps, guidance_scale=cfg,
                width=width, height=height, generator=generator,
            ).images[0]

        else:  # txt2img
            image = pipe(
                prompt=prompt, negative_prompt=negative_prompt,
                num_inference_steps=steps, guidance_scale=cfg,
                width=width, height=height, generator=generator,
            ).images[0]

        output_path = Path(output)
        if batch > 1:
            output_path = output_path.parent / f"{output_path.stem}_{i:03d}{output_path.suffix}"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        image.save(output_path)
        print(f"  Saved: {output_path}")

    print("\nDone.")


if __name__ == "__main__":
    main()
