
use sfml::graphics::Transformable;
use sfml::system::Vector2f;
use crate::polygon::create_polygon_from_vertices;
use crate::{restrictions::*, polygon};

use crate::{Polygon, polygon::PolygonBuilder, point::Point, restrictions::Restriction};

pub fn create_sample_polygons<'a>() -> Vec<Polygon<'a>> {
    let mut polygons: Vec::<Polygon<'a>> = vec![];

    let mut polygon_builder = PolygonBuilder::default();

    //points of first polygon
    let mut first_polygon_points: Vec<Vector2f> = vec![];

    let scale_factor = 1.0;  
    let x_offset = 250.0;  

    let x_offset_right = 500.0;  

    first_polygon_points.push(Vector2f::new(x_offset, 300.0));  
    first_polygon_points.push(Vector2f::new(x_offset + 200.0 * scale_factor, 300.0));  
    first_polygon_points.push(Vector2f::new(x_offset + 200.0 * scale_factor, 400.0 * scale_factor));  
    first_polygon_points.push(Vector2f::new(x_offset + 100.0 * scale_factor, 450.0 * scale_factor));  
    first_polygon_points.push(Vector2f::new(x_offset, 450.0 * scale_factor));  
    first_polygon_points.push(Vector2f::new(x_offset - 100.0 * scale_factor, 530.0 * scale_factor));  
    first_polygon_points.push(Vector2f::new(x_offset - 200.0 * scale_factor, 400.0 * scale_factor));  

    let mut second_polygon_points: Vec<Vector2f> = Vec::with_capacity(7);
    second_polygon_points.push(Vector2f::new(x_offset_right, 100.0));  
    second_polygon_points.push(Vector2f::new(x_offset_right + 200.0 * scale_factor, 100.0));  
    second_polygon_points.push(Vector2f::new(x_offset_right + 123.0 * scale_factor, 123.0 * scale_factor));  
    second_polygon_points.push(Vector2f::new(x_offset_right, 200.0 * scale_factor));  
    second_polygon_points.push(Vector2f::new(x_offset_right - 200.0 * scale_factor, 225.0 * scale_factor));  

    let mut points = vec![
        (591,209),
        (657,204),
        (652,84),
        (547,73),
        (488,328),
        (609,419),
        (666,301),
        (606,295),
        (565,292),
        (549,234),
        (555,207)
    ];

    let mut vertices: Vec<Vector2f> = vec![];

    for p in points {
        vertices.push(Vector2f::new(p.0 as f32,p.1 as f32));
    }

    let mut polygon = create_polygon_from_vertices(vertices);

    for point in first_polygon_points {
        polygon_builder.polygon.points.push(Point::new(point.x,point.y));
    }
    
    polygon_builder.polygon.add_restriction(Restriction { start_index: 2, end_index: 3, restriction: RestrictionKind::Horizontal});
    polygon_builder.polygon.add_restriction(Restriction { start_index: 5, end_index: 6, restriction: RestrictionKind::Vertical});

    polygon_builder.polygon.move_point(2,x_offset + 200.0 * scale_factor + 10.0, 400.0 * scale_factor);
    polygon_builder.polygon.move_point(5,x_offset - 100.0 * scale_factor, 530.0 * scale_factor);

    polygons.push(polygon_builder.build());
    polygon_builder = PolygonBuilder::default();

    polygon.add_restriction(Restriction { start_index: 8, end_index: 9, restriction: RestrictionKind::Vertical });
    polygon.add_restriction(Restriction { start_index: 2, end_index: 3, restriction: RestrictionKind::Horizontal });

    
    polygons.push(polygon);

    
    return polygons;

}