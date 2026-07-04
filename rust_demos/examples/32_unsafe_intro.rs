fn main() {
    let mut number = 10;

    let const_ptr = &number as *const i32;
    let mut_ptr = &mut number as *mut i32;

    unsafe {
        println!("const pointer reads: {}", *const_ptr);
        *mut_ptr += 5;
        println!("mut pointer reads: {}", *mut_ptr);
    }

    println!("number = {number}");
}
