import random

from scripts.common.camera import Camera
from scripts.common.materials import Dielectric, Lambertian, Metal
from scripts.common.shapes import Shape, Sphere
from scripts.common.texture import SolidColor
from scripts.common.vec import Vec3
from scripts.common.seq import seq

def generate():
    shapes: list[Shape] = [Sphere(
        center = Vec3(x=0, y=-1000, z=0),
        radius = 1000,
        material = Lambertian(texture = SolidColor(color = Vec3(
            x = 0.5,
            y = 0.5,
            z = 0.5,
        )))
    )]

    choices = [Dielectric.default, Lambertian.random, Metal.random]
    weights = [5, 80, 15]

    for a in seq(-11, 11, 1):
        for b in seq(-11, 11, 1):
            Material, = random.choices(choices, weights)

            m = Material()
            c = Vec3(
                x = a + 0.9*random.random(),
                y = 0.2,
                z = b + 0.9*random.random(),
            )

            shapes.append(Sphere(center=c, radius=0.2, material=m))

    shapes.append(Sphere(
        center = Vec3(x = 0, y = 1, z = 0),
        radius = 1.0,
        material = Dielectric.default()
    ))
    shapes.append(Sphere(
        center = Vec3(x = -4, y = 1, z = 0),
        radius = 1.0,
        material = Lambertian(texture = SolidColor(color = Vec3(x = 0.4, y = 0.2, z = 0.1)))
    ))
    shapes.append(Sphere(
        center = Vec3(x = 4, y=1, z=0),
        radius = 1.0,
        material = Metal(texture = SolidColor(Vec3(x = 0.7, y = 0.6, z = 0.5)), fuzz = 0)
    ))

    camera = Camera(
        look_from = Vec3(x = 13, y = 2, z = 3),
        look_at = Vec3(x = 0, y = 0, z = 0),
    ).serialize()

    objects = [shape.serialize() for shape in shapes]

    return {
        "camera": camera,
        "objects": objects,
    }
