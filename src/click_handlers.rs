use crate::algorithms::AlgorithmButton;
use crate::{polygon::*, point::PointIndex};
use crate::Vector2f;


pub fn find_clicked_button(x:f32, y:f32,buttons: &Vec<AlgorithmButton>) ->Option<usize> {

    for (index,button) in buttons.iter().enumerate() {
        let dist = ((button.position.x - x).powi(2) + (button.position.y - y).powi(2)).sqrt();
        if dist < button.radius + 5.0 {return Some(index)};
    }
    return None;
}

pub fn find_clicked_polygon(x: f32, y:f32, polygons: &mut Vec<Polygon>, mut selected_polygon_index: Option<usize>) -> Option<usize> {
    match find_polygon(x as f32, y as f32, &polygons) {
        Some(polygon_index) => {
            match selected_polygon_index {
                Some(selected_index) => {
                    if selected_index == polygon_index {
                        if let Some(polygon) = polygons.get_mut(polygon_index) {polygon.drag_position = None;}
                        return None
                    }
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

                if let Some((old_edge_start,old_edge_end)) = selected_edge  {
                    if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                        polygon.unselect_edge(old_edge_start.point_index, 
                            old_edge_end.point_index)
                    }
                    if (start_edge,end_edge) == (old_edge_start,old_edge_end) {return None}
                }
                selected_edge = Some((start_edge,end_edge));
                if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                    polygon.select_edge(start_edge.point_index, 
                        end_edge.point_index,Vector2f::new(x as f32,y as f32))
                }
            },
            None => {   
                if let Some((start_edge, end_edge)) = selected_edge {
                    if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                        polygon.unselect_edge(start_edge.point_index, 
                            end_edge.point_index)
                    }
                    selected_edge = None;
                }                     
            },
        }  

    return selected_edge;
}

pub fn find_clicked_point(x: f32, y:f32, polygons: &mut Vec<Polygon>, mut selected_point_index: Option<PointIndex>) 
    -> Option<PointIndex> {

        match find_point_index(x as f32, y as f32, &polygons) {
            Some(p_index) => {

                if let Some(index) = selected_point_index {
                    polygons.get_mut(index.polygon_index).unwrap().points
                    .get_mut(index.point_index).unwrap().unselect();

                    if index == p_index {return None}
                }
                polygons.get_mut(p_index.polygon_index).unwrap().points
                        .get_mut(p_index.point_index).unwrap().select();

                selected_point_index = Some(p_index);
            }
            None => {
                if let Some(index) = selected_point_index {
                    polygons.get_mut(index.polygon_index).unwrap().points
                        .get_mut(index.point_index).unwrap().unselect();
                }
                selected_point_index = None;
            },
        }

        return selected_point_index;
    }