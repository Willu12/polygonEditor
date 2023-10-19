extern crate sfml;

mod point;
mod polygon;
mod restrictions;
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
                            let was_point_selected = selected_point_index.is_some();

                            if selected_edge.is_none() && selected_polygon_index.is_none() {
                                selected_point_index = find_clicked_point(x as f32, y as f32, &mut polygons, selected_point_index);
                                if selected_point_index.is_some() || was_point_selected {continue}
                            }

                            let was_edge_selected = selected_edge.is_some();

                            if selected_point_index.is_none() && selected_polygon_index.is_none() {
                                selected_edge = find_clicked_edge(x as f32, y as f32, &mut polygons, selected_edge);
                                if selected_edge.is_some() || was_edge_selected {continue}
                            }

                            let was_polygon_selected = selected_polygon_index.is_some();
                            
                            if selected_edge.is_none() && selected_point_index.is_none() {
                                selected_polygon_index = find_clicked_polygon(x as f32, y as f32, &mut polygons, selected_polygon_index);
                                if selected_polygon_index.is_some() || was_polygon_selected {continue};
                            }
                            current_starting_point = Some(Vector2f::new(x as f32, y as f32));
                            polygon_builder.polygon.points.push(point);
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
