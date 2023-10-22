
use sfml::system::Vector2f;
use crate::restrictions::*;

use crate::{Polygon, polygon::PolygonBuilder, point::Point, restrictions::Restriction};

pub fn create_sample_polygons<'a>() -> Vec<Polygon<'a>> {
    let mut polygons: Vec::<Polygon<'a>> = vec![];

    let mut polygon_builder = PolygonBuilder::default();

    //points of first polygon
    let mut first_polygon_points: Vec<Vector2f> = vec![];

    let scale_factor = 1.0;  // Increase the size by a factor of 2
    let x_offset = 250.0;  // Move to the right by 200 units

    let x_offset_right = 500.0;  // Move to the right by 600 units


    first_polygon_points.push(Vector2f::new(x_offset, 300.0));  // Left center, moved right
    first_polygon_points.push(Vector2f::new(x_offset + 200.0 * scale_factor, 300.0));  // Right
    first_polygon_points.push(Vector2f::new(x_offset + 200.0 * scale_factor, 400.0 * scale_factor));  // Right-bottom
    first_polygon_points.push(Vector2f::new(x_offset + 100.0 * scale_factor, 450.0 * scale_factor));  // Bottom-center
    first_polygon_points.push(Vector2f::new(x_offset, 400.0 * scale_factor));  // Bottom-left
    first_polygon_points.push(Vector2f::new(x_offset - 100.0 * scale_factor, 450.0 * scale_factor));  // Bottom-left-center
    first_polygon_points.push(Vector2f::new(x_offset - 200.0 * scale_factor, 400.0 * scale_factor));  // Bottom-left

    let mut second_polygon_points: Vec<Vector2f> = Vec::with_capacity(7);
    second_polygon_points.push(Vector2f::new(x_offset_right, 100.0));  // Right center, moved right
    second_polygon_points.push(Vector2f::new(x_offset_right + 200.0 * scale_factor, 100.0));  // Right
    second_polygon_points.push(Vector2f::new(x_offset_right + 123.0 * scale_factor, 123.0 * scale_factor));  // Right-bottom
    second_polygon_points.push(Vector2f::new(x_offset_right, 200.0 * scale_factor));  // Bottom-left
    second_polygon_points.push(Vector2f::new(x_offset_right - 200.0 * scale_factor, 225.0 * scale_factor));  // Bottom-left


    //dodaj restrykcje do pierwszego
   



    for point in first_polygon_points {
        polygon_builder.polygon.points.push(Point::new(point.x,point.y));
    }

    polygon_builder.polygon.add_restriction(Restriction { start_index: 1, end_index: 2, restriction: RestrictionKind::Horizontal});
    polygon_builder.polygon.add_restriction(Restriction { start_index: 5, end_index: 6, restriction: RestrictionKind::Vertical});
    polygons.push(polygon_builder.build());
    polygon_builder = PolygonBuilder::default();

    for point in second_polygon_points {
        polygon_builder.polygon.points.push(Point::new(point.x,point.y));        
    }

    polygon_builder.polygon.add_restriction(Restriction { start_index: 1, end_index: 2, restriction: RestrictionKind::Vertical});
    polygon_builder.polygon.add_restriction(Restriction { start_index: 2, end_index: 3, restriction: RestrictionKind::Horizontal});

    polygons.push(polygon_builder.build());

    
    return polygons;

}