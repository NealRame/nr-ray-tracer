import random

from dataclasses import dataclass, field

from .serializable import Serializable
from .vec import Vec3

class Texture(Serializable): ...

@dataclass
class SolidColor(Texture):
    color: Vec3 = field(default_factory=Vec3)

    @classmethod
    def default(cls):
        return cls()

    @classmethod
    def random(cls):
        return cls(color = Vec3(
            x = random.random(),
            y = random.random(),
            z = random.random(),
        ))

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "SolidColor": self.color.serialize(),
        }


@dataclass
class Checker(Texture):
    even: Vec3 = field(default_factory=Vec3)
    odd: Vec3 = field(default_factory=Vec3)
    scale: int = 1;

    @classmethod
    def default(cls):
        return cls(
            even = Vec3(
                x = 1.0,
                y = 1.0,
                z = 1.0,
            ),
            odd = Vec3(
                x = 0.0,
                y = 0.0,
                z = 0.0,
            )
        )

    @classmethod
    def random(cls, *args, **kwargs):
        return cls(
            even = kwargs.get("even", Vec3(
                x = random.random(),
                y = random.random(),
                z = random.random(),
            )),
            odd = kwargs.get("odd", Vec3(
                x = random.random(),
                y = random.random(),
                z = random.random(),
            )),
            scale = kwargs.get("scale", random.randint(2, 32)),
        )

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Checker": {
                "even": self.even.serialize(),
                "odd": self.odd.serialize(),
                "scale": self.scale,
            },
        }
