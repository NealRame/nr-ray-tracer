from dataclasses import dataclass, field

from .serializable import Serializable
from .vec import Vec3

class Shape(Serializable): ...

@dataclass
class Sphere(Shape):
    center: Vec3 = field(default_factory=Vec3.zero)
    radius: float = 1
    material: int = 0
    speed: Vec3|None = None

    @classmethod
    def default(cls):
        return cls()

    def serialize(self) -> dict | float | int | list | str | tuple:
        if self.speed is None:
            return {
                "Sphere": {
                    "center": self.center.serialize(),
                    "radius": self.radius,
                    "material": self.material,
                },
            }
        else:
            return {
                "Sphere": {
                    "center": self.center.serialize(),
                    "speed": self.speed.serialize(),
                    "radius": self.radius,
                    "material": self.material,
                },
            }
