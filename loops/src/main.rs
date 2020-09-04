use std::io;

fn main() {
    loop {
        println!("Hello, world!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The Result is {0}", result);

    //3-3
    let mut number = 3;

    while number != 0 {
        println!("{:?}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 3-4
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {0}", a[index]);
        index += 1;
    }

    // 3-5
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Range
    for number in (1..4).rev() {
        println!("{:?}", number);
    }
    println!("LIFTOOF!!!");

    // 摄氏度转华氏度
    loop {
        let mut temp = String::new();

        io::stdin().read_line(&mut temp).expect("input failed !!!");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please intpu ths number !!!");
                break;
            }
        };

        // 摄氏度转华氏度
        let result: f32 = temp * 1.8 + 32.0;
        println!("{:?} ℉", result);

        // 华氏度转摄氏度
        let result: f32 = (temp - 32.0) * 1.8;
        println!("{:?} ℃", result);
    }

    // 裴波那契数列
    for n in (0..9) {
        if n == 0 || n == 1 {
            println!("{:?}", n);
        } else {
            println!("{:?}", (n - 2) + (n - 1));
        }
        //println!("{:?}", fib(n));
    }
}

fn fib(d: i32) -> i32 {
    if d == 0 || d == 1 {
        d
    } else {
        fib(d - 2) + fib(d - 1)
    }
}
