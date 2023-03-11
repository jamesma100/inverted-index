use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let mut word_pos: HashMap<String, String> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let dir_path = parse_config(&args);
    let dir_path_clone = dir_path.clone();

    let paths = fs::read_dir(dir_path_clone)
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
                    println!("word: {}", word);
                    insert_into_hm(&mut word_pos, word, &path_clone); 
                }
            }
        }
    }
    for (word, pos) in &word_pos {
        println!("{word}: {pos}");
    }
}

fn parse_config(args: &[String]) -> String {
    let file_path = args[1].clone();
    file_path
}

fn insert_into_hm(hm: &mut HashMap<String, String>, word: &str, doc: &String) {
    hm.insert(
        word.to_owned(),
        doc.to_owned(),
    );
}
