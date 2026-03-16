pub struct Utilities {
    pub symbols: &'static [u8; 7],
    pub valtype: [&'static str; 4],
    pub valstatus: [&'static str; 4],
}

impl Utilities {
    pub fn new() -> Self {
        Self {
            symbols: b">/:{}-\"",
            valtype: ["str", "bool", "int", "float"],
            valstatus: ["new", "depre", "expm", "stable"]
        }
    }
}