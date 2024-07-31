use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

#[derive(Debug, Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }
        current.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_end_of_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("测试");
        trie.insert("字典树");
        trie.insert("树");

        assert!(trie.search("测试"));
        assert!(trie.search("字典树"));
        assert!(trie.search("树"));
        assert!(!trie.search("字"));
        assert!(!trie.search("测试字典树"));
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("测试");
    trie.insert("字典树");
    trie.insert("树");

    println!("搜索 '测试': {}", trie.search("测试"));
    println!("搜索 '字典树': {}", trie.search("字典树"));
    println!("搜索 '树': {}", trie.search("树"));
    println!("搜索 '字': {}", trie.search("字"));
    println!("搜索 '测试字典树': {}", trie.search("测试字典树"));
}
