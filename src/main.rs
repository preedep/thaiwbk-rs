use std::fs::read_to_string;
use std::time::Instant;
use log::debug;

use crate::trie::thai_word_trie::TrieNode;

mod trie;

fn main() {
    pretty_env_logger::init();
    debug!("Starting up");
    let filename = "lexitron.txt";
    let mut trie = TrieNode::new();

    let before = Instant::now();
    for line in read_to_string(filename).unwrap().lines() {
        trie.insert(&line.to_string());
    }
    debug!("Elapsed insert time: {:.2?}", before.elapsed());

    let before = Instant::now();
    let results = trie.words_with_prefix(&"ขนม".to_string());
    let elapsed = before.elapsed();

    for result in results {
        println!("{}", result);
    }

    debug!("Elapsed words_with_prefix time: {:.2?}", elapsed);
}
