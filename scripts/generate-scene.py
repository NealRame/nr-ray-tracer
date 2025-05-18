#! /usr/bin/env python3

import sys
import random

from dataclasses import dataclass, astuple, field, KW_ONLY
from json import dump as json_dump
from math import pi as PI, cos, sin

assert sys.version_info >= (3, 10)

class Serializable:
    def serialize(self) -> dict | float | int | list | str | tuple:
        ...

@dataclass
class Vec3(Serializable):
    _: KW_ONLY
    x: float = 0
    y: float = 0
    z: float = 0

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

class Material(Serializable): ...

@dataclass
class Dielectric(Material):
    refraction_index: float = 1.5

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Dielectric": {
                "refraction_index": self.refraction_index,
            },
        }

@dataclass
class Lambertian(Material):
    albedo: Vec3 = field(default_factory=Vec3)

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

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Metal": {
                "albedo": self.albedo.serialize(),
                "fuzz": self.fuzz,
            },
        }

class Shape(Serializable): ...

@dataclass
class Sphere(Shape):
    center: Vec3 = field(default_factory=Vec3)
    radius: float = 1
    material: Material = field(default_factory=Lambertian)

    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            "Sphere": {
                "center": self.center.serialize(),
                "radius": self.radius,
                "material": self.material.serialize()
            },
        }


def seq(start: float, stop: float, step: float):
    v = start
    while v <= stop:
        yield v
        v += step

if __name__ == "__main__":
    shapes: list[Shape] = [Sphere(
        center = Vec3(x=0, y=-1000, z=0),
        radius = 1000,
        material = Lambertian(albedo=Vec3(
            x = 0.25,
            y = 0.25,
            z = 0.25,
        )))]

    step = 4/1001
    r = 1001

    for sigma in seq(-step, step, step):
        r = 1001*cos(sigma)
        z = sin(sigma)
        for theta in map(lambda a: a + PI/2, seq(-step, step, step)):
            c = Vec3(
                x = cos(theta),
                y = sin(theta),
                z = z
            ).mul(r).add(Vec3(y=-r + 1)).round(4)

            [s, ] = random.choices(
                ["dielectric", "lambertian", "metal"],
                weights=[1, 4, 8],
                k=1
            )

            if s == "lambertian":
                m = Lambertian(albedo=Vec3(
                    x = random.random(),
                    y = random.random(),
                    z = random.random(),
                ))
            elif s == "metal":
                m = Metal(albedo=Vec3(
                    x = random.random(),
                    y = random.random(),
                    z = random.random(),
                ), fuzz = random.random())
            else:
                m = Dielectric()

            shapes.append(Sphere(center=c, radius=1, material=m))

    camera = Camera(
        look_from = Vec3(x=8.0, y=4.0, z=10.0),
        look_at = Vec3(x=0, y=0, z=0),
    ).serialize()
    objects = [shape.serialize() for shape in shapes]

    json_dump({
        "camera": camera,
        "objects": objects,
    }, sys.stdout, indent=4)
