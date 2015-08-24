#![feature(plugin)]
#![plugin(layout_id)]

struct Data {
    a: u32,
    b: u32,
}

#[test]
fn test() {
    println!(layout_id!(Data));

    assert!(false);
}