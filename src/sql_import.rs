use crate::atlas::*;
use postgres::{Client, NoTls};

pub fn import_complete_atlas_sql() -> AtlasImport {
    let mut atlas = AtlasImport::new();
    // Use SQL queries to pull map info
    complete_atlas_query(&mut atlas);
    atlas
}

pub fn complete_atlas_query(atlas: &mut AtlasImport) -> &mut AtlasImport {
    let mut client = Client::connect("host=localhost user=postgres", NoTls).expect("ERROR");

    for row in client
        .query(
            "SELECT * FROM rustatlas ORDER BY rustatlas.region, rustatlas.name",
            &[],
        )
        .expect("ERROR")
    {
        let map: MapImport = MapImport {
            name: row.get(0),
            regionname: row.get(1),
            tier0: row.get(2),
            tier1: row.get(3),
            tier2: row.get(4),
            tier3: row.get(5),
            tier4: row.get(6),
            watchstones_0: row.get(7),
            watchstones_1: row.get(8),
            watchstones_2: row.get(9),
            watchstones_3: row.get(10),
            watchstones_4: row.get(11),
        };
        // println!("{:#?}", map);
        atlas.maps.push(map);
    }
    atlas
}
