import random

from scripts.common.camera import Camera
from scripts.common.materials import Dielectric, Lambertian, Metal
from scripts.common.shapes import Shape, Sphere
from scripts.common.vec import Vec3
from scripts.common.seq import seq

def generate():
    shapes: list[Shape] = [Sphere(
        center = Vec3(x=0, y=-1000, z=0),
        radius = 1000,
        material = Lambertian.default()
    )]

    choices = [Dielectric.default, Lambertian.random, Metal.random]
    weights = [5, 80, 15]

    for a in seq(-11, 11, 1):
        for b in seq(-11, 11, 1):
            c = Vec3(
                x = a + 0.9*random.random(),
                y = 0.2,
                z = b + 0.9*random.random(),
            )

            if c.sub(Vec3(x = 4, y = 0.2, z = 0)).length() > 0.9:
                Material, = random.choices(choices, weights)

                if random.random() < 8/10:
                    v = Vec3(x = 0, y = random.random()*0.5, z = 0.0)
                else:
                    v = None

                shapes.append(Sphere(center=c, speed=v, radius=0.2, material=Material()))

    shapes.append(Sphere(
        center = Vec3(x = 0, y = 1, z = 0),
        radius = 1.0,
        material = Dielectric.default()
    ))
    shapes.append(Sphere(
        center = Vec3(x = -4, y = 1, z = 0),
        radius = 1.0,
        material = Lambertian(albedo = Vec3(x = 0.4, y = 0.2, z = 0.1))
    ))
    shapes.append(Sphere(
        center = Vec3(x = 4, y=1, z=0),
        radius = 1.0,
        material = Metal(albedo = Vec3(x = 0.7, y = 0.6, z = 0.5), fuzz = 0)
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
