use nalgebra::Vector2;

pub type Vector = Vector2<isize>;

pub const EAST:         Vector = Vector::new( 1,  0);
pub const NORTH_EAST:   Vector = Vector::new( 1,  1);
pub const NORTH:        Vector = Vector::new( 1, -1);
pub const NORTH_WEST:   Vector = Vector::new( 0, -1);
pub const WEST:         Vector = Vector::new(-1,  0);
pub const SOUTH_WEST:   Vector = Vector::new(-1, -1);
pub const SOUTH:        Vector = Vector::new( 0, -1);
pub const SOUTH_EAST:   Vector = Vector::new( 1, -1);

pub type Point = Vector2<usize>;

pub type Next = &dyn Fn(Point) -> 