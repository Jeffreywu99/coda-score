"""Phase 0 validation: side-by-side comparison of base SDXL vs LoRA.

Generates the same prompts with and without LoRA, at the same seed,
so differences can be attributed to the LoRA adapter.
"""

import sys
from pathlib import Path

import click
import torch

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from utils.device import get_device, get_dtype, get_vram_gb

VALIDATION_PROMPTS = [
    # Prompt 1: With trigger word — should show LoRA style
    "contemporary classical music score, in the style of Boulez, for piano, serial composition, black ink on white paper, high quality engraving",
    # Prompt 2: With trigger word — different aspect
    "modern avant-garde piano score, in the style of Boulez, dense atonal notation, complex rhythms, black ink on white paper",
    # Prompt 3: WITHOUT trigger word — should NOT show LoRA style (control)
    "contemporary classical music score, black ink on white paper, traditional music engraving",
    # Prompt 4: Completely different — tests LoRA specificity
    "a photograph of a cat sitting on a piano",
]


@click.command()
@click.option("--lora", required=True, type=click.Path(exists=True), help="LoRA .safetensors path")
@click.option("--output-dir", default="outputs/validation", help="Output directory")
@click.option("--resolution", default=512, help="Image resolution")
@click.option("--seed", default=42, help="Random seed")
def main(lora, output_dir, resolution, seed):
    """Generate comparison images: base SDXL vs SDXL + LoRA."""
    from diffusers import StableDiffusionXLPipeline

    device = get_device()
    dtype = get_dtype(device)
    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    # --- Base SDXL ---
    print("=== Base SDXL (no LoRA) ===")
    pipe = StableDiffusionXLPipeline.from_pretrained(
        "stabilityai/stable-diffusion-xl-base-1.0",
        torch_dtype=dtype, variant="fp16",
    )
    if get_vram_gb() < 12:
        pipe.enable_model_cpu_offload()
        pipe.enable_vae_tiling()
    else:
        pipe.to(device)

    gen = torch.Generator(device=device).manual_seed(seed)
    for i, prompt in enumerate(VALIDATION_PROMPTS):
        img = pipe(prompt=prompt, num_inference_steps=30, guidance_scale=7.0,
                   width=resolution, height=resolution, generator=gen).images[0]
        img.save(output_dir / f"base_{i:02d}.png")
        print(f"  base_{i:02d}.png - {prompt[:60]}...")

    del pipe
    torch.cuda.empty_cache()

    # --- SDXL + LoRA ---
    print(f"\n=== SDXL + LoRA ({Path(lora).name}) ===")
    pipe = StableDiffusionXLPipeline.from_pretrained(
        "stabilityai/stable-diffusion-xl-base-1.0",
        torch_dtype=dtype, variant="fp16",
    )
    pipe.load_lora_weights(lora)
    if get_vram_gb() < 12:
        pipe.enable_model_cpu_offload()
        pipe.enable_vae_tiling()
    else:
        pipe.to(device)

    gen = torch.Generator(device=device).manual_seed(seed)
    for i, prompt in enumerate(VALIDATION_PROMPTS):
        img = pipe(prompt=prompt, num_inference_steps=30, guidance_scale=7.0,
                   width=resolution, height=resolution, generator=gen).images[0]
        img.save(output_dir / f"lora_{i:02d}.png")
        print(f"  lora_{i:02d}.png - {prompt[:60]}...")

    print(f"\nAll images saved to {output_dir}/")
    print("Compare base_00/01 vs lora_00/01 (with trigger) and base_02 vs lora_02 (control)")


if __name__ == "__main__":
    main()
