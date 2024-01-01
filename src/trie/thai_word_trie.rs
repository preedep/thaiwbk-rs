use std::collections::HashMap;

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
    pub fn insert(&mut self, word: &String) {
        let mut current_node = self;
        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_insert(TrieNode::new());
        }
        current_node.is_word = true;
    }
    pub fn search(&self, word: &String) -> bool {
        let mut current_node = self;
        for ch in word.chars() {
            match current_node.children.get(&ch) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.is_word
    }
    pub fn words_with_prefix(&self, prefix: &String) -> Vec<String> {
        let mut current_node = self;
        for ch in prefix.chars() {
            match current_node.children.get(&ch) {
                Some(node) => current_node = node,
                None => return vec![],
            }
        }
        let mut words = vec![];
        current_node.words_with_prefix_helper(prefix, &mut words);
        words
    }
    pub fn words_with_suffix(&self, suffix: &String) -> Vec<String> {
        let mut current_node = self;
        for ch in suffix.chars().rev() {
            match current_node.children.get(&ch) {
                Some(node) => current_node = node,
                None => return vec![],
            }
        }
        let mut words = vec![];
        current_node.words_with_suffix_helper(suffix, &mut words);
        words
    }
    fn words_with_suffix_helper(&self, suffix: &String, words: &mut Vec<String>) {
        if self.is_word {
            words.push(suffix.chars().rev().collect());
        }
        for (ch, node) in &self.children {
            let mut suffix = suffix.clone();
            suffix.push(*ch);
            node.words_with_suffix_helper(&suffix, words);
        }
    }
    fn words_with_prefix_helper(&self, prefix: &String, words: &mut Vec<String>) {
        if self.is_word {
            words.push(prefix.clone());
        }
        for (ch, node) in &self.children {
            let mut prefix = prefix.clone();
            prefix.push(*ch);
            node.words_with_prefix_helper(&prefix, words);
        }
    }
}

