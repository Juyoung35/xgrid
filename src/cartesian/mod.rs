mod macros;

use crate::vec_concat;
use nalgebra::Vector2;

pub type Vector = Vector2<isize>;


pub const ORIGIN:       Vector = Vector::new( 0,  0);

pub const EAST:         Vector = Vector::new( 1,  0);
pub const NORTH_EAST:   Vector = Vector::new( 1,  1);
pub const NORTH:        Vector = Vector::new( 1, -1);
pub const NORTH_WEST:   Vector = Vector::new( 0, -1);
pub const WEST:         Vector = Vector::new(-1,  0);
pub const SOUTH_WEST:   Vector = Vector::new(-1, -1);
pub const SOUTH:        Vector = Vector::new( 0, -1);
pub const SOUTH_EAST:   Vector = Vector::new( 1, -1);


pub type Vectors = &'static [Vector];


pub const ORTHOGONAL: Vectors = &[
    EAST, NORTH, WEST, SOUTH,
];
pub const DIAGONAL: Vectors = &[
    NORTH_EAST, NORTH_WEST, SOUTH_WEST, SOUTH_EAST,
];
pub const SQUARE: Vectors = vec_concat!(
    ORTHOGONAL, DIAGONAL,
);
pub const FULL_SQUARE: Vectors = vec_concat!(
    [ORIGIN], SQUARE,
);

pub type Point = Vector2<usize>;