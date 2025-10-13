use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nation {
    Tera,
    Sora,
    Aqua,
}

impl Nation {
    pub fn name(&self) -> &'static str {
        match self {
            Nation::Tera => "테라",
            Nation::Sora => "소라",
            Nation::Aqua => "아쿠아",
        }
    }
}
