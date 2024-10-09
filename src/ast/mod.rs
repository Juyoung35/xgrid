mod builders;
mod num;

struct GameConfig {
    name: String,
    symbols: Vec<Symbol>,
}

struct Symbol {
    stype: SymbolType,
    name: String,
}

enum SymbolType {
    Cell,
    CellDefault,
    CellMarker,
}