use crate::Geometry;
pub struct Report {
    pub area: Geometry,
    pub hit: Geometry,
}

impl Report {
    pub fn new() -> Self {
        return Report{area: Geometry::new(), hit: Geometry::new()}
    }
}