use sfml::graphics::RenderWindow;
use std::cmp::Ordering;

use crate::Polygon;
use crate::Vector2f;
use crate::Point;
use crate::sfml::graphics::*;


pub struct Line {
    pub a: f32,
    pub b: f32,
}

pub fn render_intersection_points(polygon: &Polygon, window: &mut RenderWindow) {

    let offset = 30.0;
    let vertexes = find_all_intersection_points(create_moved_parallel_edges(polygon, offset));

    for v in vertexes.iter() {
        let mut point = Point::new(v.x,v.y);
        point.shape.set_fill_color(Color::RED);
        window.draw(&point.shape);

    }
}

pub fn create_extern_border<'a>(polygon: &Polygon,offset: f32) -> Polygon<'a> {

    let mut vertices = find_all_intersection_points(create_moved_parallel_edges(polygon, offset));

    vertices.sort_by(|x,y| x.x.total_cmp(&y.x));
    let starting_point = vertices.iter().min_by(|x,y| x.y.total_cmp(&y.y)).unwrap().clone();

    vertices.sort_by(|x,y| compare_vertices(x,y,&starting_point));

    let p = vertices;


    return Polygon::default();


}

pub fn graham(points: Vec<Vector2f>) -> Vec<Vector2f> {
    let mut S :Vec<Vector2f> = vec![];

    if points.len() < 3 {return S};
    S.push(points[0]);
    S.push(points[1]);

   
    return S;
}

fn is_turn_left(a:Vector2f,b:Vector2f,c:Vector2f) {
    
}

fn compare_vertices(p1: &Vector2f, p2: &Vector2f,start:&Vector2f) -> Ordering {
    // Oblicz różnice kątów między punktami p1 i p2 oraz punktem początkowym p[0]
    let angle1 = (p1.y - start.y).atan2(p1.x - start.x);
    let angle2 = (p2.y - start.y).atan2(p2.x - start.x);

    angle1.partial_cmp(&angle2).unwrap_or(Ordering::Equal)
}


pub fn get_line_from_points(start:Vector2f, end: Vector2f) -> Line {
    let a = (end.y - start.y)/ (end.x - start.x);
    let b = start.y - a * start.x;

    Line {a: a, b:b}
}

pub fn find_intersection_point(first: &Line, second: &Line) -> Vector2f {
    let x = (second.b - first.b) / (first.a - second.a);
    let y = first.a * x + first.b;
    Vector2f::new(x,y) 
}

pub fn find_all_intersection_points(lines: Vec<Line>) -> Vec<Vector2f> {
    let mut intersection_points: Vec<Vector2f> = vec![];

    for first_line in lines.iter() {
        for second_line in lines.iter() {
            if first_line.a == second_line.a && first_line.b == second_line.b {continue};

            intersection_points.push(find_intersection_point(first_line, second_line));
        }   
    }
    
    return intersection_points;
}

pub fn create_moved_parallel_edges(polygon: &Polygon, offset: f32) -> Vec<Line> {

    let mut parallel_lines: Vec<Line> = vec![];

    for (start_point,end_point) in polygon.points.iter().zip(polygon.points.iter().cycle().skip(1)) {

        let start = start_point.vertex.position;
        let end = end_point.vertex.position;

        let dx = end.x - start.x;
        let dy = end.y - start.y;

        let perpendicular_x = -dy; 
        let perpendicular_y = dx; 

        let length = (perpendicular_x * perpendicular_x + perpendicular_y * perpendicular_y).sqrt();
        let scale = offset / length;
        let vector = Vector2f::new(perpendicular_x * scale, perpendicular_y* scale);
        
        parallel_lines.push(get_line_from_points(start - vector, end - vector));
        parallel_lines.push(get_line_from_points(start + vector, end + vector))
    }

    return parallel_lines;

}
