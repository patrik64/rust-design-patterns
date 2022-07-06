#[macro_use]
extern crate derive_builder;

#[allow(dead_code)]
#[derive(Default, Builder, Debug)]
struct Channel {
    token: i32,
    // opt into setter type conversion
    #[builder(setter(into))]
    special_info: i32,
    // .. a whole bunch of other fields ..
}

fn main() {
    // builder pattern, go, go, go!...
    let ch = ChannelBuilder::default()
        .token(19124)
        .special_info(42u8)
        .build()
        .unwrap();
    println!("{:?}", ch);
}