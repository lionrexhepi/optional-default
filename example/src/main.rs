use partial_default::{new, PartialDefault};

#[derive(Debug, PartialDefault)]
struct Something {
    field1: i32,
    #[optional(default = 42)]
    field2: i32,
    #[optional]
    field3: i32,
}

fn main() {
    let something = new!(Something { field1: 1 });

    println!("{something:#?}");
}
