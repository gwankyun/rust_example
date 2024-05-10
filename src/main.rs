use log;

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
    println!("file: {}", file!());
    println!("line: {}", line!());
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
    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(v[0], 1);

    {
        let mut v = vec![1, 2, 3];
        assert_eq!(v.len(), 3);
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], 4);
        v.pop();
        assert_eq!(v.len(), 3);
    }
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    log::info!("Hello, world!");

    {
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

    {
        let str1 = give_ownship();
        log::info!("{str1}");
        take_ownship(str1); // 被移走了
        // log::info!("{str}"); // 無法編譯

        let mut str2 = give_ownship();
        str2 = take_and_give_ownship(str2); // 用完再移回來
        log::info!("{str2}"); // 可以編譯
    }

    {
        let mut i = 2;
        set_value(&mut i, 0);
        assert_eq!(i, 0);
    }

    {
        let mut a: i32 = 1;
        let mut b: i32 = 2;
        swap(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    {
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

    // 數組
    array_example();

    print_info();

    vec_example();
}
