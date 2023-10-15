use crate::sfml::graphics::RenderTarget;
use crate::Point;
use sfml::graphics::*;


pub struct Polygon<'a> {
    pub points: Vec<Point<'a>>,
}

impl Polygon<'_> {

    pub fn render(&self, window: &mut RenderWindow) {        
       for point in self.points.iter() {
            window.draw(&(point.shape));
        }
        self.render_lines(window);
    }

    pub fn render_lines(&self, window:  &mut RenderWindow) {
        let mut vertex_array = Vec::<Vertex>::new();
        for point in self.points.iter() {
            vertex_array.push(point.vertex);
            window.draw_primitives(&vertex_array, PrimitiveType::LINE_STRIP, &RenderStates::default())
        }
    }
}


