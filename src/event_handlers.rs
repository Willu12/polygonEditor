use sfml::window::*;
use crate::restrictions::*;
use crate::serializer::{save_polygons, load_polygons};

use crate::{point::PointIndex, polygon::Polygon, restrictions::Restriction};

pub fn released_key_event_handler(code: Key,polygons: &mut Vec<Polygon> , mut selected_point_index: Option<PointIndex>,
    selected_edge_index: Option<(PointIndex,PointIndex)>, mut display_borders : bool ) -> (Option<PointIndex>, Option<(PointIndex,PointIndex)>,bool) {
    match code {
        Key::D => {
            if let Some(index) = selected_point_index {
                polygons.get_mut(index.polygon_index).unwrap().remove_point(index.point_index);
                selected_point_index = None;
            }
        }
        Key::A => {
            if let Some((edge_start_index,edge_end_index)) = selected_edge_index {
                polygons.get_mut(edge_start_index.polygon_index).unwrap()
                .add_point_to_edge(edge_start_index.point_index, edge_end_index.point_index);
            }
        }
        Key::H => {
            if let Some((edge_start_index,edge_end_index)) = selected_edge_index {
                let new_restriction = Restriction {start_index: edge_start_index.point_index,
                    end_index: edge_end_index.point_index, restriction: RestrictionKind::Horizontal};

                if let Some(polygon) = polygons.get_mut(edge_start_index.polygon_index) {
                    polygon.add_restriction(new_restriction);
                }
            }
        }
        Key::V => {
            if let Some((edge_start_index,edge_end_index)) = selected_edge_index {
                let new_restriction = Restriction {start_index: edge_start_index.point_index,
                    end_index: edge_end_index.point_index, restriction: RestrictionKind::Vertical};

                if let Some(polygon) = polygons.get_mut(edge_start_index.polygon_index) {
                    polygon.add_restriction(new_restriction);
                }
            }
        }
        Key::B => {
            display_borders = !display_borders;
        }
        _ => {},
    }
    return (selected_point_index,selected_edge_index,display_borders);
}

pub fn mouse_moved_event_handler(x: f32, y:f32,polygons: &mut Vec<Polygon>, selected_point_index: Option<PointIndex>,
    selected_edge_index: Option<(PointIndex,PointIndex)>, selected_polygon_index: Option<usize>) {

        if let Some(index) = selected_point_index {
            polygons.get_mut(index.polygon_index).unwrap().move_point(index.point_index, x, y);
        }

        if let Some((edge_start_index,edge_end_index)) = selected_edge_index {
            polygons.get_mut(edge_start_index.polygon_index).unwrap().
            move_edge(edge_start_index.point_index, edge_end_index.point_index, x as f32, y as f32)
        }

        if let Some(polygon_index) = selected_polygon_index {
            if let Some(polygon) = polygons.get_mut(polygon_index) {
                polygon.move_polygon(x as f32, y as f32);
            }
        }
}