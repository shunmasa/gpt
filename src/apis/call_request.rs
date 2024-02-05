use crate::models::general::llm::{ APIResponse, ChatCompletion, Message};
use dotenv::dotenv;

use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap,HeaderValue};

//Call Large Language Model(i.e GPT-4)
//dyn compile time ,Send threads if function run more than once, threads safety  
pub async fn call_gpt(messages:Vec<Message>) -> Result<String,Box<dyn std::error::Error + Send>>{
    dotenv().ok();

    //Extract API Key information

    let api_key:String = env::var("OPEN_AI_KEY")
    .expect("Open_AI _KEY not found in enviroment variables");
    let api_org:String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in enviroment variables");
    
    //Confirm endpoint
    let url:&str = "https://api.openai.com/v1/chat/completions";

    //Create Headers// hashmap
    let mut headers = HeaderMap::new();
    //Create api key header

    headers.insert(
    "authorization",
    HeaderValue::from_str(&format!("Bearer {}",api_key))
    .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?
    //? 
    
);
    //.unwrap() swap unwrap to map_err()
   
    //&format!("Bearer {}",api_key) // string

//Create api org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
        .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?
    );  //&str, not String
    
    let client = Client::builder()
    .default_headers(headers)
    .build()
    .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?;
   
     
     //Create Chat Compleation

     let chat_completion = ChatCompletion {
        model:"gpt-4-0125-preview".to_string(),
        messages,
        temperature:0.1
     };

     // Troble Shooting
    //  let res_raw = client
    //  .post(url)
    //  .json(&chat_completion)
    //  .send()
    //  .await
    //  .unwrap();
     ///Swap to content or error handling
//   dbg!(res_raw.text().await.unwrap());


//Extract GET API RESPONSE
  let res:APIResponse = client
     .post(url)
     .json(&chat_completion)
     .send()
     .await
     .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?
     .json()
     .await
     .map_err(|e| -> Box<dyn std::error::Error + Send>{Box::new(e)})?;
    //Response for success and errror 
  
  //Send Response
  Ok(res.choices[0].message.content.to_string())//or clone()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai(){
        let message = Message {
         role:"user".to_string(),
         content:"Hi there,this is a test.Give me a short response".to_string()

        };
        let messages = vec!(message);
        let res:Result<String,Box<dyn std::error::Error + Send>> = call_gpt(messages).await;
        // if let Ok(res_str) = res{
        // //    dbg!(res_str)
        //     assert!(true)
        // }else{
        //     assert!(false)
        // }

        match res{
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true)
            },
            Err(_) => {
                assert!(false)
            }
        }
    }
}
