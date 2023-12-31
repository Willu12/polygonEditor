extern crate sfml;

mod polygon_border;
mod point;
mod polygon;
mod restrictions;
mod event_handlers;
mod algorithms;
mod sample;
mod click_handlers;
mod serializer;
use algorithms::AlgorithmButton;
use algorithms::DrawAlgorithm;
use polygon_border::create_extern_border;
use sample::create_sample_polygons;
use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;
use crate::point::*;
use crate::polygon::*;
use crate::event_handlers::*;
use crate::click_handlers::*;
use crate::serializer::*;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Polygon Editor",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let mut current_starting_point: Option<Vector2f> = None;
    let mut polygons: Vec::<Polygon> = create_sample_polygons();
    let mut polygon_builder = PolygonBuilder::default();
    let mut selected_point_index: Option<PointIndex> = None;
    let mut selected_edge: Option<(PointIndex,PointIndex)> = None;
    let mut selected_polygon_index: Option<usize> = None;
    let mut drawing_algorithm: DrawAlgorithm = DrawAlgorithm::Library;
    let mut display_borders: bool = false;
    let mut offset : f32 = 10.0;
    let  max_offset:f32   = 50.0;


    let mut buttons: Vec<AlgorithmButton> = vec![AlgorithmButton::new(Vector2f::new(20.0,40.0),DrawAlgorithm::Bresenham),
                                            AlgorithmButton::new(Vector2f::new(20.0,60.0),DrawAlgorithm::Library),
                                            AlgorithmButton::new(Vector2f::new(20.0,80.0),DrawAlgorithm::Aliasing)];

    buttons[0].active = true;
    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                },
                Event::KeyReleased { code, alt:_t, ctrl:_, shift:_, system:_ } => {
                    (selected_point_index,selected_edge,display_borders) = released_key_event_handler(code, &mut polygons,
                         selected_point_index, selected_edge,display_borders);

                    match code {
                        Key::S  => {
                            save_polygons(&polygons);
                        }
                        Key::L => {
                            polygons = load_polygons();
                        }
                        _ => {},
                    }
                },
                Event::KeyPressed { code, scan, alt, ctrl, shift, system } => {
                   match code {
                        Key::Up => {

                            if offset < max_offset && display_borders {offset = offset + 1.0};
                        }
                        Key::Down => {
                            if offset > 0.0 && display_borders {offset = offset - 1.0};
                        }
                        _ => {},
                    }
                }
                Event::MouseMoved { x, y } => {   
                  mouse_moved_event_handler(x as f32, y as f32, &mut polygons, selected_point_index,
                        selected_edge, selected_polygon_index);
                },
                Event::MouseButtonReleased { button: _, x, y } => {
                    let point = Point::new(x as f32,y as f32);

                    if let Some(selected_button) = find_clicked_button(x as f32, y as f32, &buttons) {
                        for button in buttons.iter_mut() {
                            button.active = false;
                        }
                        buttons[selected_button].active = true;
                        drawing_algorithm = buttons[selected_button].algorithm;
                        continue;
                    }


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
            polygon.render(&mut window,drawing_algorithm);
            
            if display_borders {
                let borders = create_extern_border(&polygon,offset);
                //let naive_border = create_naive_border(&polygon, offset);


                for border in borders {
                    border.render_border(&mut window, drawing_algorithm);
                }

               // naive_border.render(&mut window, drawing_algorithm);

            }
        }

        for button in buttons.iter() {
            button.render(&mut window);
        }
        polygon_builder.render(&mut window,drawing_algorithm);
        
        window.display();
    }

}
