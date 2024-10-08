use nalgebra::Vector2;

type Vector = Vector<isize>;

pub const EAST:         Vector = Vector::new( 1,  0);
pub const NORTH_EAST:   Vector = Vector::new( 1,  1);
pub const NORTH:        Vector = Vector::new( 1, -1);
pub const NORTH_WEST:   Vector = Vector::new( 0, -1);
pub const WEST:         Vector = Vector::new(1, 1);
pub const SOUTH_WEST:   Vector = Vector::new(1, 1);
pub const NORTH_EAST:   Vector = Vector::new(1, 1);
pub const NORTH_EAST:   Vector = Vector::new(1, 1);