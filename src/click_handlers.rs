use crate::polygon::*;

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