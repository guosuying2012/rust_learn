
fn greet_world() {
    let southern_germany = "123";
    let chinese = "您好，世界!";
    let english = "Hello World !";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", region);
    }
}


fn main() {
    println!("Hello, world!");
    greet_world();
}
