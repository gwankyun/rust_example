pub mod test_mod {
    pub fn hello() -> String {
        log::info!("test");
        String::from("test")
    }
}

pub mod collections_example {
    // use std::collections::HashMap;
    // use std::collections::VecDeque;

    #[cfg(test)]
    fn get_array_item(a: &[i32], index: usize) -> i32 {
        a[index]
    }

    #[cfg(test)]
    mod tests {
        use crate::collections_example::*;
        #[test]
        pub fn array()
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

        #[test]
        pub fn vec()
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

        #[test]
        pub fn hash_map() {
            use std::collections::HashMap;
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
    }

}
