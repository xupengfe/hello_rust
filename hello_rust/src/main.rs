fn greet_world() {
    let southern_germany: &str = "Grüß Gott!\n";
    let chinese: &str = "世界，你好.\n";
    let english: &str = "World, hello.\n";
    let hi_rust: &str = "Hello rust!\n\n";
    let regions: [&str; 4] = [southern_germany, chinese, english, hi_rust];
    for region in regions.iter() {
        print!("{}", &region);
    }

    for region in regions {
        print!("{}", &region);
    }
}

fn str_parse() {
    let penguin_data: &str = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Xjw, 180
    Invalid,data
    ";

    let records = penguin_data.lines();

    print!("Name,\tlength(cm)\n");
    for (i, record) in records.enumerate() {
      //if i == 0 || record.trim().len() == 0 {
        if record.trim().len() == 0 {
        continue;
      }

      // 声明一个 fields 变量，类型是 Vec
      // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
      // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
      let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
      if cfg!(debug_assertions) {
          // 输出到标准错误输出
        eprintln!("dbg: record: {:?} -> fields: {:?}  i: {}, record.trim().len():{}",
               record, fields, i, record.trim().len());
      }

      let name = fields[0];
      // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
      //
      // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
      //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
      //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
      //
      // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
      if let Ok(length) = fields[1].parse::<f32>() {
          // 输出到标准输出
          println!("{}, {}cm", name, length);
      }
    }
}

fn show_add() {
    // 使用let来声明变量，进行绑定，a是不可变的
    // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
    // 语句的末尾必须以分号结尾
    let a = 10;
    // 主动指定b的类型为i32
    let b: i32 = 20;
    // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let c: i32 = 30i32;
    // 还能在数值和类型中间添加一个下划线，让可读性更好
    let d = 30_i32;
    // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
    let e = add(add(a, b), add(c, d));

    // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
    // 该函数将指定的格式化字符串输出到标准输出中(控制台)
    // {}是占位符，在具体执行过程中，会把e的值代入进来
    println!("( a:{} + b:{} ) + ( c:{} + d:{} ) = {}", a, b, c, d, e);
}

// 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
    // 返回相加值，这里可以省略return
    i + j
}

fn test_var() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn try_assert() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {}, b = {}", a, b);
    b = true;
    println!("a = {}, b = {}", a, b);
    assert_eq!(a, b);
    b = false;
    println!("a = {}, b = {}", a, b);
    assert_ne!(a, b);
}
fn main() {
    greet_world();
    str_parse();
    show_add();
    test_var();
    try_assert();
}