import random

from math import cos, sin, pi as PI

from scripts.common.camera import Camera
from scripts.common.materials import Dielectric, Lambertian, Metal
from scripts.common.shapes import Shape, Sphere
from scripts.common.seq import seq
from scripts.common.texture import Checker
from scripts.common.vec import Vec3

def generate():
    shapes: list[Shape] = [
        Sphere(
            center = Vec3(x=0, y=-1000, z=0),
            radius = 1000,
            material = Lambertian(texture=Checker(
                odd=Vec3(
                    x = 0.2,
                    y = 0.3,
                    z = 0.1,
                ),
                even=Vec3(
                    x = 0.9,
                    y = 0.9,
                    z = 0.9,
                ),
                scale=128,
            )),
        )
    ]

    step = 4/1001
    r = 1001

    choices = [Dielectric.default, Lambertian.random, Metal.random]
    weights = [1, 4, 8]

    for sigma in seq(-step, step, step):
        r = 1001*cos(sigma)
        z = sin(sigma)
        for theta in map(lambda a: a + PI/2, seq(-step, step, step)):
            c = Vec3(
                x = cos(theta),
                y = sin(theta),
                z = z
            ).mul(r).add(Vec3(y=-r + 1)).round(4)

            makeMaterial, = random.choices(choices, weights)
            shapes.append(Sphere(center=c, radius=1, material = makeMaterial()))

    camera = Camera(
        look_from = Vec3(x=8.0, y=4.0, z=10.0),
        look_at = Vec3(x=0, y=0, z=0),
    ).serialize()

    objects = [shape.serialize() for shape in shapes]

    return {
        "camera": camera,
        "objects": objects,
    }
