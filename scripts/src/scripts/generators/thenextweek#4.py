from scripts.common.camera import Camera
from scripts.common.materials import Lambertian, Metal, Material
from scripts.common.shapes import Shape, Sphere
from scripts.common.texture import Marble, PerlinRidged, SolidColor, Texture
from scripts.common.vec import Vec3

def generate():
    textures: list[Texture] = [
        SolidColor(color=Vec3(
            x = 0.4,
            y = 0.4,
            z = 0.4,
        )),
        PerlinRidged(octaves=8, frequency=0.2),
        Marble(frequency=0.2),
        SolidColor(color=Vec3(
            x = 1.00,
            y = 0.50,
            z = 0.65,
        )),
        SolidColor(color=Vec3(
            x = 0.23,
            y = 0.51,
            z = 0.88,
        ))
    ]

    materials: list[Material] = [
        Lambertian(texture = 0),
        Metal(texture = 1, fuzz=0.05),
        Lambertian(texture = 2),
        Metal(texture = 3, fuzz=0.9),
        Metal(texture = 4, fuzz=0.8),
    ]

    shapes: list[Shape] = [
        Sphere(
            center = Vec3(x=0, y=-1000000, z=0),
            radius = 1000000,
            material = 0,
        ),
        Sphere(
            center = Vec3(x=0, y=+10, z=0),
            radius = 10,
            material = 1,
        ),
        Sphere(
            center = Vec3(x=-40, y=+10, z=20),
            radius = 10,
            material = 2,
        ),
        Sphere(
            center = Vec3(x=30, y=+10, z=-20),
            radius = 10,
            material = 3,
        ),
        Sphere(
            center = Vec3(x=10, y=+10, z=25),
            radius = 10,
            material = 4,
        ),
    ]

    camera = Camera(
        look_at = Vec3(x=0, y=10, z=-2),
        look_from = Vec3(x=70.0, y=30.0),
    ).serialize()

    return {
        "camera": camera,
        "objects": [shape.serialize() for shape in shapes],
        "materials": [material.serialize() for material in materials],
        "textures": [texture.serialize() for texture in textures],
    }
