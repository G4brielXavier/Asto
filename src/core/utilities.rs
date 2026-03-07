pub struct Utilities {
    pub symbols: &'static [u8; 8],
    pub valtype: [&'static str; 3],
}

impl Utilities {
    pub fn new() -> Self {
        Self {
            symbols: b">/:${}-\"",
            valtype: ["STRING", "BOOL", "NUMBER"]
        }
    }
}