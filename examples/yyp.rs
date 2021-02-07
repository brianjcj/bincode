#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{DefaultOptions, Options};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    s: String,
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let world = World(vec![Entity { s: "ab".to_string(),  x: 1, y: 2 }, Entity { s: "b".to_string(), x: 3, y: 4 }]);


    let oo = DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_yyp_len_width();

    let encoded: Vec<u8> = oo.serialize(&world).unwrap();

    println!("encoded: {:?}", encoded);

    // 8 bytes for the length of the vector (usize), 4 bytes per float.
    // assert_eq!(encoded.len(), 8 + 4 * 4);

    let decoded: World = oo.deserialize(&encoded[..]).unwrap();
    println!("decoded: {:?}", decoded);

    assert_eq!(world, decoded);
}
