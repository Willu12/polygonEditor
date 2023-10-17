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
        "ja chce nervosol",
        Style::CLOSE,
        &Default::default(),
    );
   // window.set_vertical_sync_enabled(true);
   let mut current_starting_point: Option<Vector2f> = None;
    let mut polygons: Vec::<Polygon> = vec![];
    //let mut polygon = Polygon {points : Vec::new()};
    let mut polygon_builder = PolygonBuilder::default();
    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                },
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
        polygon_builder.polygon.render(&mut window);

        window.display();
    }

}

