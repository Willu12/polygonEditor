extern crate sfml;

mod point;
mod polygon;
mod event_handlers;
mod click_handlers;
use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;
//use sfml::system::*;
use crate::point::*;
use crate::polygon::*;
use crate::event_handlers::*;
use crate::click_handlers::*;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Bar mleczny sloneczny - poprosze nervosol",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let mut current_starting_point: Option<Vector2f> = None;
    let mut polygons: Vec::<Polygon> = vec![];
    let mut polygon_builder = PolygonBuilder::default();
    let mut selected_point_index: Option<PointIndex> = None;
    let mut selected_edge: Option<(PointIndex,PointIndex)> = None;
    let mut selected_polygon_index: Option<usize> = None;
    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                },
                Event::KeyReleased { code, alt:_t, ctrl:_, shift:_, system:_ } => {
                    (selected_point_index,selected_edge) = released_key_event_handler(code, &mut polygons,
                         selected_point_index, selected_edge);
                }
                Event::MouseMoved { x, y } => {   
                  mouse_moved_event_handler(x as f32, y as f32, &mut polygons, selected_point_index,
                        selected_edge, selected_polygon_index);
                },
                Event::MouseButtonReleased { button: _, x, y } => {
                    let point = Point::new(x as f32,y as f32);

                    match current_starting_point {
                        Some(p) => {
                            if point.intersects(p)   {
                                polygons.push(polygon_builder.build());
                                polygon_builder = PolygonBuilder::default();
                                current_starting_point = None;
                            } 
                            else {polygon_builder.polygon.points.push(point)}
                        },
                        None => {

                            //check if point is clicked:
                            //selected_point_index = 

                            match find_point_index(x as f32, y as f32, &polygons) {
                                Some(p_index) => {
                                    match selected_point_index {
                                        Some(index) => {
                                            polygons.get_mut(index.polygon_index).unwrap().points
                                            .get_mut(index.point_index).unwrap().unselect();

                                        if index == p_index {selected_point_index = None; continue;}
                                    },
                                        None => {},
                                    }
                                    polygons.get_mut(p_index.polygon_index).unwrap().points
                                            .get_mut(p_index.point_index).unwrap().select();

                                    selected_point_index = Some(p_index);
                                }
                                None => {
                                    match selected_point_index {
                                        Some(index) => {
                                            polygons.get_mut(index.polygon_index).unwrap().points
                                            .get_mut(index.point_index).unwrap().unselect();
                                    },
                                        None => {     
                                            
                                            let is_edge_selected = selected_edge.is_some();
                                            selected_edge = find_clicked_edge(x as f32, y as f32, &mut polygons, selected_edge);
                                            
                                            if is_edge_selected == false && selected_edge.is_none() {

                                                let is_polygon_selected = !selected_polygon_index.is_none();
                                                selected_polygon_index = find_clicked_polygon(x as f32, y as f32,
                                                     &polygons, selected_polygon_index);

                                                if !is_polygon_selected && selected_polygon_index.is_none() {
                                                    current_starting_point = Some(Vector2f::new(x as f32, y as f32));
                                                    polygon_builder.polygon.points.push(point);
                                                }
                                            }
                                        },
                                    }
                                    selected_point_index = None;
                                },
                            }
                        },
                    }
                },
                _ => {},
            }
        }

        window.clear(Color::BLACK);
        
        for polygon in polygons.iter() {
            polygon.render(&mut window);
        }
        polygon_builder.render(&mut window);

        window.display();
    }

}
