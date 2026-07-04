fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn grade(score: u32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 60 {
        "Pass"
    } else {
        "Fail"
    }
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
    println!("score 86 => {}", grade(86));

    let mut n = 0;
    while n < 3 {
        println!("while n = {n}");
        n += 1;
    }

    for item in 1..=3 {
        println!("for item = {item}");
    }

    let result = loop {
        n += 1;
        if n == 5 {
            break n * 10;
        }
    };
    println!("loop result = {result}");
}
