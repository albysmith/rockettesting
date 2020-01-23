#[derive(Deserialize, Debug, Clone)]
pub struct AtlasImport {
    pub maps: Vec<MapImport>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MapImport {
    pub name: Option<String>,
    pub regionname: Option<String>,
    pub tier0: Option<i32>,
    pub tier1: Option<i32>,
    pub tier2: Option<i32>,
    pub tier3: Option<i32>,
    pub tier4: Option<i32>,
    pub watchstones_0: Option<Vec<String>>,
    pub watchstones_1: Option<Vec<String>>,
    pub watchstones_2: Option<Vec<String>>,
    pub watchstones_3: Option<Vec<String>>,
    pub watchstones_4: Option<Vec<String>>,
}

impl AtlasImport {
    pub fn new() -> AtlasImport {
        AtlasImport { maps: vec![] }
    }
}
