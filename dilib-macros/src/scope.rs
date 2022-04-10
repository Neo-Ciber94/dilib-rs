use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Scope {
    Singleton,
    Scoped,
}

impl FromStr for Scope {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "singleton" => Ok(Scope::Singleton),
            "scoped" => Ok(Scope::Scoped),
            _ => Err(format!(
                "Invalid scope value: {}, expected 'singleton' or 'scoped'",
                s
            )),
        }
    }
}
