extern crate sfml;

mod point;
mod polygon;
use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;
//use sfml::system::*;
use crate::point::*;
use crate::polygon::*;

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
    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                },
                Event::KeyReleased { code, alt:_t, ctrl:_, shift:_, system:_ } => {
                    match code {
                        Key::D => {
                            match selected_point_index {
                                Some(index) => polygons.get_mut(index.polygon_index).unwrap().remove_point(index.point_index),
                                None => {},
                            }
                            selected_point_index = None;
                        }
                        _ => {},
                    }
                }
                Event::MouseMoved { x, y } => {

                    if let Some(index) = selected_point_index {
                        polygons.get_mut(index.polygon_index).unwrap().points.get_mut(index.point_index).unwrap()
                        .change_position(x as f32, y as f32);
                    }

                    if let Some((edge_start_index,edge_end_index)) = selected_edge {
                        polygons.get_mut(edge_start_index.polygon_index).unwrap().move_edge(edge_start_index.point_index, edge_end_index.point_index, x as f32, y as f32)
                    }
                },
                Event::MouseButtonReleased { button: _, x, y } => {
                    let point = Point::new(x as f32,y as f32);
                    match current_starting_point {
                        Some(p) => {
                            if point.intersects(p)   {
                                //polygon_builder.polygon.points.push(Point::new(p.x, p.y));
                                polygons.push(polygon_builder.build());
                                polygon_builder = PolygonBuilder::default();
                                current_starting_point = None;
                            } 
                            else {polygon_builder.polygon.points.push(point)}
                        },
                        None => {
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

                                            //sprawdzenie jeszcze czy moze przypadkiem nie krawedz
                                            match find_edge(x as f32, y as f32, &polygons) {
                                                Some((start_edge, end_edge)) => {
                                                    
                                                    match selected_edge {
                                                        Some((old_edge_start, old_edge_end)) => {
                                                            if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                                                                polygon.unselect_edge(old_edge_start.point_index, old_edge_end.point_index)
                                                            }
                                                            if (start_edge,end_edge) == (old_edge_start,old_edge_end) {selected_edge = None; continue;}
                                                        },
                                                        None => {},
                                                    }
                                                    selected_edge = Some((start_edge,end_edge));
                                                    if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                                                        polygon.select_edge(start_edge.point_index, end_edge.point_index,Vector2f::new(x as f32,y as f32))
                                                    }
                                                },
                                                None => {
                                                    
                                                    match selected_edge {
                                                        Some((start_edge, end_edge)) => {
                                                            if let Some(polygon) = polygons.get_mut(start_edge.polygon_index)  {
                                                                polygon.unselect_edge(start_edge.point_index, end_edge.point_index)
                                                            }
                                                            selected_edge = None;
                                                        },
                                                        None => {
                                                            current_starting_point = Some(Vector2f::new(x as f32, y as f32));
                                                            polygon_builder.polygon.points.push(point);
                                                        },
                                                    }

                                                    
                                                },
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
