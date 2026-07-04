#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrderId(u64);

type Score = u32;

fn find_user(id: UserId) {
    println!("find user: {}", id.0);
}

fn main() {
    let user_id = UserId(1);
    let order_id = OrderId(1);
    let score: Score = 95;

    find_user(user_id);
    println!("order id = {}", order_id.0);
    println!("score = {score}");

    // find_user(order_id); // This will not compile, and that is the point.
}
