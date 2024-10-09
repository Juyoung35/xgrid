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
    TakeWhile(Box<DynamicSpatial>, Boolean),
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
    HaveEq(Spatials, Symbol, Numeral),
    PairEq(Spatials, Symbol, Numeral),
}

enum NumFunc {
    CountOf(Spatial, Symbol),
}

enum Expr {

}