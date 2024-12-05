#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("http://127.0.0.1:8080")
    .await?;

    println!("{response:?}");

    Ok(())
}
