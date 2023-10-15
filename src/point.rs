//use crate::geometry::point;
extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;


pub struct Point<'a> {
    pub vertex: sfml::graphics::Vertex,
    pub shape:  sfml::graphics::CircleShape<'a>,  
}

impl Point<'_> { 
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            vertex : Vertex { position: Vector2f::new(x,y), color : Color::RED, tex_coords: Vector2f::new(x,y) },
            shape: create_point_shape( x, y),
        }
    }
}

pub fn create_point_shape(x: f32, y:f32) -> sfml::graphics::CircleShape<'static>  {
    let mut circle =  CircleShape::default();
    circle.set_radius(3.0);
    circle.set_fill_color(Color::rgb(255, 192, 203));
    circle.set_position(Vector2f::new(x, y));

    circle
}