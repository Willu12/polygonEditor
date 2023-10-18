use crate::{polygon::*, point::PointIndex};
use crate::Vector2f;

pub fn find_clicked_polygon(x: f32, y:f32, polygons: &Vec<Polygon>, mut selected_polygon_index: Option<usize>) -> Option<usize> {
    match find_polygon(x as f32, y as f32, &polygons) {
        Some(polygon_index) => {
            match selected_polygon_index {
                Some(selected_index) => {
                    if selected_index == polygon_index {return None}
                }
                None => {
                    selected_polygon_index = Some(polygon_index);
                },
            }
        },
        None => {
            if selected_polygon_index.is_some() {selected_polygon_index = None}
        },
    }
    return selected_polygon_index;
}

pub fn find_clicked_edge(x: f32, y:f32, polygons: &mut Vec<Polygon>, mut selected_edge: Option<(PointIndex,PointIndex)>) 
    -> Option<(PointIndex,PointIndex)> {
    
        match find_edge(x as f32, y as f32, &polygons) {
            Some((start_edge, end_edge)) => {
                match selected_edge {
                    Some((old_edge_start, old_edge_end)) => {
                        if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                            polygon.unselect_edge(old_edge_start.point_index, 
                                old_edge_end.point_index)
                        }
                        if (start_edge,end_edge) == (old_edge_start,old_edge_end) {return None}
                    },
                    None => {},
                }
                selected_edge = Some((start_edge,end_edge));
                if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                    polygon.select_edge(start_edge.point_index, 
                        end_edge.point_index,Vector2f::new(x as f32,y as f32))
                }
            },
            None => {   
                match selected_edge {
                    Some((start_edge, end_edge)) => {
                        if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                            polygon.unselect_edge(start_edge.point_index, 
                                end_edge.point_index)
                        }
                        selected_edge = None;
                    },
                    None => {
                        /* 
                        let is_polygon_selected = !selected_polygon_index.is_none();
                        selected_polygon_index = find_clicked_polygon(x as f32, y as f32, &polygons, selected_polygon_index);

                        if !is_polygon_selected && selected_polygon_index.is_none() {
                            current_starting_point = Some(Vector2f::new(x as f32, y as f32));
                            polygon_builder.polygon.points.push(point);
                        }
                        */
                    },
                }                        
            },
        }  


    return selected_edge;
}