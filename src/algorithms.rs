use sfml::system::{Vector2i,Vector2f};
use sfml::graphics::{*, Transformable};

use crate::polygon::Polygon;

#[derive(Debug, PartialEq, Eq,Clone,Copy)]
pub enum DrawAlgorithm {
    Library,
    Bresenham,
    Aliasing,
}

pub struct AlgorithmButton {
    pub position: Vector2f,
    pub active: bool,
    pub radius: f32,
    pub algorithm: DrawAlgorithm,
}

impl AlgorithmButton {
    pub fn new(position: Vector2f, algorithm: DrawAlgorithm) -> AlgorithmButton {
        AlgorithmButton { position: position, active: false, radius: 6.0, algorithm: algorithm }
    }

    pub fn render(&self, window:  &mut RenderWindow) {

        let font = Font::from_file("fonts/Roboto-Regular.ttf").expect("Failed to load font");
        let mut circle =  CircleShape::default();
        circle.set_radius(self.radius);
       

        match self.active {  
            true => { circle.set_fill_color(Color::GREEN)},
            false => {circle.set_fill_color(Color::WHITE)},
        }
        let text_string = match self.algorithm {
            DrawAlgorithm::Aliasing => {"Aliasing"},
            DrawAlgorithm::Bresenham => {"Bresenham"},
            DrawAlgorithm::Library => {"Library"}
        };

        let mut text = Text::new(text_string, &font, 16);
        text.set_fill_color(Color::WHITE);
        text.set_position(Vector2f::new(self.position.x + self.radius + 10.0, self.position.y - (text.character_size()/2) as f32));

        circle.set_position(Vector2f::new(self.position.x - circle.radius()/2.0, self.position.y - circle.radius()/2.0));
        window.draw(&circle);
        window.draw(&text);

    }
}

pub fn render_lines_bresenham_builder(polygon: &Polygon ,window: &mut RenderWindow) {
    
    let mut bresenham_points: Vec<Vector2i> = vec![];

    let mut vertex_array = Vec::<Vertex>::new();

    for (first, second) in polygon.points.iter().zip(polygon.points.iter().skip(1)) { 
        let start_pos = first.vertex.position;
        let end_pos = second.vertex.position;
        let mut new_points = calculate_bresenham(
            Vector2i::new(start_pos.x as i32, start_pos.y as i32),
              Vector2i::new(end_pos.x as i32, end_pos.y as i32));

        bresenham_points.append(&mut new_points);        
    }

    for point in bresenham_points {
        let vertex = Vertex::new(Vector2f::new(point.x as f32,point.y as f32),Color::RED,Vector2f::default());
        vertex_array.push(vertex);
    }
    window.draw_primitives(&vertex_array, PrimitiveType::POINTS, &RenderStates::default())
}

pub fn render_lines_bresenham_polygon(polygon: &Polygon ,window: &mut RenderWindow) {
    
    let mut bresenham_points: Vec<Vector2i> = vec![];

    let mut vertex_array = Vec::<Vertex>::new();

    for (first, second) in polygon.points.iter().zip(polygon.points.iter().cycle().skip(1)) { 
        let start_pos = first.vertex.position;
        let end_pos = second.vertex.position;
        let mut new_points = calculate_bresenham(
            Vector2i::new(start_pos.x as i32, start_pos.y as i32),
              Vector2i::new(end_pos.x as i32, end_pos.y as i32));

        bresenham_points.append(&mut new_points);        
    }

    for point in bresenham_points {
        let vertex = Vertex::new(Vector2f::new(point.x as f32,point.y as f32),Color::GREEN,Vector2f::default());
        vertex_array.push(vertex);
    }
    window.draw_primitives(&vertex_array, PrimitiveType::POINTS, &RenderStates::default())
}


pub fn calculate_bresenham(start:Vector2i, end:Vector2i) -> Vec<Vector2i> {

    let mut calculated_points: Vec<Vector2i> = vec![];

    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let g = if dx > 0 {1} else {-1};
    let h = if dy > 0 {1} else {-1};

    let dx = dx.abs();
    let dy = dy.abs();

    let mut x = start.x;
    let mut y = start.y;

    if dx > dy {
        let mut c = -dx;

        while x != end.x {
            calculated_points.push(Vector2i::new(x,y));
            c = c + 2 * dy;

            if c > 0 {
                y = y + h;
                c = c - 2 * dx;
            }
            x = x + g;
        }
    }
    else {
        let mut c = -dy;
        while y != end.y {
            calculated_points.push(Vector2i::new(x,y));

            c = c + 2 * dx;

            if c > 0 {
                x = x + g;
                c = c - 2 * dy;
            }
            y = y + h;
        }

    }
    return  calculated_points;
}

pub fn render_line_aliasing(start: Vector2f, end:Vector2f,window:  &mut RenderWindow) {

    let mut x0 = start.x;
    let mut y0 = start.y;
    let mut y1 = end.y;
    let mut x1 = end.x;

    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    if  steep  {(x0,y0) = (y0,x0); (x1,y1) = (y1,x1)}

    if x0 > x1 {(x0,x1) = (x1,x0); (y0,y1) = (y1,y0)}


    let dx = x1 - x0;
    let dy = y1 - y0;

    let gradient = if dx == 0.0 {1.0} else {dy as f32/dx as f32};

    //
    let xend = x0.round();
    let yend = y0 + gradient * (xend -x0);
    let xpxl1 = xend; // this will be used in the main loop

    let mut intery = yend + gradient;


    let xend = x1.round();
    let xpxl2 = xend ;//this will be used in the main loop

    if steep {

        for x in (xpxl1 + 1.0).floor() as i32 .. (xpxl2 - 1.0).floor() as i32 {

            plot(intery.floor() as i32,x, 1.0 - (intery).fract(), window);
            plot(1 + intery.floor() as i32,x, (intery).fract(), window);
            
            intery = intery + gradient;

        }
    }
    else {
        for x in (xpxl1 + 1.0).floor() as i32 .. (xpxl2 - 1.0).floor() as i32 {

            plot(x,intery.floor() as i32, 1.0 - (intery).fract(), window);
            plot(x, 1 + intery.floor() as i32, (intery).fract(), window);
            intery = intery + gradient;
        }
    }

    //

}

pub fn plot(x: i32, y: i32, c:f32, window:  &mut RenderWindow) {
    //plot pixel with birghtenss

    let mut v = Vertex { position: Vector2f::new(x as f32 ,y as f32), color: Color::GREEN, tex_coords : Vector2f::new(0.0,0.0)  };

    v.color.a = (255.0 * c).floor() as u8;

    //println!("{}",c);

    let vectorArray = vec![v];

    window.draw_primitives(&vectorArray, PrimitiveType::POINTS, &RenderStates::default());

}
pub fn render_lines_aliasing_polygon(polygon: &Polygon,window: &mut RenderWindow) {
    for (first, second) in polygon.points.iter().zip(polygon.points.iter().cycle().skip(1)) { 
        let start_pos = first.vertex.position;
        let end_pos = second.vertex.position;
        render_line_aliasing(start_pos, end_pos, window);
    }
}
