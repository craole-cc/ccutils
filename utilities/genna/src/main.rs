use genai::{
	chat::{
		printer::{print_chat_stream, PrintChatStreamOptions},
		ChatMessage, ChatRequest,
	},
	Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	logline::init();
	let client = Client::default();

	let request = ChatRequest::new(vec![
		ChatMessage::system("Answer the following question!"),
		ChatMessage::user("What is the meaning of life?"),
	]);

	let model = "gpt-3.5-turbo";
	logline::debug!("Using model: {:#?}", model);

	// let options Option<T> = None;

	let response =
		client.exec_chat_stream(model, request, None).await?;

	print_chat_stream(response, None).await?;

	Ok(())
}
