//use raylib::prelude::*;
use std::collections::*;
use bevy::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    address: String,
    age: i16
}

impl Viking {
    fn new(name: &str, address: &str, age: i16) -> Viking {
        Viking { name: name.to_string(), address: address.to_string(), age }
    }

    fn display(&self) {
        println!("{:?}", &self);
    }
}


fn main() {

    let text: String = String::from("Hello World");
    println!("{text}");

    let slice: &str = &text[0..5];
    println!("{slice}");

    for ch in slice.chars().rev() {
        print!("{ch}");
    }
    println!();


    let names: Vec<&str> = vec!["name1", "name2", "name3"];
    println!("{:?}  len: {}", names, names.len());
    if names.contains(&"name4") {
        println!("{}", "true");
    } else {
        println!("{}", "false");
    }
    println!("at 1 : {}", names[1]);

    let nums: Vec<i16> = vec![1, 2, 3, 4, 5];
    for num in &nums {
        match num {
            1 => {
                println!("number is 1");
            },
            2 => {
                println!("number is 2");
            },
            3 => {
                println!("number is 3");
            },
            4 => {
                println!("number is 4");
            },
            _ => {
                println!("none number");
            }
        }
    }
    for (idx, item) in nums.iter().enumerate() {
        println!("{idx}: {item}");
    }
    println!("{nums:?}");

    match true {
        true => {
            println!("true");
        },
        false => {
            println!("false");
        }
    }

    let mut map_names: HashMap<&str, i16> = HashMap::new();
    map_names.insert("name1", 10);
    map_names.insert("name2", 20);
    map_names.insert("name3", 30);
    map_names.insert("name4", 40);
    let old_value: Option<i16> = map_names.insert("name4", 10);
    println!("{:?}    size of map is: {}", map_names, map_names.len());
    if old_value.is_some() {
        println!("old value: {}", old_value.unwrap()); // 40
    }
    match old_value {
        None => println!("new insert"),
        Some(value) => println!("old value: {}", value)
    }

    let to_find: [&str; 3] = ["name2", "name3", "name5"];
    for item in to_find {
        match map_names.get(item) {
            None => {
                println!("{item} not in map!");
            },
            Some(name) => {
                println!("map have {item}, value is: {name} !");
            }
        }
    }

    for (name, age) in map_names {
        println!("key: {name} : value: {age}");
    }
    //println!("{:?}", map_names);


    let map_test:HashMap<Viking, i16> = HashMap::from([
        (Viking::new("name1", "add1", 10), 10),
        (Viking::new("name2", "add2", 20), 20),
        (Viking::new("name3", "add3", 30), 30),
        (Viking::new("name4", "add4", 40), 40),
    ]);
    for (viking, _) in &map_test {
        //println!("{viking:?} has {age}");

        viking.display();
    }

    let map_iter = map_test.iter();
    for (key, value) in map_iter {
        println!("{key:?} has {value}");
    }

    {
        let mut s:String = String::from("hello");
        println!("{s}");

        s.push_str(", world!");

        println!("{s}");

    }
    // print!("{s}"); // error: not found in this scope

    App::new()
        .add_plugins(DefaultPlugins)
        .run();

    //let (rl, thread) = raylib::init().title("Hello, World").build();
    //start_game(thread, rl);
}

/*fn start_game(thread: RaylibThread, mut handle: RaylibHandle) {

    const WIDTH: i16 = 680;

    while !handle.window_should_close() {
        let mut d = handle.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(std::format!("Hello World ! {WIDTH}").as_str(), 12, 12, 20, Color::BLACK);
        d.draw_text("Test Raylib", 80, 80, 18, Color::BLUE);
    }
}*/
