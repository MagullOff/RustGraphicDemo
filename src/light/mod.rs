use self::point_light::PointLight;
use self::reflector::Reflector;

pub mod point_light;
pub mod reflector;

pub enum Light {
    NonTargeted(PointLight),
    Targeted(Reflector),
}
