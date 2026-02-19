use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardNet {
    pub code: i32,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardLayerInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardEnabledLayers {
    pub copper_layer_count: u32,
    pub layers: Vec<BoardLayerInfo>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardOriginKind {
    Grid,
    Drill,
}

impl FromStr for BoardOriginKind {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "grid" => Ok(Self::Grid),
            "drill" => Ok(Self::Drill),
            _ => Err(format!(
                "unknown board origin kind `{value}`; expected `grid` or `drill`"
            )),
        }
    }
}

impl std::fmt::Display for BoardOriginKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grid => write!(f, "grid"),
            Self::Drill => write!(f, "drill"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vector2Nm {
    pub x_nm: i64,
    pub y_nm: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PadNetEntry {
    pub footprint_reference: Option<String>,
    pub footprint_id: Option<String>,
    pub pad_id: Option<String>,
    pub pad_number: String,
    pub net_code: Option<i32>,
    pub net_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::BoardOriginKind;

    #[test]
    fn board_origin_kind_parses_known_values() {
        assert_eq!(
            BoardOriginKind::from_str("grid").expect("grid should parse"),
            BoardOriginKind::Grid
        );
        assert_eq!(
            BoardOriginKind::from_str("drill").expect("drill should parse"),
            BoardOriginKind::Drill
        );
    }

    #[test]
    fn board_origin_kind_rejects_unknown_values() {
        let result = BoardOriginKind::from_str("other");
        assert!(result.is_err());
    }
}
