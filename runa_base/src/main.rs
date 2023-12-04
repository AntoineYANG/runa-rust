use dotenv::dotenv;

mod gpt_client;

#[tokio::main]
async fn main() {
	dotenv().ok();

	match gpt_client::fetch_gpt_response("Translate the following English text to French: 'Hello, world!'").await {
		Ok(response) => println!("{}", response),
		Err(e) => println!("{}", e),
	}
}
