import math

from scripts.common.camera import Camera
from scripts.common.materials import DiffuseLight, Lambertian, Material
from scripts.common.shapes import Quad, Shape
from scripts.common.texture import SolidColor, Texture
from scripts.common.vec import Vec3

def generate_box(a: Vec3, b: Vec3, material: int):
    min_x = min(a.x, b.x)
    min_y = min(a.y, b.y)
    min_z = min(a.z, b.z)

    max_x = max(a.x, b.x)
    max_y = max(a.y, b.y)
    max_z = max(a.z, b.z)

    a = Vec3(x = min_x, y = min_y, z = min_z)
    b = Vec3(x = max_x, y = max_y, z = max_z)

    dx = Vec3(x = max_x - min_x)
    dy = Vec3(y = max_y - min_y)
    dz = Vec3(z = max_z - min_z)

    return [
        Quad( # Front
            top_left = Vec3(x = min_x, y = min_y, z = max_z),
            u = dx,
            v = dy,
            material = material,
        ),
        Quad( # Right
            top_left = Vec3(x = max_x, y = min_y, z = max_z),
            u = dz*-1,
            v = dy,
            material = material,
        ),
        Quad( # Back
            top_left = Vec3(x = max_x, y = min_y, z = min_z),
            u = dx*-1,
            v = dy,
            material = material,
        ),
        Quad( # Left
            top_left = Vec3(x = min_x, y = min_y, z = min_z),
            u = dz,
            v = dy,
            material = material,
        ),
        Quad( # Top
            top_left = Vec3(x = min_x, y = max_y, z = max_z),
            u = dx,
            v = dz*-1,
            material = material,
        ),
        Quad( # Bottom
            top_left = Vec3(x = min_x, y = min_y, z = min_z),
            u = dx,
            v = dz,
            material = material,
        ),
    ]

def generate():
    textures: list[Texture] = [
        SolidColor(color = Vec3(
            x = 0.65,
            y = 0.05,
            z = 0.05,
        )), # red
        SolidColor(color = Vec3(
            x = 0.12,
            y = 0.45,
            z = 0.15,
        )), # green
        SolidColor(color = Vec3(
            x = 0.73,
            y = 0.73,
            z = 0.73,
        )), # white
        SolidColor(
            color = Vec3.one()*15
        ),  # light
        SolidColor(color = Vec3(
            x = 0.00,
            y = 0.82,
            z = 1.00,
        )),  # blue
    ]

    materials: list[Material] = [
        Lambertian(texture = 0),
        Lambertian(texture = 1),
        Lambertian(texture = 2),
        DiffuseLight(texture = 3),
    ]

    shapes: list[Shape] = [
        Quad(
            top_left = Vec3(x = 0, y = 0, z = 0),
            u = Vec3(x = 0, y = 555, z = 0),
            v = Vec3(x = 0, y = 0, z = 555),
            material = 0,
        ),
        Quad(
            top_left = Vec3(x = 555, y = 0, z = 0),
            u = Vec3(x = 0, y = 555, z = 0),
            v = Vec3(x = 0, y = 0, z = 555),
            material = 1,
        ),
        Quad(
            top_left = Vec3(x = 0, y = 0, z = 0),
            u = Vec3(x = 555, y = 0, z = 0),
            v = Vec3(x = 0, y = 0, z = 555),
            material = 2,
        ),
        Quad(
            top_left = Vec3(x = 555, y = 555, z = 555),
            u = Vec3(x = -555, y = 0, z = 0),
            v = Vec3(x = 0, y = 0, z = -555),
            material = 2,
        ),
        Quad(
            top_left = Vec3(x = 0, y = 0, z = 555),
            u = Vec3(x = 555, y = 0, z = 0),
            v = Vec3(x = 0, y = 555, z = 0),
            material = 2,
        ),
        Quad(
            top_left = Vec3(x = 343, y =  554, z =  332),
            u = Vec3(x = -130, y = 0, z = 0),
            v = Vec3(x = 0, y = 0, z = -105),
            material = 3,
        ),
    ]

    shapes.extend(generate_box(
        Vec3(x = 130, y = 0, z = 65),
        Vec3(x = 295, y = 165, z = 230),
        2,
    ))

    shapes.extend(generate_box(
        Vec3(x = 265, y = 0, z = 295),
        Vec3(x = 430, y = 330, z = 460),
        2,
    ))

    camera = Camera(
        background_color = Vec3(x = 0.001, y = 0.001, z = 0.001),
        look_at = Vec3(x = 278, y = 278, z = 0),
        look_from = Vec3(x = 278, y = 278, z = -800),
        defocus_angle = 0,
        field_of_view = (40*math.pi)/180,
        samples_per_pixel = 200,
        ray_max_bounces = 50,
    ).serialize()

    return {
        "camera": camera,
        "objects": [shape.serialize() for shape in shapes],
        "materials": [material.serialize() for material in materials],
        "textures": [texture.serialize() for texture in textures],
    }
