use actix_web::{web, App, HttpServer};
use fuzzywuzzy::{process, utils, fuzz};
use std::{sync::Arc, collections::{HashSet, HashMap}};
use std::str::Split;


fn get_match(name: &str, choices: &Vec<String>) -> String {
    return process::extract_one(
        name,
        choices,
        &utils::full_process,
        &fuzz::wratio,
        0,
    ).unwrap().0;
}

fn create_match_choices() -> Vec<String> {
    vec![String::from("Matt"), String::from("Peter"), String::from("Andrew")]
}
struct ApplicationData {
    choices: Vec<String>,
    word_counter: HashMap<&'static str, i64>
}

impl ApplicationData {
    fn new() -> ApplicationData {
        let choices = create_match_choices();
        let mut word_counter: HashMap<&'static str, i64> = HashMap::new();
        word_counter.insert("abbott", 5);
        word_counter.insert("llc", 50);
        word_counter.insert("fresenius", 4);
        word_counter.insert("corp", 44);
        ApplicationData {choices, word_counter}
    }
    fn matcher(&self, name: &str) -> String {
        get_match(name, &self.choices)
    }
    fn uniqueness(&self, word: &str) -> f64 {
        let word_sequence: Split<char> = word.split(' ');
        let mut uniqueness: f64 = 0.0;
        for word in word_sequence {
            let count = self.word_counter.get(word).unwrap();
            uniqueness += 1.0/(*count as f64);
        }
        uniqueness.powf(0.5);
        uniqueness
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Arc::new(ApplicationData::new()));
    HttpServer::new(move || {
        App::new().app_data(data.clone()).route("/{name}", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn index(web::Path(name): web::Path<String>, data: web::Data<Arc<ApplicationData>>) -> String {
    format!("Hello, {}", data.uniqueness(&name))
}