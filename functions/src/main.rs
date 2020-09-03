fn main() {
    println!("Hello, world! {0}", five());
    println!("Hello, world! {0}", push_one(9));
}

fn five() -> i32 {
    5
}

/*
comment
*/

fn push_one(x: i32) -> i32 {
    x + 1
}
