mod builders;
mod num;

pub use num::Num;
use std::collections::HashMap;

pub struct GameBoard {
    pub config: GameConfig,
    pub width: usize,
    pub height: usize,
}

struct GameConfig {
    name: &'static str,
    symbols: HashMap<&'static str, Symbol>,
}

struct Symbol {
    stype: SymbolType,
    name: &'static str,
    // args: Vec<Num>,
}

enum SymbolType {
    Cell,
    CellDefault,
    CellMarker,
}

struct Rect {
    x0: Num,
    y0: Num,
    x1: Num,
    y1: Num,
}

enum Expr {
    Bool,
    Int,
}