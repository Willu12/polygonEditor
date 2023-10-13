use crate::sfml::graphics::RenderTarget;
use crate::Point;
use sfml::{graphics::RenderWindow};


pub struct Polygon<'a> {
    pub points: Vec<Point<'a>>,
}

impl Polygon<'_> {

    pub fn render(&self, window: &mut RenderWindow) {
       for point in self.points.iter() {
            window.draw(&(point.shape));
        }
    }
}


