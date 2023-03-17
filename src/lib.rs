mod rust;

fn func1(n: i128) -> i32 {
    let mut k = 0;
    for _ in 0..n {
        k += 1;
    }
    k
}

fn func2(n: i128) -> i32 {
    let mut k = 0;
    for _ in 0..n {
        for _ in 0..n {
            k += 1;
        }
    }
    k
}

fn func3(n: i128) -> i32 {
    let mut k = 0;
    for _ in 0..n {
        for _ in 0..n {
            for _ in 0..n {
                k += 1;
            }
        }
    }
    k
}





pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn time_use() {
        use std::time::{Duration, Instant};
        use std::io;
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("a number");
            let n = input.trim().parse::<i128>().unwrap();
            let start = Instant::now();
            func2(n);
            let duration = start.elapsed();
            println!("Time elapsed in expensive_function() is: {:?}", duration);
        }
        // println!("{}", n);
    }

    // #[test]
    // fn borrow() {
    //     fn push(vec_ref: &mut Vec<i32>, x: i32) {
    //         let vec = *vec_ref;
    //         vec.push(x);
    //     }
    // }
}
