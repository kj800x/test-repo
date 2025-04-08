fn main() {
    println!("Hello, world!");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("1...");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("2...");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("3...");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("4...");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("5, done!");
}
