use tokio::time::{sleep, Duration};

async fn example() -> String {
    println!("Start async function");
    sleep(Duration::from_secs(2)).await; // 模拟异步操作
    println!("Async operation completed");
    "Hello, world!".to_string()
}

#[tokio::main]
async fn main() {
    let result = example().await;
    println!("{}", result);
}
