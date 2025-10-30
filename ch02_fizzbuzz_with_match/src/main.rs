fn main() {
    for i in 1..=100 {
        let s = fizzbuzz(i);
        println!("{}", s);
    }
}

fn fizzbuzz(i: i32) -> String {
    let remainder = (i % 3, i % 5);
    match remainder {
        (0, 0) => format!("FizzBuzz"),
        (0, _) => format!("Fizz"),
        (_, 0) => format!("Buzz"),
        _ => format!("{}", i),
    }
}
