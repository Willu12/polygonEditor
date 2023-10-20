use crate::polygon::Polygon;

#[derive(Debug, PartialEq, Eq,Clone,Copy)]
pub enum RestrictionKind {
    Horizontal,
    Vertical,
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Restriction {
    pub start_index: usize,
    pub end_index: usize,
    pub restriction: RestrictionKind
}

pub fn is_restriction_possible(restriction: Restriction, polygon: &Polygon) -> bool {
    for existing_restriction in polygon.restrictions.iter() {
        if restriction.restriction == existing_restriction.restriction {
            if restriction.start_index == existing_restriction.start_index || restriction.start_index == existing_restriction.end_index
               || restriction.end_index == existing_restriction.start_index || restriction.end_index == existing_restriction.end_index {
                return false;
               }
        }
    }
    return true;
}


