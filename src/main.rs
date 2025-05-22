use log;
// use rust_example::collections_example;
use rust_example::test_mod;
// use std::collections::HashMap;
// use std::collections::VecDeque;
use std::fs;
use std::env;
use std::io::Write;
// use std::io;

use log4rs;

/// 聲明模塊
pub mod a;
pub mod b;

/// 修改參數
pub fn set_value<T>(num: &mut T, value: T) -> () {
    *num = value;
}

/// 交換參數
pub fn swap<T: Copy>(a: &mut T, b: &mut T) -> () {
    let temp = *a;
    *a = *b;
    *b = temp;
}

#[cfg(test)]
fn give_ownship() -> String {
    String::from("str")
}

#[cfg(test)]
fn take_ownship(str: String) {
    log::info!("take_ownship: {}", str);
}

#[cfg(test)]
fn take_and_give_ownship(str: String) -> String {
    log::info!("take_and_give_ownship: {}", str);
    str
}

fn print_info() {
    log::info!("file: {}", file!()); // 相對文件名
    log::info!("line: {}", line!()); // 行號
}


fn add<T: Copy + std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn generic_example() {
    assert_eq!(add(1, 2), 3);
}

fn string_examle() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    log::info!("{hello}");
    log::info!("{world}");

    let str = String::from("012345");
    let str012 = &str[0..3]; // [0..3)
    log::info!("{str012}");

    {
        let str = "123".to_string();
        log::info!("{str}");
    }

    {
        let mut str = String::from("");
        assert_eq!(str, "");
        str.push_str("123");
        assert_eq!(str, "123");
        str += "456";
        assert_eq!(str, "123456");
        // 插入字符串
        str.insert_str(1, "0");
        assert_eq!(str, "1023456");
    }
}

