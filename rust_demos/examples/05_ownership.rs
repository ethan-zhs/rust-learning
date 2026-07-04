fn take_ownership(text: String) {
    println!("owned text = {text}");
}

fn copy_number(n: i32) {
    println!("copied number = {n}");
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {s2}");

    let s3 = String::from("clone me");
    let s4 = s3.clone();
    println!("s3 = {s3}, s4 = {s4}");

    take_ownership(s4);

    let n = 10;
    copy_number(n);
    println!("n is still available because i32 is Copy: {n}");
}
