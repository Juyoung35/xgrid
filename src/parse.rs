#[derive(Deserialize, Serialize, Clone, Debug)]
struct GameConfigBuilder {
  name: String,
  symbols: Vec<SymbolBuilder>,
  cs: Vec<Constraint>,
  // fullgen: Vec<String>,
  // reduce: Vec<String>,
  // solve: Vec<String>,
}

struct SymbolBuilder {
  type: SymbolTypeBuilder,
  name: String,
  args: Vec<Arg>,
  template: Option<String>,
}

enum SymbolTypeBuilder {
  Cell,
  CellDefault,
  CellMarker,
  CellExt(Rect),
}

struct Rect(Num, Num, Num, Num);
enum Num {
  Literal(isize),
  Sub(Num, Num),
  Width,
  Height,
}

enum Constraint {
}