fn tuple_example() {
    let tup: (i32, String) = (18, String::from("Tom"));
    assert_eq!(tup.0, 18);
    assert_eq!(tup.1, "Tom")
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn enum_example() {
    let mut c = Color::Red;
    log::info!("{:?}", c);
    assert_eq!(c, Color::Red);
    c = Color::Blue;
    assert_eq!(c, Color::Blue);
    c = Color::Green;
    log::info!("{:?}", c);
    assert_eq!(c, Color::Green);
}

fn flow_control_example() {
    // 選擇語句
    let c = true;
    if c {
        log::info!("yes");
    } else {
        log::info!("no");
    };
    // 可用作表達式
    let s = if c {
        String::from("yes")
    } else {
        String::from("no")
    };
    assert_eq!(s, "yes");
    // for in
    {
        let mut sum = 0;
        let a = [1, 2, 3, 4];
        for i in &a {
            sum += i;
        }
        assert_eq!(sum, 10);
    }
}

/// 文件讀寫
fn file_example() {
    // 讀取文件
    let text = fs::read_to_string("test.txt").unwrap();
    log::info!("text: {:?}", text);

    // 創建文件
    let mut file = std::fs::File::create("data.txt").unwrap();
    log::info!("file: {:?}", file);

    // 寫入文件
    file.write_all("123".as_bytes()).unwrap();

    let dest = "copy.txt";

    // 複製
    fs::copy("data.txt", dest).unwrap();
    log::info!("copy: {:?}", fs::read_to_string("copy.txt"));

    // 刪除
    if fs::metadata(dest).is_ok() {
        fs::remove_file(dest).unwrap();
    }
}

/// 生命週期標注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn ref_works() {
        // 不可變引用
        let i = 2;
        let r = &i;
        assert_eq!(*r, 2);

        // 可變引用
        let mut c = 3;
        let rc = &mut c;
        *rc += 1;
        assert_eq!(*rc, 4);
    }

    /// 可變
    #[test]
    fn mut_example() {
        let mut i = 2;
        set_value(&mut i, 0);
        assert_eq!(i, 0);

        {
            let mut a: i32 = 1;
            let mut b: i32 = 2;
            swap(&mut a, &mut b);
            assert_eq!(a, 2);
            assert_eq!(b, 1);
        }
    }

    #[test]
    fn loop_example() {
        let mut i = 1;
        let mut sum = 0;
        loop {
            if i >= 6 {
                break;
            }
            sum += i;
            i += 1;
        }
        assert_eq!(sum, 15);
    }

    #[test]
    fn while_example() {
        let mut n = 3;
        while n != 0 {
            n -= 1;
        }
        assert_eq!(n, 0);
    }

    #[test]
    fn for_example() {
        let a = [1, 2, 3, 4, 5];
        let mut sum = 0;
        for i in &a {
            sum += i;
        }
        assert_eq!(sum, 15);
    }

    /// 所有權
    #[test]
    fn ownship_example() {
        let str1 = give_ownship();
        log::info!("{str1}");
        take_ownship(str1); // 被移走了
        // log::info!("{str}"); // 無法編譯

        let mut str2 = give_ownship();
        str2 = take_and_give_ownship(str2); // 用完再移回來
        log::info!("{str2}"); // 可以編譯
    }

    fn append_string(s: &mut String, s2: String) {
        s.push_str(s2.as_str());
    }

    #[test]
    fn brrow_example() {
        let mut str = String::from("Hello");
        append_string(&mut str, String::from(", world"));
        assert_eq!(str, "Hello, world");
    }

    #[test]
    fn slice_example() {
        let s = String::from("Hello, world");
        let hello: &str = &s[0..5]; // [0..5)
        let world = &s[7..12]; // [7..12)
        assert_eq!(hello, "Hello");
        assert_eq!(world, "world");

        let a = [1, 2, 3, 4, 5];
        let a012: &[i32] = &a[0..3]; // [0..3)
        assert_eq!(a012, &[1, 2, 3]);
    }

    struct Person {
        age: u32,
        sex: bool,
        name: String,
    }

    impl Person {
        /// 更新方法
        fn grow(&mut self) {
            self.age += 1
        }
    }

    #[test]
    fn struct_example() {
        // 都要初始化
        // 順序無關
        let mut tom = Person {
            age: 18,
            sex: true,
            name: String::from("Tom"),
        };

        assert_eq!(tom.age, 18);
        assert_eq!(tom.sex, true);
        assert_eq!(tom.name, String::from("Tom"));

        // 更新語法
        tom = Person {
            age: 19,
            ..tom
        };

        assert_eq!(tom.age, 19);

        // 原地更新
        tom.grow();
        assert_eq!(tom.age, 20);
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(PartialEq)]
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    #[test]
    fn tuple_struct_example() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        // 解構用的語法不一樣
        let Point(x, y, z) = origin;

        assert_eq!(black, Color(0, 0, 0));
        assert_eq!(origin, Point(0, 0, 0));
        assert_eq!((x, y, z), (0, 0, 0));
    }

    #[test]
    fn raw_pointer_example() {
        let mut n = 1;
        let p = &mut n as *mut i32;

        unsafe {
            *p = 2;
        }
        assert_eq!(n, 2);

        let mut a = [1, 2, 3, 4, 5];
        let p = a.as_mut_ptr();
        unsafe {
            *p.wrapping_add(1) = 10;
        }
        assert_eq!(a, [1, 10, 3, 4, 5]);
    }

    #[test]
    fn box_example() {
        let b = Box::new(5);
        assert_eq!(*b, 5);

        let mut a = Box::new([0;100]);
        a[0] = 1;
        assert_eq!(a[0], 1);

        #[allow(unused_assignments)]
        let mut p = std::ptr::null_mut();
        {
            p = Box::into_raw(Box::new(10));
        }
        let b = unsafe {
            *p = 11;
            Box::from_raw(p)
        };
        assert_eq!(*b, 11);
    }

    /// 自定义结构体，实现 Drop trait
    struct CustomResource {
        name: String,
    }

    impl Drop for CustomResource {
        /// 当 CustomResource 实例离开作用域时自动调用此方法
        fn drop(&mut self) {
            println!("Cleaning up resource: {}", self.name);
        }
    }

    #[test]
    fn custom_drop_example() {
        {
            let resource = CustomResource {
                name: String::from("Example")
            };
            // 当 resource 离开作用域时，Drop trait 的 drop 方法会自动被调用
            let _ = resource;
        }
    }

    struct Rectangle {
        width: u32,
        height: u32, 
    }

    impl Rectangle  {
        fn new(width: u32, height: u32) -> Self {
            Self { width, height } 
        }

        fn square(size: u32) -> Self {
            Self::new(size, size)
        }

        fn area(&self) -> u32 {
            self.width * self.height 
        }
    }

    #[test]
    fn impl_example() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);

        let square = Rectangle::square(10);
        assert_eq!(square.area(), 100);
    }

    /// 几何形状枚举
    #[derive(Debug)]
    enum Geometry {
        /// 矩形，包含宽度和高度
        Rectangle { width: f64, height: f64 },
        /// 圆形，包含半径
        Circle { radius: f64 },
        /// 三角形，包含三条边的长度
        Triangle { a: f64, b: f64, c: f64 },
    }

    impl Geometry {
        /// 计算面积
        fn area(&self) -> f64 {
            match self {
                Self::Rectangle { width, height } => width * height,
                Self::Circle { radius } => {
                    let pi = std::f64::consts::PI;
                    radius * radius * pi
                }
                Self::Triangle { a, b, c } => {
                    let s = (a + b + c) / 2.0;
                    (s * (s - a) * (s - b) * (s - c)).sqrt() 
                }
            }
        } 
    }

    #[test]
    fn geometry_example() {
        let rect = Geometry::Rectangle { width: 10.0, height: 20.0 };
        let circle = Geometry::Circle { radius: 5.0 };
        let triangle = Geometry::Triangle { a: 3.0, b: 4.0, c: 5.0 };
        assert_eq!(rect.area() as i32, 200);
        assert_eq!(circle.area() as i32, 78);
        assert_eq!(triangle.area() as i32, 6);
    }

    #[test]
    fn mod_example() {
        assert_eq!(a::mod_name(), "a");
        assert_eq!(b::mod_name(), "b");
        assert_eq!(b::c::mod_name(), "c");
    }
}

fn main() {
    log4rs::init_file(
        "config/log4rs.yaml",
        Default::default()).unwrap();
    log::info!("Hello, world!");

    print_info();
    generic_example();
    string_examle();
    tuple_example();
    enum_example();
    assert_eq!(test_mod::hello(), "test");
    flow_control_example();

    // 當前目錄
    let current_dir = env::current_dir().unwrap();
    log::info!("current_dir: {:?}", current_dir);

    file_example();

    test_mod::hello();

    log::info!("longest: {:?}", longest("12", "123"));
}
