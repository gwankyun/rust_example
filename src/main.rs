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

fn main() {
    println!("Hello, world!");

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
}
