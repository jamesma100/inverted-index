use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let mut word_pos = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let dir_path = parse_config(&args);

    let paths = fs::read_dir(dir_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    for path in paths {
        let path_clone = path.clone();
        let contents: String = fs::read_to_string(path)
            .expect("Error reading file");

        for line in contents.lines() {
            let splitline = line.split([' ', ',', '.', '?', '!']);
            for word in splitline {
                if !word.is_empty() {
                    insert_into_hm(&mut word_pos, word, &path_clone); 
                }
            }
        }
    }
    // info(&mut word_pos);
    // get_occurences(&mut word_pos, "well");
}

fn parse_config(args: &[String]) -> String {
    let file_path = args[1].clone();
    file_path
}

fn insert_into_hm(hm: &mut HashMap<String, Vec<String>>, word: &str, doc: &String) {
    hm.entry(word.to_owned()).or_insert(Vec::new()).push(doc.to_owned());
}

fn info(hm: &mut HashMap<String, Vec<String>>) {
    for (word, pos) in hm {
        println!("{}: {}", word, pos.len());
    }
}

fn get_occurences(hm: &mut HashMap<String, Vec<String>>, word: &str) {
    let docs = hm.get(word);
    println!("{:?}", docs); 
}
