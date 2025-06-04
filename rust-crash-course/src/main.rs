// #![deny(clippy::all)]
// #[derive(PartialEq)]
// #[derive(Debug)]

fn main() {
    let mut value = 10;
    print!("{}", value);

    let _ref1 = &mut value;
    *_ref1 = 5;
    print!("{}", _ref1);
    value = 6;
    print!("{}", value)
}
