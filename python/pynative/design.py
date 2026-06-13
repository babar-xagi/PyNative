from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass
from typing import Any, TypeAlias


StyleInput: TypeAlias = "Style | Mapping[str, Any] | None"


@dataclass(frozen=True)
class Style:
    color: str | None = None
    background_color: str | None = None
    font_size: int | None = None
    font_weight: str | None = None
    width: int | None = None
    height: int | None = None
    padding: int | None = None
    margin: int | None = None
    gap: int | None = None
    align: str | None = None

    def to_dict(self) -> dict[str, Any]:
        values = {
            "color": self.color,
            "background_color": self.background_color,
            "font_size": self.font_size,
            "font_weight": self.font_weight,
            "width": self.width,
            "height": self.height,
            "padding": self.padding,
            "margin": self.margin,
            "gap": self.gap,
            "align": self.align,
        }
        return {key: value for key, value in values.items() if value is not None}


def normalize_style(style: StyleInput) -> dict[str, Any]:
    if style is None:
        return {}

    if isinstance(style, Style):
        return style.to_dict()

    return {
        str(key): value
        for key, value in style.items()
        if value is not None
    }


def props_with_style(props: dict[str, Any], style: StyleInput) -> dict[str, Any]:
    normalized = normalize_style(style)
    if normalized:
        return {**props, "style": normalized}
    return props
