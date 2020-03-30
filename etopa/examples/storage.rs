use etopa::data::SecureStorage;

fn main() {
    let mut db = SecureStorage::new(
        "/home/lennart/test.txt".to_string(),
        "12345678901234567890123456789012",
    )
    .unwrap();
    println!("{:?}", db.parse().unwrap());
    db.write_file("etopa_secure_storage#1\nhallo=du\nnix\nda=7".to_string());
    println!("{:?}", db.parse().unwrap());
}
