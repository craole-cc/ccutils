use dotenvy::dotenv;
// use dotenv_vault::dotenv;
use genai::{
  Client, ServiceTarget,
  adapter::{self, AdapterKind},
  chat::{
    ChatMessage, ChatRequest,
    printer::{PrintChatStreamOptions, print_chat_stream}
  }
};
use std::env::var;

const MODELS: &[(&str, &str)] = &[
  ("MODEL_TAG", "MODEL_KEY"),
  ("OPENAI_MODEL", "OPENAI_API_KEY"),
  ("ANTHROPIC_MODEL", "ANTHROPIC_API_KEY"),
  ("GEMINI_MODEL", "GEMINI_API_KEY"),
  ("COHERE_MODEL", "COHERE_API_KEY"),
  ("XAI_MODEL", "XAI_API_KEY"),
  ("GROQ_MODEL", "GROQ_API_KEY"),
  ("OLLAMA_MODEL", "OLLAMA_API_KEY") // (MODEL_OLLAMA, ""),
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
  let print_options = PrintChatStreamOptions::from_print_events(false);

  for (model_var, key_var) in MODELS {
    let model = match var(model_var) {
      Ok(val) => val,
      Err(_) => continue
    };
    let key = match var(key_var) {
      Ok(val) => val,
      Err(_) => continue
    };
    logline::trace!("{:#?}: {:#?}", &model, &key);

    let response = client.exec_chat(&model, request.clone(), None).await?;

    // let response_content = match response.content_text_into_string() {
    //     Ok(content) => content,
    //     Err(err) => {
    //         logline::error!("Error parsing response content: {:#?}", &err);
    //         continue;
    //     }
    // };

    let stream = client.exec_chat_stream(&model, request.clone(), None).await?;

    // logline::trace!("Response: {:#?}", &response_content);
    // logline::trace!(
    // 	"StreamedResponse: {:#?}",
    // 	print_chat_stream(stream, Some(&print_options)).await?
    // );
  }

  Ok(())
}

fn get_env_var(key: &str) -> Result<String, Box<dyn std::error::Error>> {
  var(key).map_err(|_| format!("{key} not set").into())
}
