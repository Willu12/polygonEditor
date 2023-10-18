use sfml::window::*;

use crate::{point::PointIndex, polygon::Polygon};

pub fn released_key_event_handler(code: Key,polygons: &mut Vec<Polygon> , mut selected_point_index: Option<PointIndex>, selected_edge_index: Option<(PointIndex,PointIndex)>)
 -> (Option<PointIndex>, Option<(PointIndex,PointIndex)> ) {
    match code {
        Key::D => {
            if let Some(index) = selected_point_index {
                polygons.get_mut(index.polygon_index).unwrap().remove_point(index.point_index);
                selected_point_index = None;
            }
        }
        Key::A => {
            if let Some((edge_start_index,edge_end_index)) = selected_edge_index {
                polygons.get_mut(edge_start_index.polygon_index).unwrap().add_point_to_edge(edge_start_index.point_index, edge_end_index.point_index);
            }
        }
        _ => {},
    }
    return (selected_point_index,selected_edge_index);
}