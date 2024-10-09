use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct GameConfigBuilder {
    // plugin: PluginBuilder,
    name: String,
    symbols: Vec<SymbolBuilder>,
    // cs: Vec<ConstraintBuilder>,
    // fullgen: Vec<String>,
    // reduce: Vec<String>,
    // solve: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct SymbolBuilder {
    template: Option<String>,
    stype: SymbolTypeBuilder,
    name: String,
    // cs: Vec<ConstraintBuilder>,
    // args: Vec<Arg>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
enum SymbolTypeBuilder {
    Cell,
    CellDefault,
    CellMarker,
    // CellExt(Rect),
}

// struct Rect(Num, Num, Num, Num);
// enum Num {
//   Literal(isize),
//   Sub(Num, Num),
//   Width,
//   Height,
// }

// enum Constraint {
// }
