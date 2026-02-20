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
    pub presence: PadstackPresenceState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PadstackPresenceState {
    Present,
    NotPresent,
    Unknown(i32),
}

impl std::fmt::Display for PadstackPresenceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Present => write!(f, "PSP_PRESENT"),
            Self::NotPresent => write!(f, "PSP_NOT_PRESENT"),
            Self::Unknown(value) => write!(f, "UNKNOWN({value})"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorRgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardStackupLayerType {
    Copper,
    Dielectric,
    Silkscreen,
    SolderMask,
    SolderPaste,
    Undefined,
    Unknown(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoardStackupDielectricProperties {
    pub epsilon_r: f64,
    pub loss_tangent: f64,
    pub material_name: String,
    pub thickness_nm: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoardStackupLayer {
    pub layer: BoardLayerInfo,
    pub user_name: String,
    pub material_name: String,
    pub enabled: bool,
    pub thickness_nm: Option<i64>,
    pub layer_type: BoardStackupLayerType,
    pub color: Option<ColorRgba>,
    pub dielectric_layers: Vec<BoardStackupDielectricProperties>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoardStackup {
    pub finish_type_name: String,
    pub impedance_controlled: bool,
    pub edge_has_connector: bool,
    pub edge_has_castellated_pads: bool,
    pub edge_has_edge_plating: bool,
    pub layers: Vec<BoardStackupLayer>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardLayerClass {
    Silkscreen,
    Copper,
    Edges,
    Courtyard,
    Fabrication,
    Other,
    Unknown(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoardLayerGraphicsDefault {
    pub layer_class: BoardLayerClass,
    pub line_thickness_nm: Option<i64>,
    pub text_font_name: Option<String>,
    pub text_size_nm: Option<Vector2Nm>,
    pub text_stroke_width_nm: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraphicsDefaults {
    pub layers: Vec<BoardLayerGraphicsDefault>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InactiveLayerDisplayMode {
    Normal,
    Dimmed,
    Hidden,
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NetColorDisplayMode {
    All,
    Ratsnest,
    Off,
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardFlipMode {
    Normal,
    FlippedX,
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RatsnestDisplayMode {
    AllLayers,
    VisibleLayers,
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrcSeverity {
    Warning,
    Error,
    Exclusion,
    Ignore,
    Info,
    Action,
    Debug,
    Undefined,
}

impl std::fmt::Display for DrcSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Exclusion => "exclusion",
            Self::Ignore => "ignore",
            Self::Info => "info",
            Self::Action => "action",
            Self::Debug => "debug",
            Self::Undefined => "undefined",
        };
        write!(f, "{value}")
    }
}

impl FromStr for DrcSeverity {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            "exclusion" => Ok(Self::Exclusion),
            "ignore" => Ok(Self::Ignore),
            "info" => Ok(Self::Info),
            "action" => Ok(Self::Action),
            "debug" => Ok(Self::Debug),
            "undefined" => Ok(Self::Undefined),
            _ => Err(format!(
                "unknown drc severity `{value}`; expected warning, error, exclusion, ignore, info, action, debug, or undefined"
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardEditorAppearanceSettings {
    pub inactive_layer_display: InactiveLayerDisplayMode,
    pub net_color_display: NetColorDisplayMode,
    pub board_flip: BoardFlipMode,
    pub ratsnest_display: RatsnestDisplayMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NetClassType {
    Explicit,
    Implicit,
    Unknown(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetClassBoardSettings {
    pub clearance_nm: Option<i64>,
    pub track_width_nm: Option<i64>,
    pub diff_pair_track_width_nm: Option<i64>,
    pub diff_pair_gap_nm: Option<i64>,
    pub diff_pair_via_gap_nm: Option<i64>,
    pub color: Option<ColorRgba>,
    pub tuning_profile: Option<String>,
    pub has_via_stack: bool,
    pub has_microvia_stack: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetClassInfo {
    pub name: String,
    pub priority: Option<i32>,
    pub class_type: NetClassType,
    pub constituents: Vec<String>,
    pub board: Option<NetClassBoardSettings>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetClassForNetEntry {
    pub net_name: String,
    pub net_class: NetClassInfo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PcbViaType {
    Through,
    BlindBuried,
    Micro,
    Blind,
    Buried,
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PcbPadType {
    Pth,
    Smd,
    EdgeConnector,
    Npth,
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PcbZoneType {
    Copper,
    Graphical,
    RuleArea,
    Teardrop,
    Unknown(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbTrack {
    pub id: Option<String>,
    pub start_nm: Option<Vector2Nm>,
    pub end_nm: Option<Vector2Nm>,
    pub width_nm: Option<i64>,
    pub layer: BoardLayerInfo,
    pub net: Option<BoardNet>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbArc {
    pub id: Option<String>,
    pub start_nm: Option<Vector2Nm>,
    pub mid_nm: Option<Vector2Nm>,
    pub end_nm: Option<Vector2Nm>,
    pub width_nm: Option<i64>,
    pub layer: BoardLayerInfo,
    pub net: Option<BoardNet>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbVia {
    pub id: Option<String>,
    pub position_nm: Option<Vector2Nm>,
    pub via_type: PcbViaType,
    pub net: Option<BoardNet>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PcbFootprint {
    pub id: Option<String>,
    pub reference: Option<String>,
    pub position_nm: Option<Vector2Nm>,
    pub orientation_deg: Option<f64>,
    pub layer: BoardLayerInfo,
    pub pad_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbPad {
    pub id: Option<String>,
    pub number: String,
    pub pad_type: PcbPadType,
    pub position_nm: Option<Vector2Nm>,
    pub net: Option<BoardNet>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbBoardGraphicShape {
    pub id: Option<String>,
    pub layer: BoardLayerInfo,
    pub net: Option<BoardNet>,
    pub geometry_kind: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbBoardText {
    pub id: Option<String>,
    pub layer: BoardLayerInfo,
    pub text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbBoardTextBox {
    pub id: Option<String>,
    pub layer: BoardLayerInfo,
    pub text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbField {
    pub name: String,
    pub visible: bool,
    pub text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbZone {
    pub id: Option<String>,
    pub name: String,
    pub zone_type: PcbZoneType,
    pub layer_count: usize,
    pub filled: bool,
    pub polygon_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbDimension {
    pub id: Option<String>,
    pub layer: BoardLayerInfo,
    pub text: Option<String>,
    pub style_kind: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbGroup {
    pub id: Option<String>,
    pub name: String,
    pub item_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PcbUnknownItem {
    pub type_url: String,
    pub raw_len: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PcbItem {
    Track(PcbTrack),
    Arc(PcbArc),
    Via(PcbVia),
    Footprint(PcbFootprint),
    Pad(PcbPad),
    BoardGraphicShape(PcbBoardGraphicShape),
    BoardText(PcbBoardText),
    BoardTextBox(PcbBoardTextBox),
    Field(PcbField),
    Zone(PcbZone),
    Dimension(PcbDimension),
    Group(PcbGroup),
    Unknown(PcbUnknownItem),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{BoardOriginKind, DrcSeverity};

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

    #[test]
    fn drc_severity_parses_known_values() {
        assert_eq!(
            DrcSeverity::from_str("warning").expect("warning should parse"),
            DrcSeverity::Warning
        );
        assert_eq!(
            DrcSeverity::from_str("error").expect("error should parse"),
            DrcSeverity::Error
        );
    }

    #[test]
    fn drc_severity_rejects_unknown_values() {
        let result = DrcSeverity::from_str("fatal");
        assert!(result.is_err());
    }
}
