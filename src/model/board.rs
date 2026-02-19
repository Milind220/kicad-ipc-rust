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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArcStartMidEndNm {
    pub start: Vector2Nm,
    pub mid: Vector2Nm,
    pub end: Vector2Nm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolyLineNodeGeometryNm {
    Point(Vector2Nm),
    Arc(ArcStartMidEndNm),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolyLineNm {
    pub nodes: Vec<PolyLineNodeGeometryNm>,
    pub closed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolygonWithHolesNm {
    pub outline: Option<PolyLineNm>,
    pub holes: Vec<PolyLineNm>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PadShapeAsPolygonEntry {
    pub pad_id: String,
    pub layer_id: i32,
    pub layer_name: String,
    pub polygon: PolygonWithHolesNm,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PadstackPresenceEntry {
    pub item_id: String,
    pub layer_id: i32,
    pub layer_name: String,
    pub presence: String,
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
