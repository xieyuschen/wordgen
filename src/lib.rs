use reqwest;
use serde_wasm_bindgen::to_value;

use crate::word::{Word, new_word};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::future_to_promise;
mod word;

#[wasm_bindgen]
pub async fn process(input: &[u8]) -> js_sys::Promise {
    let input = input.to_owned();
    future_to_promise(async move {
        let words: Vec<&str> = std::str::from_utf8(&input).unwrap().split(",").collect();
        let mut result: Vec<Word> = vec![];
        for word in words.iter() {
            let details = get_response(&word).await;
            result.push(details);
        }
        
        let output = emit_output(result).into_bytes();
        to_value(&output)
        .map_err(|err| err.into())
    })
}

async fn get_response(w: &str) -> Word {
    let req_url = format!(
        "https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}",
        w,"be046f67-379a-4d1a-9003-a6322995bbec" 
    );

    let resp = reqwest::get(&req_url).await.unwrap().text().await.unwrap();
    
    new_word(w.to_string(),resp).unwrap()
}

fn emit_output(words: Vec<Word>) -> String {
    let mut html = String::new();

    html.push_str("<style>");
    html.push_str("table { border-collapse: collapse; }");
    html.push_str("td, th { border: 1px solid black; padding: 8px; }");
    html.push_str("</style>");
    html.push_str("<table>");
    html.push_str("<tr>");
    html.push_str("<th>Word</th>");
    html.push_str("<th>Seperation</th>");
    html.push_str("<th>Pronunciation</th>");
    html.push_str("<th>Definition</th>");
    html.push_str("</tr>");

    for w in words.iter() {
        let mut total="".to_string();
        for d in w.def.iter(){
            total=format!("{}\n", d);
        }

        html.push_str("<tr>");
        html.push_str("<td>");
        html.push_str(&w.origin);
        html.push_str("</td>");
        html.push_str("<td>");
        html.push_str(&w.hw);
        html.push_str("</td>");
        html.push_str("<td>");
        html.push_str(&w.pro);
        html.push_str("</td>");
        html.push_str("<td>");
        html.push_str(&total);
        html.push_str("</td>");
        html.push_str("</tr>");
    }
    html.push_str("</table>");

    html
}
