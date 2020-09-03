fn main() {
    let mut x = 5;
    println!("The value of x is: {0}", x);
    x = 6;
    println!("The value of x is: {0}", x);

    const MAX_POINT: u32 = 100_000;
    println!("The value of MAX_POINT is: {0}", MAX_POINT);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {0}", y);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The Space Length is: {0}", spaces);

    // 数值类型
    let num: u8 = 255;
    let num: i8 = 0;

    let num: u16 = 65535;
    let num: i16 = 0;

    let num: u32 = 100_000_100;
    let num: i32 = 0;

    let num: u64 = 00001;
    let num: i64 = 0;

    let num: u128 = 0111111111;
    let num: i128 = 0;

    let num: isize = 01;
    let num: usize = 0;

    // 数值运算
    let num = num + 1;
    let num = num - 1;
    let num = num / 1;
    let num = num * 1;
    let num = num % 1;

    // 浮点型
    let float: f32 = 3.14;
    let float: f64 = 3.1415926;

    // 布尔类型
    let b: bool = false;

    // 复合类型
    let tup: (i32, u16, bool, f32) = (1314, 1234, false, 3.1415); // 元组
    let (a, b, c, d) = tup; //  解构（destructuring）
    println!("{0}.{1}", c, tup.3); // 也可以使用点号（.）后跟值的索引来直接访问它们

    // 数组类型
    let array = [1, 2, 3, 4, 5];
    // 在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // 如果要为每个元素创建包含相同值的数组，可以指定初始值，后跟分号，然后在方括号中指定数组的长度
    // 等效于 let array = [3, 3, 3, 3, 3];
    let array = [3; 5];

    println!(
        "{0}, {1}, {2}, {3}, {4}",
        array[0], array[1], array[2], array[3], array[4]
    );
}
