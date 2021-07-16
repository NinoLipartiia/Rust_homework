pub enum ThreeDimFigure {
    Sphere { r :f32 },
    Parallelepiped { x :f32, y :f32, z :f32}, 
}
    
impl ThreeDimFigure {

    pub fn fig_volume(&self) -> f32 {
        match self {
            ThreeDimFigure::Sphere{ r } => std::f32::consts::PI*r*r*r / 3.0 * 4.0,
            ThreeDimFigure::Parallelepiped{x, y, z} => x*y*z,
        }
    }
}

