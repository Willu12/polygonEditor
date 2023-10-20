use sfml::system::{Vector2i,Vector2f};
use sfml::graphics::*;

use crate::polygon::Polygon;

#[derive(Debug, PartialEq, Eq,Clone,Copy)]
pub enum DrawAlgorithm {
    Library,
    Bresenham,
}

pub fn render_lines_bresenham_builder(polygon: &Polygon ,window: &mut RenderWindow) {
    
    let mut bresenham_points: Vec<Vector2i> = vec![];

    let mut vertex_array = Vec::<Vertex>::new();

    for (first, second) in polygon.points.iter().zip(polygon.points.iter().skip(1)) { 
        let start_pos = first.vertex.position;
        let end_pos = second.vertex.position;
        let mut new_points = calculate_bresenham(
            Vector2i::new(start_pos.x as i32, start_pos.y as i32),
              Vector2i::new(end_pos.x as i32, end_pos.y as i32));

        bresenham_points.append(&mut new_points);        
    }

    for point in bresenham_points {
        let vertex = Vertex::new(Vector2f::new(point.x as f32,point.y as f32),Color::RED,Vector2f::default());
        vertex_array.push(vertex);
    }
    window.draw_primitives(&vertex_array, PrimitiveType::POINTS, &RenderStates::default())
}

pub fn render_lines_bresenham_polygon(polygon: &Polygon ,window: &mut RenderWindow) {
    
    let mut bresenham_points: Vec<Vector2i> = vec![];

    let mut vertex_array = Vec::<Vertex>::new();

    for (first, second) in polygon.points.iter().zip(polygon.points.iter().cycle().skip(1)) { 
        let start_pos = first.vertex.position;
        let end_pos = second.vertex.position;
        let mut new_points = calculate_bresenham(
            Vector2i::new(start_pos.x as i32, start_pos.y as i32),
              Vector2i::new(end_pos.x as i32, end_pos.y as i32));

        bresenham_points.append(&mut new_points);        
    }

    for point in bresenham_points {
        let vertex = Vertex::new(Vector2f::new(point.x as f32,point.y as f32),Color::GREEN,Vector2f::default());
        vertex_array.push(vertex);
    }
    window.draw_primitives(&vertex_array, PrimitiveType::POINTS, &RenderStates::default())
}


pub fn calculate_bresenham(start:Vector2i, end:Vector2i) -> Vec<Vector2i> {

    let mut calculated_points: Vec<Vector2i> = vec![];

    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let g = if dx > 0 {1} else {-1};
    let h = if dy > 0 {1} else {-1};

    let dx = dx.abs();
    let dy = dy.abs();

    let mut x = start.x;
    let mut y = start.y;

    if dx > dy {
        let mut c = -dx;

        while x != end.x {
            calculated_points.push(Vector2i::new(x,y));
            c = c + 2 * dy;

            if c > 0 {
                y = y + h;
                c = c - 2 * dx;
            }
            x = x + g;
        }
    }
    else {
        let mut c = -dy;
        while y != end.y {
            calculated_points.push(Vector2i::new(x,y));

            c = c + 2 * dx;

            if c > 0 {
                x = x + g;
                c = c - 2 * dy;
            }
            y = y + h;
        }

    }
    return  calculated_points;
}