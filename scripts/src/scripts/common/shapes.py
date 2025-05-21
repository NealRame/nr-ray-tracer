from dataclasses import dataclass, field

from .materials import Material, Lambertian
from .serializable import Serializable
from .vec import Vec3

class Shape(Serializable): ...

@dataclass
class Sphere(Shape):
    center: Vec3 = field(default_factory=Vec3.zero)
    radius: float = 1
    material: Material = field(default_factory=Lambertian.default)

    @classmethod
    def default(cls):
        return cls()

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Sphere": {
                "center": self.center.serialize(),
                "radius": self.radius,
                "material": self.material.serialize()
            },
        }
