use crate::sfml::graphics::RenderTarget;
use crate::Point;
use sfml::graphics::*;

#[derive(Default)]
pub struct Polygon<'a> {
    pub points: Vec<Point<'a>>,
}

#[derive(Default)]
pub struct PolygonBuilder<'a> {
    pub polygon: Polygon<'a>,
}

impl<'a> PolygonBuilder<'a> {

    pub fn build (mut self) -> Polygon<'a> {
        return std::mem::replace(&mut self.polygon, Polygon::default());
    }
}

impl<'a> Polygon<'a> {

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