use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let mut wordmap = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let (dir_path, word) = parse_config(&args);

    let paths = fs::read_dir(dir_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    for path in paths {
        let contents: String = fs::read_to_string(&path)
            .expect("Error reading file");

        for line in contents.lines() {
            let splitline = line.split([' ', ',', '.', '?', '!']);
            for word in splitline {
                if !word.is_empty() {
                    insert_into_hm(&mut wordmap, word, &path); 
                }
            }
        }
    }
    sort_and_dedup(&mut wordmap);
    get_occurences(&mut wordmap, word);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let file_path = &args[1];
    let word = &args[2]; 
    (file_path, word)
}

fn insert_into_hm(hm: &mut HashMap<String, Vec<String>>, word: &str, doc: &String) {
    hm.entry(word.to_owned()).or_insert(Vec::new()).push(doc.to_owned());
}

fn sort_and_dedup(hm: &mut HashMap<String, Vec<String>>) {
    for (_, pos) in hm {
        pos.sort();
        pos.dedup();
    }
}

fn get_occurences(hm: &mut HashMap<String, Vec<String>>, word: &str) {
    let opt = hm.get(word);
    if !opt.is_none() {
        let docs = opt.unwrap();
        for doc in docs {
            println!("{}", doc);
        }
    }
}
