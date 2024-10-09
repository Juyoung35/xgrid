enum Boolean {
    Literal(bool),
    And(Box<Boolean>, Box<Boolean>),
    Or(Box<Boolean>, Box<Boolean>),
    Not(Box<Boolean>),
    Xor(Box<Boolean>, Box<Boolean>),
    // Expr(Box<Expr>),
    BoolFunc(Box<BoolFunc>),
}

enum Numeral {
    Literal(isize),
    Add(Box<Numeral>, Box<Numeral>),
    Sub(Box<Numeral>, Box<Numeral>),
    Mul(Box<Numeral>, Box<Numeral>),
    Div(Box<Numeral>, Box<Numeral>),
    // Expr(Box<Expr>),
    NumFunc(Box<NumFunc>),
}

enum Spatials {
    Single(Spatial),
    Multiple(Vec<Spatial>),
}

enum Spatial {
    Static(StaticSpatial),
    Dynamic(DynamicSpatial),
}

enum StaticSpatial {
    Square(usize),
    Plus(usize),
    Cross(usize),
    Row,
    Col,
    Region,
}

enum DynamicSpatial {
    TakeWhile(Box<DynamicSpatial>, Box<Boolean>),
    WkSquare,
    WkPlus,
    WkCross,
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
}

struct Symbol;

enum BoolFunc {
    HaveEq(Spatials, Symbol, Box<Numeral>),
    PairEq(Spatials, Symbol, Box<Numeral>),
}

enum NumFunc {
    CountOf(Spatials, Symbol),
}

enum Expr {

}