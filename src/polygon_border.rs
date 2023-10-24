use sfml::graphics::RenderWindow;
use std::vec;
use crate::PolygonBuilder;

use crate::Polygon;
use crate::Vector2f;
use crate::Point;
use crate::sfml::graphics::*;



pub fn render_intersection_points(polygon: &Polygon, window: &mut RenderWindow) {

    let offset = 30.0;
    let  vertices = calculate_all_border_points(&polygon, offset);

    for v in vertices.iter() {
        let point = Point::new(v.x,v.y);
        window.draw(&point.shape);
    }
   
}

pub fn create_extern_border<'a>(polygon: &Polygon,offset: f32) -> Polygon<'a> {

    let  vertices = calculate_all_border_points(&polygon, offset);

    let mut polygon_builder = PolygonBuilder::default();


    for point in vertices {
        polygon_builder.polygon.points.push(Point::new(point.x,point.y));
    }

    return polygon_builder.build();

}

fn calculate_normal_vector(start: Vector2f, end:Vector2f ) -> Vector2f {

    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let perpendicular_x = -dy; 
    let perpendicular_y = dx; 

    let mut normal_vector = Vector2f::new(perpendicular_x,perpendicular_y);
    normal_vector = normal_vector / (normal_vector.dot(normal_vector)).sqrt();
    
    return normal_vector;
}


pub fn calcluate_border_point(polygon: &Polygon, point_index: usize, offset: f32) -> Vector2f {

    let current_point = polygon.points[point_index].vertex.position.clone();
    let previous_point:Vector2f = if point_index != 0 {polygon.points[point_index - 1 ].vertex.position.clone()} else {polygon.points[polygon.points.len() -1 ].vertex.position.clone()};
    let next_point = polygon.points[(point_index + 1) % polygon.points.len()].vertex.position.clone();

    let n1 = calculate_normal_vector(previous_point, current_point);
    let n2 = calculate_normal_vector(current_point, next_point);

    let mut numerator = n1 + n2;
    //znormalizuj
    numerator = numerator / (numerator.dot(numerator)).sqrt();

    let dividor = ((n1.dot(n2) + 1.0 ) / 2.0).sqrt();

    return current_point + (numerator /dividor) * offset;
    
}

pub fn calculate_all_border_points(polygon: &Polygon, offset: f32) -> Vec<Vector2f> {

    let mut border_points: Vec<Vector2f> = vec![];
    for point_index in 0..polygon.points.len() {
        border_points.push(calcluate_border_point(&polygon, point_index,offset));
    }
    return border_points;
}
