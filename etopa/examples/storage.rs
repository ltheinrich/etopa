use etopa::data::SecureStorage;

fn main() {
    let db = SecureStorage::new(
        "/home/lennart/test.txt".to_string(),
        "12345678901234567890123456789012",
    )
    .unwrap();
    println!("{}", db.value("hallo").unwrap());
    let a: i32 = db.get("nice").unwrap();
    println!("{}", a);
}
