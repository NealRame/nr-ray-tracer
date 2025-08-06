from scripts.common.camera import Camera
from scripts.common.materials import DiffuseLight, Lambertian, Material
from scripts.common.shapes import Quad, Shape, Sphere
from scripts.common.texture import Marble, SolidColor, Texture
from scripts.common.vec import Vec3

def generate():
    textures: list[Texture] = [
        SolidColor(color=Vec3(
            x = 0.4,
            y = 0.4,
            z = 0.4,
        )),
        Marble(frequency=0.2),
        SolidColor(color=Vec3(
            x = 4.00,
            y = 2.00,
            z = 1.00,
        )),
        SolidColor(color=Vec3(
            x = 1.00,
            y = 2.00,
            z = 4.00,
        )),
    ]

    materials: list[Material] = [
        Lambertian(texture = 0),
        Lambertian(texture = 1),
        DiffuseLight(texture = 2),
        DiffuseLight(texture = 3),
    ]

    shapes: list[Shape] = [
        Sphere(
            center = Vec3(x=0, y=-1000000, z=0),
            radius = 1000000,
            material = 0,
        ),
        Sphere(
            center = Vec3(x=0, y=2, z=0),
            radius = 2,
            material = 1,
        ),
        Quad(
            top_left = Vec3(x = 3, y = 1, z = -2),
            u = Vec3(x = 2, y = 0, z = 0),
            v = Vec3(x = 0, y = 2, z = 0),
            material = 2,
        ),
        Sphere(
            center = Vec3(x=0, y=7, z=0),
            radius = 1,
            material = 3,
        ),
    ]

    camera = Camera(
        background_color = Vec3(x = 0.001, y = 0.001, z = 0.001),
        look_at = Vec3(x = 0, y = 2, z = 0),
        look_from = Vec3(x = 26, y = 3, z = 6),
    ).serialize()

    return {
        "camera": camera,
        "objects": [shape.serialize() for shape in shapes],
        "materials": [material.serialize() for material in materials],
        "textures": [texture.serialize() for texture in textures],
    }
