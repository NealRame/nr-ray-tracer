from dataclasses import dataclass, field

from .serializable import Serializable
from .vec import Vec3

@dataclass
class Camera(Serializable):
    look_at: Vec3 | None = field(default=None)
    look_from: Vec3 | None = field(default=None)
    view_up: Vec3 | None = field(default=None)

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            ** ({} if self.look_at == None else {
                "look_at": self.look_at.serialize()
            }),
            ** ({} if self.look_from == None else {
                "look_from": self.look_from.serialize()
            }),
            ** ({} if self.view_up == None else {
                "view_up": self.view_up.serialize()
            }),
        }
