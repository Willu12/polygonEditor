use sfml::graphics::RenderWindow;
use std::cmp::Ordering;
use std::vec;
use crate::PolygonBuilder;

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
    let mut vertices = find_all_intersection_points(create_moved_parallel_edges(polygon, offset));

    

    vertices.sort_by(|x,y| x.x.total_cmp(&y.x));

    let starting_point = vertices.iter().min_by(|y,x| x.y.total_cmp(&y.y)).unwrap().clone();
    println!("{},{}",starting_point.x,starting_point.y);

    for v in vertices.iter() {
        let mut point = Point::new(v.x,v.y);
        if v.clone() == starting_point {point.shape.set_fill_color(Color::RED)} else {point.shape.set_fill_color(Color::MAGENTA)};
        window.draw(&point.shape);
    }
   
}

pub fn create_extern_border<'a>(polygon: &Polygon,offset: f32,window: &mut RenderWindow) -> Polygon<'a> {

    let mut polygon_builder = PolygonBuilder::default();
    let mut vertices = find_all_intersection_points(create_moved_parallel_edges(polygon, offset));

    vertices.sort_by(|x,y| x.x.total_cmp(&y.x));
    vertices.sort_by(|y,x| x.y.total_cmp(&y.y));

    let starting_point = vertices.remove(0);

    vertices.sort_by(|y,x| compare_vertices(x,y,&starting_point));
    //let remaining_vertices = vertices[1..vertices.len()];
    vertices.insert(0, starting_point);
    
    let font = Font::from_file("fonts/Roboto-Regular.ttf").expect("Failed to load font");

    for (index,point) in vertices.iter().enumerate() {
        let mut text = Text::new(index.to_string().as_str(), &font, 16);
        text.set_fill_color(Color::WHITE);
        text.set_position(Vector2f::new(point.x , point.y));

        window.draw(&text);
    }

    let p = vertices;

    let extern_border_points = graham(p);

    

    for point in extern_border_points {
        polygon_builder.polygon.points.push(Point::new(point.x,point.y));
    }

    return polygon_builder.build();

}

pub fn graham(points: Vec<Vector2f>) -> Vec<Vector2f> {
    let mut S :Vec<Vector2f> = vec![];

    if points.len() < 3 {return S};
    S.push(points[0]);
    S.push(points[1]);

    for k in 2..points.len() {
        if S.len() == 1 {S.push(points[k]); continue;}

        while is_turn_left(S.get(S.len()-2).unwrap().clone(), S.last().unwrap().clone(), points[k]) {
            S.pop();
            println!("popped vertex");
        }
        S.push(points[k]);
        println!("pushed {} vertex",k);
    }
    println!("stop");

    return S;
}

fn is_turn_left(p0: Vector2f, p1: Vector2f, p2: Vector2f) -> bool {
    cross_product(p2 - p0, p1 - p0) < 0.0
}

fn cross_product(v1: Vector2f, v2: Vector2f) -> f32 {
    let cross_product = v1.x * v2.y - v2.x * v1.y;
    println!("cross product is eequal {}",cross_product);
    return cross_product;

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

pub fn find_intersection_point(first: &Line, second: &Line) -> Option<Vector2f> {
    let x = (second.b - first.b) / (first.a - second.a);
    let y = first.a * x + first.b;

    if x >= 0.0 && x <= 800.0 && y >= 0.0 && y <= 600.0 {Some(Vector2f::new(x,y))} else {None}
    
}

pub fn find_all_intersection_points(lines: Vec<Line>) -> Vec<Vector2f> {
    let mut intersection_points: Vec<Vector2f> = vec![];

    for (index,first_line) in lines.iter().enumerate() {
        for second_line in lines.iter().skip(index + 1) {
            if first_line.a == second_line.a && first_line.b == second_line.b {continue};
            
            if let Some(intersection_point) = find_intersection_point(first_line, second_line) {
                intersection_points.push(intersection_point);
            }
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
