
#[derive(Debug, Clone, Copy)]
pub enum Scope {
    Singleton,
    Scoped,
}

impl Scope {
    pub fn from_str(s: &str) -> Self {
        match s {
            "singleton" => Scope::Singleton,
            "scoped" => Scope::Scoped,
            _ => panic!(
                "Invalid scope value: {}, expected 'singleton' or 'scoped'",
                s
            ),
        }
    }
}
