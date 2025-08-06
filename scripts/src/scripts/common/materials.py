import random

from dataclasses import dataclass

from .serializable import Serializable

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
    texture: int = 0

    @classmethod
    def default(cls):
        return cls()

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Lambertian": {
                "texture": self.texture,
            },
        }

@dataclass
class Metal(Material):
    texture: int = 0
    fuzz: float = 0.5

    @classmethod
    def default(cls):
        return cls()

    @classmethod
    def random(cls, texture):
        return cls(
            texture = texture,
            fuzz = random.random(),
        )

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Metal": {
                "texture": self.texture,
                "fuzz": self.fuzz,
            },
        }

@dataclass
class DiffuseLight(Material):
    texture: int = 0

    @classmethod
    def default(cls):
        return cls()

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "DiffuseLight": {
                "texture": self.texture,
            },
        }
