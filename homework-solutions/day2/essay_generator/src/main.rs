use trie_rs::Trie;
use trie_rs::TrieBuilder;
use rand::seq::SliceRandom;
use std::env;
use std::fs;

type Graph = Trie<String>;
type Phrase = Vec<Vec<String>>;

fn read_sentences(filename: &str) -> Phrase {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut sentences = Vec::new();
    for line in contents.split(".") {
        let mut phrase = Vec::new();
        for word in line.split_whitespace() {
            let word = word.trim().trim_end_matches(|c| char::is_ascii_punctuation(&c)).to_lowercase();
            if !word.is_empty() {
                phrase.push(word);
            }
        }
        if !phrase.is_empty() {
            sentences.push(phrase);
        }
    }
    sentences
}

fn construct_graph(sentences: &Phrase) -> (Vec<String>, Graph) {
    let mut builder = TrieBuilder::new();
    let mut roots = Vec::new();
    for sentence in sentences.iter() {
        roots.push(sentence[0].clone());
        builder.push(sentence);
    }
    (roots, builder.build())
}

fn wordcount(words: &String) -> usize {
    words.split_whitespace().count()
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn choose_options<T>(list: &[T]) -> &T {
    list.choose(&mut rand::thread_rng()).unwrap()
}

fn make_essay(root_words: Vec<String>, graph: Graph, length: usize) -> String {
    let mut essay = String::from("");
    while wordcount(&essay) < length {
        // random walk down the graph for another sentence
        let word = choose_options(&root_words);
        let results = graph.predictive_search(vec![word.clone()]);
        let sentence = choose_options(&results);
        essay += uppercase_first_letter(&sentence.join(" ")).as_str();
        essay += ". ";
    }
    essay
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Args: sampletext");
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut essay_length = 500;
    if args.len() > 2 {
        essay_length = args[2].parse().unwrap();
    }

    let sentences = read_sentences(filename);

    let (root_words, graph) = construct_graph(&sentences);
    let essay = make_essay(root_words, graph, essay_length);
    println!("{}", essay);
}
