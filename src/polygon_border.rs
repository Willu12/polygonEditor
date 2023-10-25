use sfml::graphics::RenderWindow;
use std::vec;
use crate::PolygonBuilder;

use crate::Polygon;
use crate::Vector2f;
use crate::Point;
use crate::polygon::create_polygon_from_vertices;
use crate::sfml::graphics::*;
use geo::*;



pub fn render_intersection_points(polygon: &Polygon, window: &mut RenderWindow) {

    let offset = 30.0;
    let  vertices = calculate_all_border_points(&polygon, offset);

    for v in vertices.iter() {
        let point = Point::new(v.x,v.y);
        window.draw(&point.shape);
    }
}

    pub fn create_naive_border<'a>(polygon: &Polygon,offset: f32) -> Polygon<'a> {

        let  vertices = calculate_all_border_points(&polygon, offset);
    
        let mut polygon_builder = PolygonBuilder::default();
    
    
        for point in vertices {
            polygon_builder.polygon.points.push(Point::new(point.x,point.y));
        }
    
        let mut polygon_border =  polygon_builder.build();    
        
        for p in polygon_border.points.iter_mut() {
            p.vertex.color = Color::YELLOW;
        }
        return polygon_border;
    }
    

pub fn create_extern_border<'a>(polygon: &Polygon,offset: f32) -> Vec<Polygon<'a>> {

    let  vertices = calculate_all_border_points(&polygon, offset);

    let mut polygon_builder = PolygonBuilder::default();


    for point in vertices {
        polygon_builder.polygon.points.push(Point::new(point.x,point.y));
    }

    let polygon_border =  polygon_builder.build();
    let polygons = fix_border(polygon_border,&polygon);


    return polygons;
}



fn fix_border<'a>(polygon: Polygon<'a>, base_polygon: &Polygon) -> Vec<Polygon<'a>> {

    let mut polygons: Vec<Polygon> = vec![];
    let mut new_polygon_vertices: Vec<Vector2f> = vec![];
    let mut current_vertices: Vec<Vector2f> = vec![];
    let mut remaining_vertices: Vec<Vector2f> = vec![];
    let mut base_polygon_vertices: Vec<Vector2f> = vec![];

    for p in base_polygon.points.iter() {
        base_polygon_vertices.push(p.vertex.position);
    }

    for point in polygon.points {
        current_vertices.push(point.vertex.position);
    }

    while current_vertices.len() > 2 {

        remaining_vertices = vec![];
        new_polygon_vertices = vec![];
        new_polygon_vertices.push(current_vertices[0].clone());
        for v in current_vertices.iter().skip(1) {
            
            if is_edge_intersecting(&current_vertices, new_polygon_vertices.last().unwrap().clone(), v.clone()) == false &&   is_edge_intersecting(&base_polygon_vertices, new_polygon_vertices.last().unwrap().clone(), v.clone() ) == false {
                new_polygon_vertices.push(v.clone());
            }
            else {
                remaining_vertices.push(v.clone());
            }
        }

        while new_polygon_vertices.len() > 0 && (is_edge_intersecting(&current_vertices,new_polygon_vertices.first().unwrap().clone(), new_polygon_vertices.last().unwrap().clone()) || is_edge_intersecting(&base_polygon_vertices,new_polygon_vertices.first().unwrap().clone(), new_polygon_vertices.last().unwrap().clone()))
        {
            remaining_vertices.push(new_polygon_vertices.pop().unwrap());
        }
        if new_polygon_vertices.len() > 2 {
        polygons.push(create_polygon_from_vertices(new_polygon_vertices.clone()));
        }
        current_vertices = remaining_vertices.clone();
       
    }

    return polygons;
}

fn is_edge_intersecting(polygon: &Vec<Vector2f>, start: Vector2f, end:Vector2f) -> bool {

    let edge = Line::new(coord!{x: start.x, y: start.y}, coord!{x:end.x, y:end.y});

        for (start_point,end_point) in polygon.iter().zip(polygon.iter().cycle().skip(1)){

            let segment = Line::new(coord!{x: start_point.x, y: start_point.y}, coord!{x:end_point.x, y: end_point.y});
            
                if *start_point == start || *start_point == end || *end_point == start || *end_point == end {continue;}

                if edge.intersects(&segment) {return true};
        }
        return false;
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
    let mut previous_point:Vector2f = if point_index != 0 {polygon.points[point_index - 1 ].vertex.position.clone()} else {polygon.points[polygon.points.len() -1 ].vertex.position.clone()};
    let mut  next_point = polygon.points[(point_index + 1) % polygon.points.len()].vertex.position.clone();

    if polygon.is_clockwise() {(previous_point,next_point) = (next_point, previous_point)};

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
