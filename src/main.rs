use std::collections::HashMap;
use std::path::Path;

fn main() {
    let usage_s = String::from("Usage: ./kvstore [PUT|GET|SHOW] KEY VALUE");
    let mut arguments = std::env::args().skip(1);
    let fun_name = arguments.next().expect(&usage_s);

    let mut database = Database::new().expect("Create DB Failed!");

    match fun_name.as_str() {
        "put" => {
            let key = arguments.next().expect(&usage_s);
            let val = arguments.next().expect(&usage_s);
            database.put(key, val).expect("DB put failed!");
        }
        "get" => {
            let key = arguments.next().expect(&usage_s);
            let val = database.get(&key).expect("Key not found.");
            println!("{key} -> {val}");
        }
        "show" => {
            println!("Store:");
            for (k, v) in database.map.iter() {
                println!("{k} -> {v}");
            }
        }
        _ => {
            // key = None;
            // val = None;
            println!("Not valid!");
        }
    }


}


struct Database {
    map: HashMap<String, String>
}

impl Database {
    pub fn put(&mut self, key: String, val: String) -> Result<(), std::io::Error> {
        // insert a new k,v pair
        println!("Did put {key} -> {val}");
        self.map.insert(key, val);
        let mut lines: Vec<String> = Vec::new();
        for (k, v) in self.map.iter() {
            lines.push(format!("{k}\t{v}"))
        }
        std::fs::write("kv.db", lines.join("\n"))?;
        Ok(())
    }

    pub fn get(&mut self, key: &String) -> Option<&String> {
        self.map.get(key)
    }

    fn new() -> Result<Database, std::io::Error>{
        // create file if it doesn't exist
        if !Path::new("./kv.db").exists() {
            std::fs::write("./kv.db", b"")?;
        }
        // read the file
        let contents = std::fs::read_to_string("./kv.db")?;
        let mut map = HashMap::new();
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt Database");
            // .to_owned() because key and value are of type &str, which is just a pointer
            // "view" to somewhere inside of contents, which is owned by the current scope.
            // However, we want the *map* to own the memory of the String so that we can
            // transfer ownership of the map to the database itself
            map.insert(key.to_owned(), value.to_owned());
            // String::from also works to copy the memory and transfer ownership here.
            // map.insert(String::from(key), String::from(value));
            // This also works
            // map.insert(key.to_string(), key.to_string());
        }
        // parse the string
        // populate the map
        Ok(Database {
            map
        })
    }
}
