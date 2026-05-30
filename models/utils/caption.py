"""Caption template generation for LoRA training.

Captions condition the diffusion model during training. The trigger word
activates the LoRA style; the rest describes visual characteristics.
"""

from dataclasses import dataclass, field


@dataclass
class ScoreMetadata:
    """Metadata for a single score image page."""

    composer: str
    work_title: str = ""
    period: str = ""
    techniques: list[str] = field(default_factory=list)
    instrumentation: str = ""
    notation_type: str = "traditional"
    page_type: str = "full_page"
    source_type: str = "scan"
    trigger_word: str = ""


PERIOD_LABELS = {
    "early_modern": "early 20th century",
    "post_war": "post-war avant-garde",
    "late_20th": "late 20th century",
    "contemporary": "contemporary",
}

TECHNIQUE_LABELS = {
    "serialism": "serial composition",
    "integral_serialism": "integral serialism",
    "spectralism": "spectral music",
    "stochastic": "stochastic composition",
    "algorithmic": "algorithmic composition",
    "graphic_notation": "graphic notation",
    "extended_techniques": "extended techniques",
    "microtonality": "microtonal notation",
    "new_complexity": "new complexity style",
    "minimalism": "minimalist composition",
    "proportional_notation": "proportional notation",
    "aleatoric": "aleatoric composition",
}

NOTATION_LABELS = {
    "traditional": "traditional staff notation",
    "proportional": "proportional notation",
    "graphic": "graphic notation",
    "mixed": "mixed notation",
    "text_score": "text score",
}


def generate_caption(meta: ScoreMetadata) -> str:
    """Generate a training caption from score metadata.

    The caption is a comma-separated description that conditions the
    diffusion model. It always starts with a base descriptor and ends
    with quality tags.
    """
    parts = ["contemporary classical music score"]

    if meta.trigger_word:
        parts.append(meta.trigger_word)
    elif meta.composer:
        parts.append(f"in the style of {meta.composer}")

    if meta.period in PERIOD_LABELS:
        parts.append(PERIOD_LABELS[meta.period])

    tech_labels = [TECHNIQUE_LABELS.get(t, t) for t in meta.techniques[:3]]
    if tech_labels:
        parts.append(", ".join(tech_labels))

    if meta.instrumentation:
        parts.append(f"for {meta.instrumentation}")

    if meta.notation_type in NOTATION_LABELS:
        parts.append(NOTATION_LABELS[meta.notation_type])

    parts.append("high quality music engraving")
    parts.append("black ink on white paper")

    return ", ".join(parts)


STYLE_TEMPLATES: dict[str, ScoreMetadata] = {
    "boulez": ScoreMetadata(
        composer="Pierre Boulez",
        period="post_war",
        techniques=["serialism", "integral_serialism"],
        instrumentation="piano",
        notation_type="traditional",
        trigger_word="in the style of Boulez",
    ),
    "xenakis": ScoreMetadata(
        composer="Iannis Xenakis",
        period="post_war",
        techniques=["stochastic", "algorithmic"],
        instrumentation="orchestral",
        notation_type="mixed",
        trigger_word="in the style of Xenakis",
    ),
    "graphic_notation": ScoreMetadata(
        composer="",
        period="post_war",
        techniques=["graphic_notation"],
        notation_type="graphic",
        trigger_word="graphic notation score",
    ),
    "ferneyhough": ScoreMetadata(
        composer="Brian Ferneyhough",
        period="late_20th",
        techniques=["new_complexity", "extended_techniques"],
        instrumentation="solo",
        notation_type="traditional",
        trigger_word="in the style of Ferneyhough",
    ),
}
