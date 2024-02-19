use std::collections::HashMap;

use crate::{
    tokenizers::basic::Tokenizer,
    util::{get_stats, merge},
    TokenizerTrait,
};

use regex::Regex;

const GPT4_SPLIT_PATTERN: &str = r#"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?\p{L}+|\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]|\s+"#;

struct RegexTokenizer {
    tokenizer: Tokenizer,
    compiled_pattern: Regex,
}

impl RegexTokenizer {
    pub fn new() -> Self {
        let mut tokenizer = Tokenizer::new();
        tokenizer.pattern = GPT4_SPLIT_PATTERN.to_string();
        let compiled_pattern = Regex::new(GPT4_SPLIT_PATTERN).unwrap();

        RegexTokenizer { tokenizer, compiled_pattern }
    }

    pub fn encode_chunk(&self, chunk: &str) -> Vec<u32> {
        let mut chunk_ids: Vec<u32> = chunk.bytes().map(|m| m as u32).collect();
        let mut i = 0;
        while i + 1 < chunk_ids.len() {
            let pair = (chunk_ids[i], chunk_ids[i + 1]);
            if let Some(&new_id) = self.tokenizer.merges.get(&pair) {
                chunk_ids[i] = new_id; // Replace the pair with the new_id
                chunk_ids.remove(i + 1); // Remove the second part of the pair
                                         // Do not increment i, to check the new pair formed with the new_id
            } else {
                i += 1;
            }
        }
        chunk_ids
    }
}

impl TokenizerTrait for RegexTokenizer {
    fn train(&mut self, text: &str, vocab_size: u32, verbose: bool) {
        assert!(vocab_size >= 256);
        let num_merges = vocab_size - 256;
        let text_chunks: Vec<&str> =
            self.compiled_pattern.find_iter(text).map(|m| m.as_str()).collect();
        let mut ids: Vec<Vec<u32>> = text_chunks
            .iter()
            .map(|&chunk| chunk.as_bytes().iter().map(|&b| b as u32).collect())
            .collect();
        for i in 0..num_merges {
            let mut stats = HashMap::new();
            for chunk_ids in &ids {
                let chunk = get_stats(chunk_ids);
                for (pair, count) in chunk {
                    *stats.entry(pair).or_insert(0) += count;
                }
            }
            if let Some(pair) = self.tokenizer.find_most_frequent_pair(&stats) {
                let new_id = 256 + i as u32;
                ids = ids.into_iter().map(|chunk_ids| merge(chunk_ids, pair, new_id)).collect();
                self.tokenizer.merges.insert(pair, new_id);
                let first_part = self.tokenizer.vocab.get(&pair.0).unwrap_or(&vec![]).clone();
                let second_part = self.tokenizer.vocab.get(&pair.1).unwrap_or(&vec![]).clone();
                self.tokenizer.vocab.insert(new_id, [first_part, second_part].concat());

                if verbose {
                    println!(
                        "merge {}/{}: {:?} -> {} had {} occurrences",
                        i + 1,
                        vocab_size - 256,
                        pair,
                        new_id,
                        stats[&pair]
                    );
                }
            } else {
                break;
            }
        }
    }

    fn encode(&self, text: &str) -> Vec<u32> {
        let text_chunks: Vec<&str> =
            self.compiled_pattern.find_iter(text).map(|m| m.as_str()).collect();

        let mut ids: Vec<u32> = Vec::new();

        for chunk in &text_chunks {
            let chunk_ids = self.encode_chunk(chunk);
            ids.extend(chunk_ids);
        }
        ids
    }

    fn decode(&self, ids: &[u32]) -> String {
        self.tokenizer.decode(ids)
    }

    fn save(&self, file_prefix: &str) -> std::io::Result<()> {
        self.tokenizer.save(file_prefix)
    }

    fn load(&mut self, model_file: &str) -> std::io::Result<()> {
        self.tokenizer.load(model_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let test_strings = ["", "?", "hello world!!!? (안녕하세요!) lol123 😉"];
        let tokenizer = RegexTokenizer::new();
        for test_string in test_strings {
            let ids = tokenizer.encode(test_string);
            let decoded = tokenizer.decode(&ids);
            assert_eq!(test_string, decoded);
        }
    }
}
