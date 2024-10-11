use std::io::Cursor;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Foo {
    a: u32,
    b: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Bar {
    a: String,
    b: u32,
}

fn main() {
    let foo = Foo {
        a: 69,
        b: "foo".to_string(),
    };
    let bar = Bar {
        a: "bar".to_string(),
        b: 420,
    };

    let mut buf = Vec::new();

    foo.serialize(&mut buf).unwrap();
    println!("buf [serialized: foo]: {buf:?}");
    bar.serialize(&mut buf).unwrap();
    println!("buf [serialized: foo, bar]: {buf:?}");

    let mut cursor = Cursor::new(buf);

    let foo_deser = Foo::deserialize_reader(&mut cursor).unwrap();
    println!("foo (deserialized): {foo_deser:?}");
    println!("bar [serialized: bar] [deserialized: foo]: {cursor:?}");
    let bar_deser = Bar::deserialize_reader(&mut cursor).unwrap();
    println!("bar (deserialized): {bar_deser:?}");
    println!("bar [deserialized: foo, bar]: {cursor:?}");
}
