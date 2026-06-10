from __future__ import annotations

import json
import sys
from collections.abc import Callable
from dataclasses import dataclass
from typing import Any

from pynative.state import State
from pynative.widgets import Widget


@dataclass
class App:
    root: Widget

    def to_dict(self) -> dict:
        return {
            "kind": "App",
            "props": {},
            "children": [self.root.to_dict()],
        }

    def to_json(self) -> str:
        return json.dumps(self.to_dict(), separators=(",", ":"))

    def summarize(self) -> dict:
        try:
            from pynative import _native
        except ImportError:
            return {
                "root": "App",
                "node_count": self._count_nodes(self.to_dict()),
                "max_depth": self._max_depth(self.to_dict()),
                "native": False,
            }

        summary = json.loads(_native.summarize_widget_tree_json(self.to_json()))
        summary["native"] = True
        return summary

    def run(self, mode: str = "desktop") -> None:
        if mode == "summary":
            summary = self.summarize()
            print(json.dumps(summary, indent=2))
            return

        if mode not in {"desktop", "windows"}:
            raise ValueError(f"Unsupported run mode: {mode}")

        try:
            from pynative import _native
        except ImportError:
            summary = self.summarize()
            print(json.dumps(summary, indent=2))
            return

        callbacks = self._callback_registry()
        state_bindings = self._state_registry()
        if callbacks or state_bindings:
            result = json.loads(
                _native.run_windows_widget_tree_json_with_events(
                    self.to_json(),
                    lambda event_json: self._dispatch_native_event(
                        json.loads(event_json),
                        callbacks,
                        state_bindings,
                    ),
                )
            )
        else:
            result = json.loads(_native.run_windows_widget_tree_json(self.to_json()))

        print(json.dumps(result, indent=2))

    def _dispatch_native_event(
        self,
        event: dict[str, Any],
        callbacks: dict[int, Callable[[], None]],
        state_bindings: dict[int, State[str]],
    ) -> str | None:
        event_kind = event.get("kind")
        callback_id = int(event.get("callback_id", 0))

        if event_kind == "button_click":
            callback = callbacks.get(callback_id)
            if callback is None:
                print(
                    f"PyNative warning: no callback registered for id {callback_id}",
                    file=sys.stderr,
                )
                return None

            callback()
            return self.to_json()

        if event_kind == "input_change":
            state = state_bindings.get(callback_id)
            if state is None:
                print(
                    f"PyNative warning: no input state registered for id {callback_id}",
                    file=sys.stderr,
                )
                return None

            state.set(str(event.get("value") or ""))
            return self.to_json()

        print(f"PyNative warning: unsupported native event {event_kind}", file=sys.stderr)
        return None

    def _callback_registry(self) -> dict[int, Callable[[], None]]:
        callbacks: dict[int, Callable[[], None]] = {}
        self._collect_events(self.root, callbacks, {})
        return callbacks

    def _state_registry(self) -> dict[int, State[str]]:
        states: dict[int, State[str]] = {}
        self._collect_events(self.root, {}, states)
        return states

    def _collect_events(
        self,
        widget: Widget,
        callbacks: dict[int, Callable[[], None]],
        state_bindings: dict[int, State[str]],
    ) -> None:
        callback = getattr(widget, "on_click", None)
        if callback is not None:
            callbacks[id(callback)] = callback

        value = getattr(widget, "value", None)
        if isinstance(value, State):
            state_bindings[id(value)] = value

        child = getattr(widget, "child", None)
        if child is not None:
            self._collect_events(child, callbacks, state_bindings)

        for child in getattr(widget, "children", []):
            self._collect_events(child, callbacks, state_bindings)

    def _print_summary(self) -> None:
        summary = self.summarize()
        print(json.dumps(summary, indent=2))

    def _count_nodes(self, node: dict) -> int:
        return 1 + sum(self._count_nodes(child) for child in node.get("children", []))

    def _max_depth(self, node: dict) -> int:
        children = node.get("children", [])
        if not children:
            return 1
        return 1 + max(self._max_depth(child) for child in children)
