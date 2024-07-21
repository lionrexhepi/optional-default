use optional_default::OptionalDefault;

#[derive(Debug, OptionalDefault)]
struct Something {
    franz: i32,
    #[optional(default = 42)]
    leonard: i32,
    #[optional]
    field3: i32,
}

fn main() {
    let something = Something! {
        leonard: 1,
        franz: 2
    };
    println!("{:#?}", something);
}
