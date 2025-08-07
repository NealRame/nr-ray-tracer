from dataclasses import dataclass, field

from .serializable import Serializable
from .vec import Vec3

@dataclass
class Camera(Serializable):
    background_color: Vec3 | None = field(default=None)
    look_at: Vec3 | None = field(default=None)
    look_from: Vec3 | None = field(default=None)
    view_up: Vec3 | None = field(default=None)

    samples_per_pixel: int | None = field(default=None)
    ray_max_bounces: int | None = field(default=None)

    defocus_angle: float | None = field(default=None)
    focus_dist: float | None = field(default=None)
    field_of_view: float | None = field(default=None)


    def serialize(self) -> dict | float | int | list | str | tuple:
        return {
            ** ({} if self.background_color == None else {
                "background_color": self.background_color.serialize(),
            }),
            ** ({} if self.look_at == None else {
                "look_at": self.look_at.serialize()
            }),
            ** ({} if self.look_from == None else {
                "look_from": self.look_from.serialize()
            }),
            ** ({} if self.view_up == None else {
                "view_up": self.view_up.serialize()
            }),
            ** ({} if self.samples_per_pixel == None else {
                "samples_per_pixel": self.samples_per_pixel,
            }),
            ** ({} if self.ray_max_bounces == None else {
                "ray_max_bounces": self.ray_max_bounces,
            }),
            ** ({} if self.defocus_angle == None else {
                "defocus_angle": self.defocus_angle,
            }),
            ** ({} if self.focus_dist == None else {
                "focus_dist": self.focus_dist,
            }),
            ** ({} if self.field_of_view == None else {
                "field_of_view": self.field_of_view,
            }),
        }
