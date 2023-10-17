//use crate::geometry::point;
extern crate sfml;

use sfml::graphics::*;
use sfml::system::*;

#[derive(Clone)]
pub struct Point<'a> {
    pub vertex: sfml::graphics::Vertex,
    pub shape:  sfml::graphics::CircleShape<'a>,  
}
#[derive(Clone,Copy,PartialEq)]
pub struct PointIndex {
    pub polygon_index: usize,
    pub point_index: usize,
}

impl<'a> Point<'a> { 
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            vertex : Vertex { position: Vector2f::new(x,y), color : Color::RED, tex_coords: Vector2f::new(x,y) },
            shape: create_point_shape(x, y),
        }
    }

    pub fn intersects(&self, point: Vector2f) -> bool {
        let dist = ((self.vertex.position.x - point.x).powi(2) + (self.vertex.position.y - point.y).powi(2)).sqrt();
        if dist <= (6.0 + 10.0) {return  true} else {return  false}
    }

    pub fn select(&mut self) {
        self.shape.set_radius(2.0 * self.shape.radius());
        self.shape.set_fill_color(Color::WHITE);
        self.shape.set_position(Vector2f::new(self.vertex.position.x - self.shape.radius()/2.0, self.vertex.position.y - self.shape.radius()/2.0));
    }

    pub fn unselect(&mut self) {
        self.shape.set_radius(self.shape.radius() / 2.0);
        self.shape.set_fill_color(Color::rgb(255, 192, 203));
        self.shape.set_position(Vector2f::new(self.vertex.position.x - self.shape.radius()/2.0, self.vertex.position.y - self.shape.radius()/2.0));
    }

    pub fn change_position(&mut self, x:f32, y:f32) {
        self.vertex.position =  Vector2f::new(x,y);
        self.shape.set_position(Vector2f::new(x - self.shape.radius()/2.0, y - self.shape.radius()/2.0));
    }


}

pub fn create_point_shape<'a>(x: f32, y:f32) -> sfml::graphics::CircleShape<'a>  {
    let mut circle =  CircleShape::default();
    circle.set_radius(3.0);
    circle.set_fill_color(Color::rgb(255, 192, 203));
    circle.set_position(Vector2f::new(x - circle.radius()/2.0, y - circle.radius()/2.0));

    circle
}