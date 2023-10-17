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
    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                },
                Event::MouseMoved { x, y } => {
                    match find_point_index(x as f32, y as f32, &polygons) {
                        Some(pointIndex) => {
                            
                            
                            
                        }
                        None => { },
                    }
                }
                Event::MouseButtonReleased { button: _, x, y } => {
                    let point = Point::new(x as f32,y as f32);

                    match current_starting_point {
                        Some(p) => {
                            if point.intersects(p)   {
                                polygon_builder.polygon.points.push(Point::new(p.x, p.y));
                                polygons.push(polygon_builder.build());
                                polygon_builder = PolygonBuilder::default();
                                current_starting_point = None;
                            } 
                            else {polygon_builder.polygon.points.push(point)}
                        },
                        None => {
                            //jezeli nie jestesmy w trakcie budowania wielokata to powinnismy sprawdzic czy nie nie chcemy wybrac
                            //istniejacego juz punktu, by go zaznaczyc
                            
                           // let p = find_point(x as f32, y as f32, &mut polygons);
                            match find_point_index(x as f32, y as f32,&polygons) {
                                Some(p_index) => {
                                    match selected_point_index {
                                        Some(index) => {
                                            polygons.get_mut(index.polygon_index).unwrap().points
                                            .get_mut(index.point_index).unwrap().unselect();
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
                                            current_starting_point = Some(Vector2f::new(x as f32, y as f32));
                                            polygon_builder.polygon.points.push(point);
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
        polygon_builder.polygon.render(&mut window);

        window.display();
    }

}
