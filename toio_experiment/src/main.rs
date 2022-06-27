use std::time::Duration;
use toio::Cube;
use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    // Search for the nearest cube.
    let mut cube = Cube::search().nearest().await.unwrap();

    // Connect.
    cube.connect().await.unwrap();

    // Print status.
    println!("version   : {}", cube.version().await.unwrap());
    println!("battery   : {}%", cube.battery().await.unwrap());
    println!("button    : {}", cube.button().await.unwrap());

    // Move forward.
    cube.go(10, 10, None).await.unwrap();

    delay_for(Duration::from_secs(3)).await;

    // Spin for 2 seconds.
    cube.go(100, 5, Some(Duration::from_secs(2))).await.unwrap();

    delay_for(Duration::from_secs(3)).await;
}