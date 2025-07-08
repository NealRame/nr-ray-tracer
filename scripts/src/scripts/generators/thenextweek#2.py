from scripts.common.camera import Camera
from scripts.common.materials import Lambertian, Metal
from scripts.common.shapes import Shape, Sphere
from scripts.common.texture import Checker, SolidColor
from scripts.common.vec import Vec3

def generate():
    tex1 = SolidColor.random()
    tex2 = Checker.random(scale = 24)

    shapes: list[Shape] = [
        Sphere(
            center = Vec3(x=0, y=-10, z=0),
            radius = 10,
            material = Metal(texture = tex1, fuzz = 0.03125),
        ),
        Sphere(
            center = Vec3(x=0, y=+10, z=0),
            radius = 10,
            material = Lambertian(texture = tex2),
        ),
    ]

    camera = Camera(
        look_from = Vec3(x=13.0, y=2.0, z=3.0),
        look_at = Vec3(x=0, y=0, z=0),
    ).serialize()

    objects = [shape.serialize() for shape in shapes]

    return {
        "camera": camera,
        "objects": objects,
    }
