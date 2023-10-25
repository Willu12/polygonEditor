use crate::algorithms::*;
use crate::point::PointIndex;
use crate::sfml::graphics::RenderTarget;
use crate::Point;
use crate::restrictions::*;

use sfml::graphics::*;
use sfml::system::Vector2f;
use geo::*;



#[derive(Default)]
pub struct Polygon<'a> {
    pub points: Vec<Point<'a>>,
    pub drag_position: Option<Vector2f>,
    pub restrictions: Vec<Restriction>,
}

#[derive(Default)]
pub struct PolygonBuilder<'a> {
    pub polygon: Polygon<'a>,
}

impl<'a> PolygonBuilder<'a> {

    pub fn build (mut self) -> Polygon<'a> {

        for point in self.polygon.points.iter_mut() {
            point.vertex.color = Color::GREEN;
        }

      //  self.reverse_if_clockwise();

        return std::mem::replace(&mut self.polygon, Polygon::default());
    }

    pub fn render(&self, window: &mut RenderWindow, algorithm_kind:DrawAlgorithm) {        
        for point in self.polygon.points.iter() {
             window.draw(&(point.shape));
         }
         self.render_lines(window,algorithm_kind);
     }



    fn reverse_if_clockwise(&mut self ) {
        if self.polygon.is_clockwise() {self.polygon.points.reverse()};
    }


     pub fn render_lines(&self, window:  &mut RenderWindow, algorithm_kind: DrawAlgorithm) {

        match algorithm_kind {
            DrawAlgorithm::Library => {
                let mut vertex_array = Vec::<Vertex>::new();
                for point in self.polygon.points.iter() {
                    vertex_array.push(point.vertex);
                }
                window.draw_primitives(&vertex_array, PrimitiveType::LINE_STRIP, &RenderStates::default())
                
            }
            DrawAlgorithm::Bresenham => {
                render_lines_bresenham_builder(&self.polygon, window);
            }
            DrawAlgorithm::Aliasing => {
                render_lines_bresenham_builder(&self.polygon, window)
            }
        }
    }
}

impl<'a> Polygon<'a> {

    pub fn render(&self, window: &mut RenderWindow, algorithm: DrawAlgorithm) {        

        self.render_border(window, algorithm);

        self.render_restriction_icons(window);
        self.render_points_indexes(window);
    }

    pub fn render_border(&self, window: &mut RenderWindow, algorithm: DrawAlgorithm) {        
        for point in self.points.iter() {
             window.draw(&(point.shape));
         }
         self.render_lines(window,algorithm);
     }



    fn render_points_indexes(&self, window: &mut RenderWindow) {

        let font = Font::from_file("fonts/Roboto-Regular.ttf").expect("Failed to load font");
        for (index,point) in self.points.iter().enumerate() {
            let mut text = Text::new(index.to_string().as_str(), &font, 16);
            text.set_fill_color(Color::WHITE);
            text.set_position(Vector2f::new(point.vertex.position.x , point.vertex.position.y));
    
            window.draw(&text);
        }
    }

    pub fn render_lines(&self, window:  &mut RenderWindow,algorithm:DrawAlgorithm ) {

        match algorithm {
            DrawAlgorithm::Library => {
                let mut vertex_array = Vec::<Vertex>::new();
                for point in self.points.iter() {
                    vertex_array.push(point.vertex);
                }
                match vertex_array.first() {
                    Some(vertex) => {vertex_array.push(vertex.clone())},
                    None => {}
                }
                window.draw_primitives(&vertex_array, PrimitiveType::LINE_STRIP, &RenderStates::default())
            },
            DrawAlgorithm::Bresenham => {
                render_lines_bresenham_polygon(&self, window);
            },
            DrawAlgorithm::Aliasing => {
                render_lines_aliasing_polygon(&self,window);
            }
        }
    }


    pub fn render_restriction_icons(&self, window: &mut RenderWindow) {
        // tutaj trzeba dodac jakies rysowanie tych restrykicji
        for restriction in self.restrictions.iter() {
            match restriction.restriction {
                RestrictionKind::Horizontal => {
                    let  texture = Texture::from_file("icons/horizontal.png").expect("Failed to load image");
                    let  mut sprite = Sprite::with_texture(&texture);
                    sprite.set_scale(Vector2f::new(0.5,0.5));
                    if let Some(mut middle_point) = self.get_middle_point(restriction.start_index,restriction.end_index) {

                       middle_point.y = middle_point.y + 5.0;
                        sprite.set_position(middle_point);
                    }
                    window.draw(&sprite);
                }
                RestrictionKind::Vertical => {
                    let  texture = Texture::from_file("icons/vertical.png").expect("Failed to load image");
                    let  mut sprite = Sprite::with_texture(&texture);
                    sprite.set_scale(Vector2f::new(0.5,0.5));
                    if let Some(mut middle_point) = self.get_middle_point(restriction.start_index,restriction.end_index) {

                       middle_point.x = middle_point.x + 5.0;
                       middle_point.y = middle_point.y - 2.0;
                        sprite.set_position(middle_point);
                    }
                    window.draw(&sprite);
                },
            }
        }
    }

