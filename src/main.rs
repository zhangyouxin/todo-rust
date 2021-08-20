use std::{collections::HashMap, io::Read};
#[derive(Debug)]
struct Todo {
    map: HashMap<String, bool>
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.map)
    }
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("todos.txt")?;
            let mut content = String::new();
            file.read_to_string(& mut content)?;
            let map: HashMap<String, bool> = content
                .lines()
                .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
                .map(|v| (v[0], v[1]))
                .map(|(k,v)| (String::from(k), <bool as std::str::FromStr>::from_str(v).unwrap()))
                .collect();
            Ok(Todo {map})
    }
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), std::io::Error>{
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("todos.txt", content)
    }
    fn complete(&mut self, key: &String) -> Option<()>{
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
fn main() {
    let mut todo = Todo::new().expect("Init todos failure.");

    let action = std::env::args().nth(1).expect("Please specify an action");

    if action == "ls" {
        println!("{}", todo);
    } else {
        let item = std::env::args().nth(2).expect("Please specify an item");
        if action == "add" {
            todo.insert(item);
            match todo.save() {
                Ok(_) => println!("action saved."),
                Err(why) => println!("Something wrong: {}", why),
            }
        } else if action == "complete" {
            match todo.complete(&item) {
                None => println!("{} is not found!", item),
                Some(_) => match todo.save() {
                    Ok(_) => println!("{} saved.", item),
                    Err(why) => println!("Oops: {}", why),
                }
            }
        }
    }

}