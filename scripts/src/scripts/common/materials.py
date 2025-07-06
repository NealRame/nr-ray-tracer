import random

from dataclasses import dataclass, field

from .serializable import Serializable
from .texture import Texture, SolidColor

class Material(Serializable): ...

@dataclass
class Dielectric(Material):
    refraction_index: float = 1.5

    @classmethod
    def default(cls):
        return cls()

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Dielectric": {
                "refraction_index": self.refraction_index,
            },
        }

@dataclass
class Lambertian(Material):
    texture: Texture = field(default_factory=SolidColor.default)

    @classmethod
    def default(cls):
        return cls()

    @classmethod
    def random(cls):
        return cls(texture = SolidColor.random())

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Lambertian": {
                "texture": self.texture.serialize(),
            },
        }

@dataclass
class Metal(Material):
    texture: Texture = field(default_factory=SolidColor.default)
    fuzz: float = 0.5

    @classmethod
    def default(cls):
        return cls()

    @classmethod
    def random(cls):
        return cls(
            texture = SolidColor.random(),
            fuzz = random.random(),
        )

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Metal": {
                "texture": self.texture.serialize(),
                "fuzz": self.fuzz,
            },
        }
