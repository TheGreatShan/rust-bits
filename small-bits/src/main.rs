fn main() {
    let emoji: char = '😂';
    let tup: (i64, String) = (500, String::from("Fortnite"));
    println!("Hello, world! {emoji}");
    let res = tup.1;
    println!("Hello, world! {res}");
}
