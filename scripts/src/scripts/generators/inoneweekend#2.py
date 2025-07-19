import random

from scripts.common.camera import Camera
from scripts.common.materials import Dielectric, Lambertian, Material, Metal
from scripts.common.shapes import Shape, Sphere
from scripts.common.texture import SolidColor, Texture
from scripts.common.vec import Vec3
from scripts.common.seq import seq

def generate():
    textures: list[Texture] = [
        SolidColor(color = Vec3(
            x = 0.5,
            y = 0.5,
            z = 0.5,
        )),
    ]
    materials: list[Material] = [Lambertian(texture = 0)]

    shapes: list[Shape] = [Sphere(
        center = Vec3(x=0, y=-1000, z=0),
        radius = 1000,
        material = 0,
    )]

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

    choices = [
        generate_dielectric,
        generate_lambertian,
        generate_metal
    ]
    weights = [5, 80, 15]

    for a in seq(-11, 11, 1):
        for b in seq(-11, 11, 1):
            generate_material, = random.choices(choices, weights)

            m = generate_material()
            c = Vec3(
                x = a + 0.9*random.random(),
                y = 0.2,
                z = b + 0.9*random.random(),
            )

            if random.random() < 8/10:
                v = Vec3(x = 0, y = random.random()*0.5, z = 0.0)
            else:
                v = None

            shapes.append(Sphere(center=c, speed=v, radius=0.2, material=m))

    shapes.append(Sphere(
        center = Vec3(x = 0, y = 1, z = 0),
        radius = 1.0,
        material = generate_dielectric()
    ))
    shapes.append(Sphere(
        center = Vec3(x = -4, y = 1, z = 0),
        radius = 1.0,
        material = generate_lambertian()
    ))
    shapes.append(Sphere(
        center = Vec3(x = 4, y=1, z=0),
        radius = 1.0,
        material = generate_metal()
    ))

    camera = Camera(
        look_from = Vec3(x = 13, y = 2, z = 3),
        look_at = Vec3(x = 0, y = 0, z = 0),
    ).serialize()

    return {
        "camera": camera,
        "objects": [shape.serialize() for shape in shapes],
        "materials": [material.serialize() for material in materials],
        "textures": [texture.serialize() for texture in textures],
    }
