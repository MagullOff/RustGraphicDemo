use self::point_light::PointLight;
use self::reflector::Reflector;

mod point_light;
mod reflector;

pub enum Light {
    NonTargeted(PointLight),
    Targeted(Reflector),
}
