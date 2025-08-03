from scripts.common.camera import Camera
from scripts.common.materials import Lambertian, Material
from scripts.common.shapes import Shape, Quad
from scripts.common.texture import SolidColor, Texture
from scripts.common.vec import Vec3

def generate():
    textures: list[Texture] = [
        SolidColor(color=Vec3(x = 1.0, y = 0.2, z = 0.2,)),
        SolidColor(color=Vec3(x = 0.2, y = 1.0, z = 0.2,)),
        SolidColor(color=Vec3(x = 0.2, y = 0.2, z = 1.0,)),
        SolidColor(color=Vec3(x = 1.0, y = 0.5, z = 0.0,)),
        SolidColor(color=Vec3(x = 0.2, y = 0.8, z = 0.8,)),
    ]

    materials: list[Material] = [
        Lambertian(texture = 0),
        Lambertian(texture = 1),
        Lambertian(texture = 2),
        Lambertian(texture = 3),
        Lambertian(texture = 4),
    ]

    shapes: list[Shape] = [
        Quad(
            top_left = Vec3(x = -3, y = -2, z = 5),
            u = Vec3(x = 0, y = 0, z = -4),
            v = Vec3(x = 0, y = 4, z = 0),
            material = 0,
        ),
        Quad(
            top_left = Vec3(x = -2, y = -2, z = 0),
            u = Vec3(x = 4, y = 0, z = 0),
            v = Vec3(x = 0, y = 4, z = 0),
            material = 1,
        ),
        Quad(
            top_left = Vec3(x = 3, y = -2, z = 1),
            u = Vec3(x = 0, y = 0, z = 4),
            v = Vec3(x = 0, y = 4, z = 0),
            material = 2,
        ),
        Quad(
            top_left = Vec3(x = -2, y = 3, z = 1),
            u = Vec3(x = 4, y = 0, z = 0),
            v = Vec3(x = 0, y = 0, z = 4),
            material = 3,
        ),
        Quad(
            top_left = Vec3(x = -2, y = -3, z = 5),
            u = Vec3(x = 4, y = 0, z = 0),
            v = Vec3(x = 0, y = 0, z = -4),
            material = 4,
        ),
    ]

    camera = Camera(
        look_at = Vec3(x = 0, y = 0, z = 0),
        look_from = Vec3(x = 0, y = 0, z = 9),
    ).serialize()

    return {
        "camera": camera,
        "objects": [shape.serialize() for shape in shapes],
        "materials": [material.serialize() for material in materials],
        "textures": [texture.serialize() for texture in textures],
    }
