use crate::polygon::create_polygon_from_vertices;
use crate::restrictions::{RestrictionKind, Restriction};
use crate::{Polygon, restrictions};
use std::{fs::*, io::Write};
use std::io::*;
use sfml::system::Vector2f;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


pub fn save_polygons(polygons : &Vec<Polygon>) {
    let filename = "zapis";
   // remove_file(filename).;
    let mut file = LineWriter::new(File::create(filename).expect("File already exists"));




    for polygon in polygons.iter() {
        file.write("vertices:\n".as_bytes());

        for p in polygon.points.iter() {

            file.write((p.vertex.position.x.to_string() + "_" + &p.vertex.position.y.to_string() + "\n" ).as_bytes());

        }
        file.write("stop\n".as_bytes());
        if polygon.restrictions.len() > 0 {
            file.write("restrictions\n".as_bytes());

            for restriction in polygon.restrictions.iter() {
                
                let restriction_name = if restriction.restriction == RestrictionKind::Horizontal {"horizontal"} else {"vertical"};

                file.write((restriction.start_index.to_string() +"_" + &restriction.end_index.to_string() + "_" + &restriction_name +"\n").as_bytes());
            }
            file.write("stop_restrictions\n".as_bytes());
        }
        file.write("end\n".as_bytes());
    }
}

pub fn load_polygons<'a>() -> Vec<Polygon<'a>> {

   let mut polygons : Vec<Polygon> = vec![];


   let strings = read_lines("zapis");

    let mut creating_polygon = false;
    let mut adding_restriction = false;

    let mut polygon: Polygon = Polygon::default();

    let mut current_polygon_vertices:Vec<Vector2f> = vec![];
    let mut current_restricitons: Vec<Restriction> = vec![];

   for s in strings {

        //println!("{}",s);
        if s == "vertices:" {creating_polygon = true; current_polygon_vertices = vec![]; continue;};
        if s == "stop" {polygon = create_polygon_from_vertices(current_polygon_vertices.clone());creating_polygon = false};
        if s == "end" {polygons.push(polygon); polygon = Polygon::default()}
        if s== "restrictions" {adding_restriction = true; current_restricitons = vec![]; continue; }
        if s == "stop_restrictions" {adding_restriction = false; continue;}

        if(creating_polygon == true) {
            let mid_point = s.find("_").unwrap();
            let x = &s[0..mid_point];
            let y = &s[mid_point + 1 ..s.len()];

            let x_int = x.parse::<i32>().unwrap();
            let y_int =  y.parse::<i32>().unwrap();
            let v = Vector2f::new(x_int as f32, y_int as f32);
            current_polygon_vertices.push(v);
        }
        
        if adding_restriction {
            let split: Vec<&str> = s.split('_').collect();

            let start_index: i32 = split[0].parse::<i32>().unwrap();
            let end_index: i32 = split[1].parse::<i32>().unwrap();

            let restrictionKind = if split[2] == "horizontal" {RestrictionKind::Horizontal} else {RestrictionKind::Vertical};
            
            polygon.add_restriction(Restriction {start_index : start_index as usize, end_index : end_index as usize, restriction : restrictionKind});
        }


   }



   return polygons;
}