    pub fn get_middle_point(&self, start: usize, end: usize) -> Option<Vector2f> {

        if let Some(start_point) = self.points.get(start) {
            if let Some(end_point) = self.points.get(end) {
                return Some(Vector2f::new((start_point.vertex.position.x + end_point.vertex.position.x)/2.0,(start_point.vertex.position.y + end_point.vertex.position.y)/2.0));
            }
        }
        return None;
    }

    pub fn remove_point(&mut self, point_index: usize) {

        self.remove_point_edges_restrictions(point_index);
        self.points.remove(point_index);
    }

    pub fn select_edge(&mut self, edge_start_index: usize, edge_end_index: usize, drag_position: Vector2f) {
       if let Some(v1) = self.points.get_mut(edge_start_index) { 
        v1.vertex.color = Color::WHITE;
       }

       if let Some(v2) = self.points.get_mut(edge_end_index) { 
        v2.vertex.color = Color::WHITE;
       }

       self.drag_position = Some(drag_position);
    }

    pub fn unselect_edge(&mut self, edge_start_index: usize, edge_end_index: usize) {
        if let Some(v1) = self.points.get_mut(edge_start_index) { 
         v1.vertex.color = Color::GREEN;
        }
 
        if let Some(v2) = self.points.get_mut(edge_end_index) { 
         v2.vertex.color = Color::GREEN;
        }

        self.drag_position = None;
    }

    pub fn move_edge(&mut self,edge_start_index: usize, edge_end_index: usize, x:f32, y:f32) {

        if let Some(last_pos) = self.drag_position {
            if let Some(v1) = self.points.get(edge_start_index) { 
                self.move_point(edge_start_index,v1.vertex.position.x + x - last_pos.x,v1.vertex.position.y + y - last_pos.y);
               }
               
               if let Some(v2) = self.points.get(edge_end_index) { 
                self.move_point(edge_end_index, v2.vertex.position.x + x - last_pos.x, v2.vertex.position.y + y - last_pos.y);
               }
               self.drag_position = Some(Vector2f::new(x,y));
        }

       // self.reverse_if_clockwise();

    }

    pub fn move_point(&mut self, point_index: usize, x:f32, y:f32) {
        let restrictions = self.get_point_restrictions(point_index);

        if let Some(point) = self.points.get_mut(point_index) {
            point.change_position(x, y);
   
        }

        for restriction in restrictions {
            let other_point_index = if restriction.start_index == point_index {restriction.end_index} else {restriction.start_index};

            if let Some(other_point) = self.points.get_mut(other_point_index) {
                match restriction.restriction {
                    RestrictionKind::Horizontal => other_point.align_horizontally(y),
                    RestrictionKind::Vertical => other_point.align_vertically(x),
                }
            }
        }
    }

    pub fn add_point_to_edge(&mut self,edge_start_index: usize, edge_end_index: usize) {

        let mut new_point = Point::new(0.0,0.0);
        if let Some(v1) = self.points.get(edge_start_index) {
            if let Some(v2) = self.points.get(edge_end_index) {
                new_point = Point::new((v1.vertex.position.x + v2.vertex.position.x)/2.0, (v1.vertex.position.y + v2.vertex.position.y)/2.0);
            }
        }
        
        self.remove_edge_restriction(edge_start_index, edge_end_index);

        self.points.insert(edge_end_index, new_point);
    }



    pub fn move_polygon(&mut self,x:f32, y:f32) {

        if let Some(last_pos) = self.drag_position {
            for point in self.points.iter_mut() {
                point.change_position(point.vertex.position.x + x - last_pos.x, point.vertex.position.y + y - last_pos.y);
            }
        }
        self.drag_position = Some(Vector2f::new(x,y));
    }

    pub fn is_point_inside(&self, point: &Vector2f) -> bool {

        let ray = Line::new(coord!{x: point.x,y : point.y}, coord!{x: point.x, y: 0.0});
        let mut counter = 0;
       for (start_point, end_point)  in  self.points.iter().zip(self.points.iter().cycle().skip(1)) {

           let segment = Line::new(coord!{x: start_point.vertex.position.x, y: start_point.vertex.position.y},
            coord!{x: end_point.vertex.position.x, y: end_point.vertex.position.y});

            match segment.intersects(&ray) {
                true => {counter += 1;},
                false => {},
            }
        }
        return counter % 2 == 1;
    }

