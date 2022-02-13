use crate::vec3::Vec3f;
use crate::frame::Frame;
use crate::object::Object;
use crate::light::Light;

pub struct Scene {
    pub frame: Frame,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
    pub bg_color: Vec3f
}