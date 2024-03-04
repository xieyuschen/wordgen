use crate::word::{new_word, Word};
use reqwest;
use std::{fs::{self, File}, io::Write};
mod word;

fn main() {
    let f = fs::read_to_string("word.txt").unwrap();
    let words: Vec<&str> = f.split(",").collect();
    // let dic_key = std::env::var("DICKEY").unwrap();
    let dic_key= "be046f67-379a-4d1a-9003-a6322995bbec";
    let mut result: Vec<Word> = vec![];
    for word in words.iter() {
        let req_url = format!(
            "https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}",
            word, dic_key
        );
        let resp = match reqwest::blocking::get(req_url) {
            Ok(resp) => resp.text().unwrap(),
            Err(err) => panic!("Error: {}", err),
        };

        let details = match new_word(word.to_string(),resp) {
            Ok(e) => e,
            Err(_)=> continue,
        };
        result.push(details);
    }
    emit_output(result);
}

fn emit_output(words: Vec<Word>) {
    let mut file = File::create("output.html").unwrap();

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
    
    file.write(html.as_bytes()).unwrap();

}