    pub fn add_restriction(&mut self, restriction: Restriction) {

        if let Some(old_restriction) = self.get_edge_restriction(restriction.start_index, restriction.end_index) {
            if old_restriction.restriction == restriction.restriction {
                self.remove_edge_restriction(restriction.start_index, restriction.end_index);
                return;
            }
        }

        if is_restriction_possible(restriction, self) == false {return}

        self.remove_edge_restriction(restriction.start_index, restriction.end_index);

        self.restrictions.push(restriction);

        let start_point_vector = self.points.get(restriction.start_index).unwrap().vertex.position;

        //apply restriction: 
        match restriction.restriction {
            RestrictionKind::Horizontal => {
                if let Some(point) = self.points.get_mut(restriction.end_index) {
                    point.align_horizontally(start_point_vector.y);
                }
            },
            RestrictionKind::Vertical => {
                if let Some(point) = self.points.get_mut(restriction.end_index) {
                    point.align_vertically(start_point_vector.x);
                }
            }
        }        

    }

    pub fn get_point_restrictions(&self, point_index: usize) -> Vec<Restriction> {
        let mut point_restrictions: Vec<Restriction> = vec![];

        for restriction in self.restrictions.iter() {
            if restriction.start_index == point_index || restriction.end_index == point_index {
                point_restrictions.push(restriction.clone());
            }
        }
        return point_restrictions;
    }

    pub fn get_edge_restriction(&self, start_edge_index: usize, end_edge_index: usize) -> Option<Restriction> {

        for restriction in self.restrictions.iter() {
            if restriction.start_index == start_edge_index || restriction.end_index == end_edge_index {
                return Some(restriction.clone());
            }
        }
        return None;
    }

    pub fn remove_edge_restriction(&mut self, start_edge_index: usize, end_edge_index: usize) {
        if let Some(restriction) = self.get_edge_restriction(start_edge_index, end_edge_index) {
            if let Some(restriction_index) = self.restrictions.iter().position(|x| *x == restriction) {
                self.restrictions.remove(restriction_index);
                return;
            }
        }
    }

    fn remove_point_edges_restrictions(&mut self, point_index:usize) {
        
        let previous_point_index = if point_index == 0 {self.points.len() - 1} else {point_index + 1 };
        let next_point_index = (point_index + 1) % self.points.len();

        self.remove_edge_restriction(previous_point_index, point_index);
        self.remove_edge_restriction(point_index, next_point_index);
    }
    pub fn is_clockwise(&self) -> bool {
        let mut sum: f32 = 0.0;
        for (start_point, end_point)  in  self.points.iter().zip(self.points.iter().cycle().skip(1)) {
            sum = sum + (end_point.vertex.position.x - start_point.vertex.position.x) * (end_point.vertex.position.y + start_point.vertex.position.y);
        }
        return sum < 0.0;
    }

}

pub fn create_polygon_from_vertices<'a>(vertices: Vec<Vector2f>) -> Polygon<'a> {

    let mut polygon_builder = PolygonBuilder::default();

    for v in vertices {
        polygon_builder.polygon.points.push(Point::new(v.x,v.y));
    }

    return polygon_builder.build();
}

pub fn find_point_index (x: f32, y:f32, polygons:& Vec<Polygon>) -> Option<PointIndex> {
    for (polygon_index, polygon) in polygons.iter().enumerate() {
        for (point_index, point) in polygon.points.iter().enumerate() {
            if point.intersects(Vector2f::new(x,y)) {return Some(PointIndex { polygon_index: polygon_index, point_index: point_index })} 
        }
    }
    return  None;
}

pub fn find_edge (x: f32, y:f32, polygons:& Vec<Polygon>) -> Option<(PointIndex,PointIndex)> {
    for (polygon_index, polygon) in polygons.iter().enumerate() {
        
        for (first, second) in polygon.points.iter().enumerate().zip(polygon.points.iter().cycle().skip(1).enumerate()) {
                        
            if check_edge_intersection(first.1.vertex.position, second.1.vertex.position, Vector2f::new(x,y))  {
                return Some((PointIndex::new(polygon_index,first.0),PointIndex::new(polygon_index,(second.0 + 1)%polygon.points.len())));
            }
        }
    }
    return  None;
}

pub fn find_polygon(x: f32, y:f32, polygons:& Vec<Polygon>) -> Option<usize> {
    for(polygon_index,polygon) in polygons.iter().enumerate() {
        if polygon.is_point_inside(&Vector2f::new(x,y)) {return Some(polygon_index)};
    }
    return None;
}

fn check_edge_intersection(edge_start: Vector2f,edge_end: Vector2f, v: Vector2f) -> bool {
    let z = 1.0;

    let edge_length = ((edge_start.x - edge_end.x).powi(2) + (edge_start.y - edge_end.y).powi(2)).sqrt();
    let start_v_length = ((edge_start.x - v.x).powi(2) + (edge_start.y - v.y).powi(2)).sqrt();
    let end_v_length = ((v.x - edge_end.x).powi(2) + (v.y - edge_end.y).powi(2)).sqrt();

    
    if edge_length  >= start_v_length + end_v_length - z {return  true};
    return false
}