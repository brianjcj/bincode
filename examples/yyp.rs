#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

use serde::ser;
use serde::ser::SerializeTupleStruct;
use bincode::{DefaultOptions, Options};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MyMsg {
    n: u32,
    m: u32,
    x: u16,
    y: u8,
}

#[derive(Deserialize, PartialEq, Debug)]
struct WithLen32<T: ser::Serialize> {
    len: u32,
    inner: T,
}

impl<T: ser::Serialize> ser::Serialize for WithLen32<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // it works any way
        let oo = DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .with_yyp_len_width();
        let len = oo.serialized_size(&self.inner).expect("len") as u32;

        let mut ts = serializer.serialize_tuple_struct("WithLen32", 2)?;
        ts.serialize_field(&len)?;
        ts.serialize_field(&self.inner)?;
        ts.end()
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    s: String,
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let world = World(vec![
        Entity {
            s: "ab".to_string(),
            x: 1,
            y: 2,
        },
        Entity {
            s: "b".to_string(),
            x: 3,
            y: 4,
        },
    ]);

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

    println!("==========test with len32=======");

    {
        let msg = MyMsg { n: 1, m: 2, x: 3, y: 4 };
        let encoded: Vec<u8> = oo.serialize(&msg).unwrap();
        println!("msg: encoded: {:?}", encoded);

        let msg_with_len32 = WithLen32 {len: 0, inner: msg};
        let encoded: Vec<u8> = oo.serialize(&msg_with_len32).unwrap();
        println!("msg_with_len32: encoded: {:?}", encoded);

        let decoded: WithLen32<MyMsg> = oo.deserialize(&encoded[..]).unwrap();
        println!("decoded: {:?}", decoded);
    }


}
