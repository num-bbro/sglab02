use sglab02_lib::add;

#[tokio::main]
async fn main() {
    sglab02_lib::sg::prc5::prc54().await.expect("?");
    println!("Adder2: {}", add(2, 2));
}
