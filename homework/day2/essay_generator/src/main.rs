use trie_rs::Trie;
use trie_rs::TrieBuilder;
use rand::seq::SliceRandom;

/// Construct a graph from a list of sentences
fn construct_graph(sentences: Vec<Vec<&str>>) -> (Vec<&str>, Trie<&str>) {
    let mut builder = TrieBuilder::new();
    let mut roots = Vec::new();
    for sentence in sentences.iter() {
        roots.push(sentence[0]);
        builder.push(sentence);
    }
    (roots, builder.build())
}

/// Choose one of the options from a list randomly
fn choose_options<T>(list: &[T]) -> &T {
    list.choose(&mut rand::thread_rng()).unwrap()
}


fn main() {
    let sentences = vec![
        vec!["the", "brown", "dog", "jumped"],
        vec!["the", "white", "rabbit", "ate", "a", "carrot"],
        vec!["the", "brown", "bear", "roared"],
        vec!["the", "white", "rabbit", "hid"],
    ];
    let (root_words, graph) = construct_graph(sentences);

    let results = graph.predictive_search(vec![root_words[0]]);
    println!("num results: {}", results.len());
    for result in results.iter() {
        println!("result: {}", result.join(" "));
    }
}
