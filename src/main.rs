extern crate reqwest;
extern crate scraper;
extern crate clap;

use clap::{Arg, App};

// importation syntax 
use scraper::{Html, Selector};
use std::str;

fn main() {
    
    let matches = App::new("lyrics")
        .version("0.1.0")
        .author("William <me@stylite.me>")
        .about("Genius lyric grabber")
        .arg(Arg::with_name("URL")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("url to lyrics"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    lyrics(url);
}

fn lyrics(url: &str) {

   let mut resp = reqwest::get(url).unwrap(); 
   assert!(resp.status().is_success());

   let body = resp.text().unwrap();
   // parses string of HTML as a document   
   let fragment = Html::parse_document(&body);
   // parses based on a CSS selector
   let lyric = Selector::parse("p").unwrap();

   // iterate over elements matching our selector
   for lyric in fragment.select(&lyric) {
        // grab the headline text and place into a vector
        let lyrics = lyric.text();
        for x in lyrics {
            let stripresult = str::replace(x, "\n", "");
            let result = str::replace(&stripresult, "\"\"", "");
            println!("{}", result)
        }  
    }
}

