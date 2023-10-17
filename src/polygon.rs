use crate::point::PointIndex;
use crate::sfml::graphics::RenderTarget;
use crate::Point;
use sfml::graphics::*;
use sfml::system::Vector2f;


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

        for point in self.polygon.points.iter_mut() {
            point.vertex.color = Color::GREEN;
        }
        return std::mem::replace(&mut self.polygon, Polygon::default());
    }

    pub fn render(&self, window: &mut RenderWindow) {        
        for point in self.polygon.points.iter() {
             window.draw(&(point.shape));
         }
         self.render_lines(window);
     }

     pub fn render_lines(&self, window:  &mut RenderWindow) {
        let mut vertex_array = Vec::<Vertex>::new();
        for point in self.polygon.points.iter() {
            vertex_array.push(point.vertex);
        }
        window.draw_primitives(&vertex_array, PrimitiveType::LINE_STRIP, &RenderStates::default())
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
        }
        match vertex_array.first() {
            Some(vertex) => {vertex_array.push(vertex.clone())},
            None => {}
        }
        window.draw_primitives(&vertex_array, PrimitiveType::LINE_STRIP, &RenderStates::default())
    }
}

pub fn find_point_index<'a> (x: f32, y:f32, polygons:& Vec<Polygon>) -> Option<PointIndex> {
    for (polygon_index, polygon) in polygons.iter().enumerate() {
        for (point_index, point) in polygon.points.iter().enumerate() {
            if point.intersects(Vector2f::new(x,y)) {return Some(PointIndex { polygon_index: polygon_index, point_index: point_index })} 
        }
    }
    return  None;
}