from scripts.common.camera import Camera
from scripts.common.materials import Lambertian, Metal, Material
from scripts.common.shapes import Shape, Sphere
from scripts.common.texture import Checker, SolidColor, Texture
from scripts.common.vec import Vec3

def generate():
    textures: list[Texture] = [
        SolidColor(color = Vec3(
            x = 0.5,
            y = 0.5,
            z = 0.5,
        )),
        Checker(
            even = Vec3(
                x = 1,
                y = 0,
                z = 0,
            ),
            odd = Vec3(
                x = 1,
                y = 1,
                z = 1,
            ),
            scale = 9,
        ),
    ]

    materials: list[Material] = [
        Metal(texture = 0, fuzz = 0.015625),
        Lambertian(texture = 1),
    ]

    shapes: list[Shape] = [
        Sphere(
            center = Vec3(x=0, y=-10000, z=0),
            radius = 10000,
            material = 0,
        ),
        Sphere(
            center = Vec3(x=0, y=+10, z=0),
            radius = 10,
            material = 1,
        ),
    ]

    camera = Camera(
        look_from = Vec3(x=80.0, y=30.0, z=3.0),
        look_at = Vec3(x=0, y=10, z=0),
    ).serialize()

    return {
        "camera": camera,
        "objects": [shape.serialize() for shape in shapes],
        "materials": [material.serialize() for material in materials],
        "textures": [texture.serialize() for texture in textures],
    }
