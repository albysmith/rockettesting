use crate::atlas::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AtlasDisplay {
    pub maps: Vec<MapDisplay>,
}

impl AtlasDisplay {
    pub fn new() -> AtlasDisplay {
        AtlasDisplay { maps: vec![] }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MapDisplay {
    pub name: String,
    pub regionname: String,
    pub tier0: i32,
    pub tier1: i32,
    pub tier2: i32,
    pub tier3: i32,
    pub tier4: i32,
    // pub watchstones_0: String,
    // pub watchstones_1: String,
    // pub watchstones_2: String,
    // pub watchstones_3: String,
    // pub watchstones_4: String,
    pub watchstones_0: Vec<String>,
    pub watchstones_1: Vec<String>,
    pub watchstones_2: Vec<String>,
    pub watchstones_3: Vec<String>,
    pub watchstones_4: Vec<String>,
}

impl MapDisplay {
    pub fn convert(map: MapImport) -> MapDisplay {
        let mut newmap = MapDisplay {
            name: map.name.unwrap_or_default(),
            regionname: map.regionname.unwrap_or_default(),
            tier0: map.tier0.unwrap_or_default(),
            tier1: map.tier1.unwrap_or_default(),
            tier2: map.tier2.unwrap_or_default(),
            tier3: map.tier3.unwrap_or_default(),
            tier4: map.tier4.unwrap_or_default(),
            // watchstones_0: "".to_string(),
            // watchstones_1: "".to_string(),
            // watchstones_2: "".to_string(),
            // watchstones_3: "".to_string(),
            // watchstones_4: "".to_string(),
            watchstones_0: map.watchstones_0.unwrap_or_default(),
            watchstones_1: map.watchstones_1.unwrap_or_default(),
            watchstones_2: map.watchstones_2.unwrap_or_default(),
            watchstones_3: map.watchstones_3.unwrap_or_default(),
            watchstones_4: map.watchstones_4.unwrap_or_default(),
        };
        // for conn in map.watchstones_0.unwrap_or_default() {
        //     newmap.watchstones_0.push_str("  ");
        //     newmap.watchstones_0.push_str(&conn)
        // }
        // for conn in map.watchstones_1.unwrap_or_default() {
        //     newmap.watchstones_1.push_str("  ");
        //     newmap.watchstones_1.push_str(&conn)
        // }
        // for conn in map.watchstones_2.unwrap_or_default() {
        //     newmap.watchstones_2.push_str("  ");
        //     newmap.watchstones_2.push_str(&conn)
        // }
        // for conn in map.watchstones_3.unwrap_or_default() {
        //     newmap.watchstones_3.push_str("  ");
        //     newmap.watchstones_3.push_str(&conn)
        // }
        // for conn in map.watchstones_4.unwrap_or_default() {
        //     newmap.watchstones_4.push_str("  ");
        //     newmap.watchstones_4.push_str(&conn)
        // }
        newmap
    }
}
