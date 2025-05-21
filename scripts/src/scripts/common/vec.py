from dataclasses import dataclass, astuple, KW_ONLY

from .serializable import Serializable

@dataclass
class Vec3(Serializable):
    _: KW_ONLY
    x: float = 0
    y: float = 0
    z: float = 0

    @classmethod
    def one(cls):
        return cls(
            x = 1.0,
            y = 1.0,
            z = 1.0,
        )

    @classmethod
    def zero(cls):
        return cls(
            x = 0.0,
            y = 0.0,
            z = 0.0,
        )

    def mul(self, k: float):
        return Vec3(
            x = k*self.x,
            y = k*self.y,
            z = k*self.z,
        )

    def add(self, v):
        return Vec3(
            x = self.x + v.x,
            y = self.y + v.y,
            z = self.z + v.z,
        )

    def round(self, r = 0):
        return Vec3(
            x = round(self.x, r),
            y = round(self.y, r),
            z = round(self.z, r),
        )

    def serialize(self) -> dict | float | int | list | str | tuple:
        return astuple(self)
