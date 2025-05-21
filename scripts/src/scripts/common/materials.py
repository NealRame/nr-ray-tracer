import random

from dataclasses import dataclass, field

from .serializable import Serializable
from .vec import Vec3

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
    albedo: Vec3 = field(default_factory=Vec3)

    @classmethod
    def default(cls):
        return cls(albedo = Vec3(
            x = 0.5,
            y = 0.5,
            z = 0.5,
        ))

    @classmethod
    def random(cls):
        return cls(albedo = Vec3(
            x = random.random(),
            y = random.random(),
            z = random.random(),
        ))

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Lambertian": {
                "albedo": self.albedo.serialize(),
            },
        }

@dataclass
class Metal(Material):
    albedo: Vec3 = field(default_factory=Vec3)
    fuzz: float = 0

    @classmethod
    def default(cls):
        return cls(
            albedo = Vec3(
                x = 0.5,
                y = 0.5,
                z = 0.5,
            ),
            fuzz =  0.5,
        )

    @classmethod
    def random(cls):
        return cls(
            albedo = Vec3(
                x = random.random(),
                y = random.random(),
                z = random.random(),
            ),
            fuzz = random.random(),
        )

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Metal": {
                "albedo": self.albedo.serialize(),
                "fuzz": self.fuzz,
            },
        }
