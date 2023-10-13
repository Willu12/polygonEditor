//use crate::geometry::point;
extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;


pub struct Point<'a> {
    pub x: i32,
    pub y: i32,
    pub shape:  sfml::graphics::CircleShape<'a>,  
}

pub fn create_point_shape(x: i32, y:i32) -> sfml::graphics::CircleShape<'static>  {
    let mut circle =  CircleShape::default();
    circle.set_radius(3.0);
    circle.set_fill_color(Color::rgb(255, 0, 100));
    circle.set_position(Vector2f::new(x as f32, y as f32));

    circle
}