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
