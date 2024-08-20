use log;
use rust_example::test_mod;
use std::collections::HashMap;
use std::fs;
use std::env;
use std::io::Write;
// use std::io;

use log4rs;

/// 修改參數
fn set_value<T>(num: &mut T, value: T) -> () {
    *num = value;
}

/// 交換參數
fn swap<T: Copy>(a: &mut T, b: &mut T) -> () {
    let temp = *a;
    *a = *b;
    *b = temp;
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

fn give_ownship() -> String {
    String::from("str")
}

fn take_ownship(str: String) {
    log::info!("take_ownship: {}", str);
}

fn take_and_give_ownship(str: String) -> String {
    log::info!("take_and_give_ownship: {}", str);
    str
}

fn get_array_item(a: &[i32], index: usize) -> i32 {
    a[index]
}

fn print_info() {
    log::info!("file: {}", file!()); // 相對文件名
    log::info!("line: {}", line!()); // 行號
}

fn array_example()
{
    let a = [1, 2, 3, 4, 5];
    log::info!("{}", a[0]);
    log::info!("{}", a.len());
    let sa = &a[1..3];
    for i in a {
        print!("{} ", i);
    }
    print!("\n");
    log::info!("");
    for i in sa {
        print!("{} ", i);
    }
    print!("\n");
    log::info!("");

    assert_eq!(get_array_item(&a[0..], 0), 1);
}

fn vec_example()
{
    let mut v: Vec<i32> = Vec::new();
    v.push(1); // 增加
    v.push(2);
    v.push(3);

    assert_eq!(v[0], 1);

    {
        let mut v = vec![1, 2, 3];
        assert_eq!(v.len(), 3);
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], 4);
        v.pop(); // 移除
        assert_eq!(v.len(), 3);
    }
}

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

fn hash_map_example() {
    let mut m = HashMap::new();

    m.insert("a", 1);
    m.insert("b", 2);
    m.insert("c", 3);

    assert_eq!(m.len(), 3);

    let a = m.get("a");
    match a {
        None => {
            ();
        }
        Some(i) => {
            log::info!("{}", i);
        }
    }
}

fn add<T: Copy + std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn generic_example() {
    assert_eq!(add(1, 2), 3);
}

fn ownship_example() {
    let str1 = give_ownship();
    log::info!("{str1}");
    take_ownship(str1); // 被移走了
    // log::info!("{str}"); // 無法編譯

    let mut str2 = give_ownship();
    str2 = take_and_give_ownship(str2); // 用完再移回來
    log::info!("{str2}"); // 可以編譯
}

fn ref_example() {
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

fn main() {
    log4rs::init_file(
        "config/log4rs.yaml",
        Default::default()).unwrap();
    log::info!("Hello, world!");

    ref_example();
    ownship_example();
    mut_example();
    struct_example();
    // 數組
    array_example();
    print_info();
    vec_example();
    hash_map_example();
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
}
