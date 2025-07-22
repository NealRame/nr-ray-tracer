from scripts.common.camera import Camera
from scripts.common.materials import Lambertian, Metal, Material
from scripts.common.shapes import Shape, Sphere
from scripts.common.texture import Image, SolidColor, Texture
from scripts.common.vec import Vec3

def generate():
    textures: list[Texture] = [
        SolidColor(color=Vec3(
            x = 0.4,
            y = 0.4,
            z = 0.4,
        )),
        Image(file = "scenes/earth.jpg"),
        Image(file = "scenes/moon.jpg"),
    ]

    materials: list[Material] = [
        Metal(texture = 0, fuzz = 0.03125),
        Lambertian(texture = 1),
        Lambertian(texture = 2),
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
        Sphere(
            center = Vec3(x=-12, y=+12, z=-20),
            radius = 3,
            material = 2,
        ),
    ]

    camera = Camera(
        look_at = Vec3(x=0, y=10, z=0),
        look_from = Vec3(x=60.0, y=20.0, z=3.0),
    ).serialize()

    return {
        "camera": camera,
        "objects": [shape.serialize() for shape in shapes],
        "materials": [material.serialize() for material in materials],
        "textures": [texture.serialize() for texture in textures],
    }
