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
}
