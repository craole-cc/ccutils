use dotenvy::dotenv;
// use dotenv_vault::dotenv;
use genai::{
	adapter::{self, AdapterKind},
	chat::{
		printer::{print_chat_stream, PrintChatStreamOptions},
		ChatMessage, ChatRequest,
	},
	Client, ServiceTarget,
};
use std::env::var;

const MODELS: &[(&str, &str)] = &[
	("OPENAI_MODEL", "OPENAI_API_KEY"),
	("ANTHROPIC_MODEL", "ANTHROPIC_API_KEY"),
	("GEMINI_MODEL", "GEMINI_API_KEY"),
	("COHERE_MODEL", "COHERE_API_KEY"),
	("XAI_MODEL", "XAI_API_KEY"),
	("GROQ_MODEL", "GROQ_API_KEY"),
	("OLLAMA_MODEL", "OLLAMA_API_KEY"),
	// (MODEL_OLLAMA, ""),
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv()?; // Load variables from .env
	logline::init(); // Initialize logging

	let client = Client::default();
	let request = ChatRequest::new(vec![
		ChatMessage::system("Answer the following question!"),
		ChatMessage::user("What is the meaning of life?"),
	]);
	// let options Option<T> = None;
	// let response =
	// 	client.exec_chat_stream(model, request, None).await?;

	// print_chat_stream(response, None).await?;

	for (model_var, key_var) in MODELS {
		let model = match var(model_var) {
			Ok(val) => val,
			Err(_) => continue,
		};

		let key = match var(key_var) {
			Ok(val) => val,
			Err(_) => continue,
		};
		logline::trace!("{:#?}: {:#?}", &model, &key);


		let adapter = client.resolve_service_target(&model);
		// logline::trace!("adapter_kind: {:#?}", adapter_kind.ok().unwrap());

		// let adapter_kind = adapter::AdapterKind::from(model.as_str());

		// let adapter_kind = client.resolve_model_iden(&model);
		// client.resolve_model_iden(&model)?.adapter_kind;
	}

	Ok(())
}
