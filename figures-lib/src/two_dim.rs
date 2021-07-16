pub enum TwoDimFigure {
    Circle {r :f32 }, 
    Rectangular { length :f32, width :f32 },
}

impl TwoDimFigure {

    pub fn fig_area(&self) -> f32  {
        match self {
            TwoDimFigure::Circle{ r } => std::f32::consts::PI*r*r,
            TwoDimFigure::Rectangular{ length, width} => length*width,
        }
    }
}
