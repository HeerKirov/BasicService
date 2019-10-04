
pub enum RegisterMode {
    Open,
    Code,
    Close
}

pub struct GlobalSetting {
    pub id: i32,
    pub register_mode: RegisterMode,
    pub effective_max: Option<i64>,
    pub effective_default: i64
}

impl RegisterMode {
    pub fn to_string(&self) -> String {
        match self {
            Self::Open => "Open",
            Self::Code => "Code",
            Self::Close => "Close"
        }.to_string()
    }
    pub fn from_string(s: &String) -> Option<Self> {
        Self::from(&s)
    }
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "Open" => Some(Self::Open),
            "Code" => Some(Self::Code),
            "Close" => Some(Self::Close),
            _ => None
        }
    }
}