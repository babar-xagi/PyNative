from __future__ import annotations

from collections.abc import Callable, Sequence
from dataclasses import dataclass, field
from typing import Any

from pynative.design import StyleInput, props_with_style
from pynative.state import State


@dataclass
class Widget:
    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": self.__class__.__name__,
            "props": {},
            "children": [],
        }


@dataclass
class Window(Widget):
    child: Widget
    title: str = "PyNative UI"
    style: StyleInput = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Window",
            "props": props_with_style({"title": self.title}, self.style),
            "children": [self.child.to_dict()],
        }


@dataclass
class Column(Widget):
    children: Sequence[Widget] = field(default_factory=list)
    style: StyleInput = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Column",
            "props": props_with_style({}, self.style),
            "children": [child.to_dict() for child in self.children],
        }


@dataclass
class Row(Widget):
    children: Sequence[Widget] = field(default_factory=list)
    style: StyleInput = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Row",
            "props": props_with_style({}, self.style),
            "children": [child.to_dict() for child in self.children],
        }


@dataclass
class Text(Widget):
    value: str | Callable[[], str]
    style: StyleInput = None

    def to_dict(self) -> dict[str, Any]:
        value = self.value() if callable(self.value) else self.value
        return {
            "kind": "Text",
            "props": props_with_style({"value": value}, self.style),
            "children": [],
        }


@dataclass
class Button(Widget):
    label: str
    on_click: Callable[[], None] | None = None
    style: StyleInput = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Button",
            "props": props_with_style(
                {
                    "label": self.label,
                    "callback_id": id(self.on_click) if self.on_click else None,
                },
                self.style,
            ),
            "children": [],
        }


@dataclass
class Input(Widget):
    value: State[str]
    placeholder: str = ""
    style: StyleInput = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Input",
            "props": props_with_style(
                {
                    "value": self.value.value,
                    "placeholder": self.placeholder,
                    "state_id": id(self.value),
                },
                self.style,
            ),
            "children": [],
        }


@dataclass
class Image(Widget):
    src: str
    alt: str = ""
    style: StyleInput = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Image",
            "props": props_with_style(
                {
                    "src": self.src,
                    "alt": self.alt,
                },
                self.style,
            ),
            "children": [],
        }
