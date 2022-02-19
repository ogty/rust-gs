use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

extern crate uuid;
use uuid::Uuid;

trait Initializer {
    fn write(&mut self, path: &str);
}

struct ArticleInitializer {
    title: String,
    topics: Vec<String>,
}

impl Default for ArticleInitializer {
    fn default() -> ArticleInitializer {
        ArticleInitializer {
            title: String::from(""),
            topics: Vec::new(),
        }
    }
}

impl Initializer for ArticleInitializer {
    fn write(&mut self, path: &str) {
        let path = Path::new(&path);
        let display = path.display();
        let mut template = String::from("");

        if self.title != String::from("") {
            let topics_length = self.topics.len();
            if topics_length > 0 {
                if topics_length > 2 {
                    let before_last = topics_length - 1;
                    let topics = self.topics[..before_last].join(", ");
                    template = format!(
                        "---\ntitle: {}\nemoji: 🐒\ntype: tech\ntopics: [{}{}]\npublished: false\n---", 
                        self.title, topics, self.topics[before_last]
                    );
                } else {
                    template = format!(
                        "---\ntitle: {}\nemoji: 🐒\ntype: tech\ntopics: [{}]\npublished: false\n---", 
                        self.title, self.topics[0]
                    );
                }
            } else {
                template = format!("---\ntitle: {}\nemoji: 🐒\ntype: tech\ntopics: []\npublished: false\n---", self.title);
            }
        } else {
            template = String::from("---\ntitle: \nemoji: 🐒\ntype: tech\ntopics: []\npublished: false\n---");
        }
    
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
    
        match file.write_all(template.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        }
    }
}

fn main() {
    let id = Uuid::new_v4();
    let path = format!("./articles/{}.md", id);
    let mut args: Vec<String> = env::args().collect();
    args.push(String::from(""));

    let mut initializer = ArticleInitializer{ ..Default::default() };
    initializer.title = args[1].to_string();
    initializer.topics = (&args[2..]).to_vec();
    initializer.write(&path);
}
