from __future__ import annotations

from collections.abc import Callable, Sequence
from dataclasses import dataclass, field
from typing import Any

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

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Window",
            "props": {"title": self.title},
            "children": [self.child.to_dict()],
        }


@dataclass
class Column(Widget):
    children: Sequence[Widget] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Column",
            "props": {},
            "children": [child.to_dict() for child in self.children],
        }


@dataclass
class Row(Widget):
    children: Sequence[Widget] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Row",
            "props": {},
            "children": [child.to_dict() for child in self.children],
        }


@dataclass
class Text(Widget):
    value: str | Callable[[], str]

    def to_dict(self) -> dict[str, Any]:
        value = self.value() if callable(self.value) else self.value
        return {
            "kind": "Text",
            "props": {"value": value},
            "children": [],
        }


@dataclass
class Button(Widget):
    label: str
    on_click: Callable[[], None] | None = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Button",
            "props": {
                "label": self.label,
                "callback_id": id(self.on_click) if self.on_click else None,
            },
            "children": [],
        }


@dataclass
class Input(Widget):
    value: State[str]
    placeholder: str = ""

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Input",
            "props": {
                "value": self.value.value,
                "placeholder": self.placeholder,
                "state_id": id(self.value),
            },
            "children": [],
        }


@dataclass
class Image(Widget):
    src: str
    alt: str = ""

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": "Image",
            "props": {
                "src": self.src,
                "alt": self.alt,
            },
            "children": [],
        }
