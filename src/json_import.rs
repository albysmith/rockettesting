use crate::atlas::*;
use std::fs;

pub fn import_complete_atlas() -> AtlasImport {
    // Read JSON to pull atlas data
    let atlas: AtlasImport =
        serde_json::from_str(&fs::read_to_string("static/database.json").expect("ERROR"))
            .expect("ERROR");
    atlas
}

// pub fn complete_atlas_json(atlas: &mut AtlasImport) -> &mut AtlasImport {
// }
