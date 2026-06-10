from __future__ import annotations

from collections.abc import Callable
from typing import Generic, TypeVar

T = TypeVar("T")


class State(Generic[T]):
    def __init__(self, value: T):
        self._value = value
        self._listeners: list[Callable[[T], None]] = []

    @property
    def value(self) -> T:
        return self._value

    def set(self, value: T) -> None:
        if value == self._value:
            return

        self._value = value
        for listener in list(self._listeners):
            listener(value)

    def subscribe(self, listener: Callable[[T], None]) -> Callable[[], None]:
        self._listeners.append(listener)

        def unsubscribe() -> None:
            self._listeners.remove(listener)

        return unsubscribe
