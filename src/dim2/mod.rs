mod csg;
mod line_strip;
mod shapes;

pub use self::csg::Csg;
pub use self::line_strip::LineStrip;
use Unit;

#[derive(Clone, Copy)]
pub struct Point(pub Unit, pub Unit);
