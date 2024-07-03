use error_chain::error_chain;
use reqwest;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers: {:#?}", res.headers());
    let body = res.text().await?;
    println!("Body: \n{}", body);
    Ok(())
}
