pub trait Movable {
    fn update(&mut self, tick: f32, animation: bool);
}
