use std::error::Error;
use clap::{Arg, Command};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Proxy;
use reqwest::blocking::ClientBuilder;
use serde_json::{Value, json};


const CHATGPT_URL: &str = "https://api.openai.com/v1/chat/completions";
//const CHATGPT_URL: &str =  "https://api.openai.com/v1/models";

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let matches = Command::new("CLI to chat with chatGPT")
        .version("1.0")
        .author("KubyWang <214182526@qq.com>")
        .about("A command line tool to chat with chatGPT")
        .arg(
            Arg::new("question")
                .short('q')
                .long("question")
                .value_name("QUESTION")
                .help("Sets your question to send  to chatGPT")
                .required(true),
        )
        .arg(
            Arg::new("proxy")
                .short('x')
                .long("proxy")
                .value_name("PROXY")
                .required(false)
                .help("Sets the HTTPs's proxy if you can't connect to chatGPT directly. Usage: [protocol://]host[:port]")
        )
        .arg(
            Arg::new("api_key")
                .long("api_key")
                .value_name("API_KEY")
                .help("Sets the chatGPT key, see more: https://platform.openai.com/docs/api-reference/authentication")
                .required(true)

        )
        .get_matches();



    let question:  &String = matches.get_one("question").expect("write your question");

    let api_key:  &String = matches.get_one("api_key").expect("chatGPT api key is required");
    
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.63"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let mut secret = String::from("Bearer ");
    secret.push_str(api_key);
    
    headers.insert("Authorization", HeaderValue::from_str(&secret.as_str()).unwrap());
    
    
    let proxy: Option<&String> = matches.get_one("proxy");

    let proxy = match proxy {
         Some(p) => Some(Proxy::all(p).expect("caonuima")),
         None => None,
     };

    let client_builder = ClientBuilder::new().default_headers(headers);
    let client_builder = if let Some(proxy) = proxy {
         client_builder.proxy(proxy)
     } else {
         client_builder
     };

    let client = client_builder.build()?;
    let request_body = json!({"model": "gpt-3.5-turbo", "temperature": 0.7, "messages": [{"role": "user", "content": question}]});
    let json_body = serde_json::to_string(&request_body)?;
    let response = client.post(CHATGPT_URL).body(json_body).send()?;

    let answer = response.text()?;
    let json_value: Value = serde_json::from_str(&answer).unwrap();
    let message = &json_value["choices"][0]["message"]["content"];
    println!();
    println!("{}", message.to_string());
    Ok(())
}