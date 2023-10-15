extern crate sfml;

mod point;
mod polygon;
use sfml::graphics::*;
use sfml::window::*;
//use sfml::system::*;
use crate::point::*;
use crate::polygon::Polygon;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Akademia pana kleksa",
        Style::CLOSE,
        &Default::default(),
    );
   // window.set_vertical_sync_enabled(true);
    let mut polygon = Polygon { points : Vec::new()};
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
                    polygon.points.push(point);
                },
                _ => {},
            }
        }

        window.clear(Color::BLACK);
        polygon.render(&mut window);
        window.display();
    }

}

