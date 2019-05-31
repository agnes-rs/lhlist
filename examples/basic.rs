#[macro_use] extern crate lhlist;

use lhlist::Label;

#[label(name="My Label!", type=u16)]
struct MyLabel;

fn main() {
    println!("{}", MyLabel::name());
    println!("{}", <MyLabel as Label>::AssocType::max_value());
}