import random

from math import cos, sin, pi as PI

from scripts.common.camera import Camera
from scripts.common.materials import Dielectric, Lambertian, Metal, Material
from scripts.common.shapes import Shape, Sphere
from scripts.common.seq import seq
from scripts.common.texture import Checker, SolidColor, Texture
from scripts.common.vec import Vec3

def generate():
    textures: list[Texture] = [
        Checker(
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
        )
    ]

    materials: list[Material] = [Lambertian(texture = 0)]

    shapes: list[Shape] = [
        Sphere(
            center = Vec3(x=0, y=-1000, z=0),
            radius = 1000,
            material = 0,
        )
    ]

    step = 4/1001
    r = 1001

    def generate_dielectric():
        i = len(materials)
        materials.append(Dielectric.default())
        return i

    def generate_lambertian():
        i = len(materials)
        j = len(textures)

        textures.append(SolidColor.random())
        materials.append(Lambertian(texture = j))
        return i

    def generate_metal():
        i = len(materials)
        j = len(textures)

        textures.append(SolidColor.random())
        materials.append(Metal(texture = j, fuzz = random.random()))

        return i

    choices = [generate_dielectric, generate_lambertian, generate_metal]
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

            generate_material, = random.choices(choices, weights)
            shapes.append(Sphere(center=c, radius=1, material = generate_material()))

    camera = Camera(
        look_from = Vec3(x=8.0, y=4.0, z=10.0),
        look_at = Vec3(x=0, y=0, z=0),
    ).serialize()

    return {
        "camera": camera,
        "objects": [shape.serialize() for shape in shapes],
        "materials": [material.serialize() for material in materials],
        "textures": [texture.serialize() for texture in textures],
    }
