fn main() {
    let ans_1 = f_to_c(20);
    println!("20 degrees Fahrenheit is {} degrees Celsius", ans_1);
    let ans_2 = fib(10);
    println!("10th fibonacci number is {}", ans_2);
    print_twelve_days_of_xmas();
}

fn f_to_c(t : i32) -> i32 {
    (t - 32 ) * 5 / 9
}

fn fib(n : i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n-1) + fib(n-2),
    }
}

fn print_twelve_days_of_xmas() {
    const DAYS : [&'static str; 12] = [
        "first", // <- type is `&'static str`, will go further in chapter 4
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    for day in DAYS.iter() {
        println!("On the {} day of Christmas my true love sent to me", day);
    }
}


