use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use crate::util::render_token;
use crate::TokenizerTrait;

struct Tokenizer {
    merges: HashMap<(u32, u32), u32>,
    vocab: HashMap<u32, Vec<u8>>,
    pattern: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            merges: HashMap::new(),
            vocab: (0..256).map(|idx| (idx, vec![idx as u8])).collect(),
            pattern: String::new(),
        }
    }

    pub fn build_vocab(&mut self) {
        self.vocab.clear();
        for idx in 0..256 {
            self.vocab.insert(idx, vec![idx as u8]);
        }

        let mut next_idx = 256;
        for &(idx1, idx2) in self.merges.keys() {
            if let (Some(token1), Some(token2)) = (self.vocab.get(&idx1), self.vocab.get(&idx2)) {
                let mut new_token = token1.clone();
                new_token.extend(token2);
                self.vocab.insert(next_idx, new_token);
                next_idx += 1;
            }
        }
    }
}

impl TokenizerTrait for Tokenizer {
    fn train(&mut self, text: &str, vocab_size: u32, verbose: bool) {
        unimplemented!()
    }

    fn encode(&self, text: &str) -> Vec<u32> {
        unimplemented!()
    }

    fn decode(&self, ids: &[u32]) -> String {
        unimplemented!()
    }

    fn save(&self, file_prefix: &str) -> io::Result<()> {
        let model_file_path = format!("{}.model", file_prefix);
        let vocab_file_path = format!("{}.vocab", file_prefix);

        let mut model_file = File::create(model_file_path)?;
        writeln!(model_file, "{}", self.pattern)?;
        for (&(idx1, idx2), &idx) in &self.merges {
            writeln!(model_file, "{} {}", idx1, idx2)?;
        }

        let mut vocab_file = File::create(vocab_file_path)?;
        for (&idx, token) in &self.vocab {
            // Handling of rendering tokens would go here
            // This example skips over complex Unicode handling for brevity
            let token_string = render_token(token);
            writeln!(vocab_file, "{} [{}]", idx, token_string)?;
        }

        Ok(())
    }

    fn load(&mut self, model_file: &str) -> io::Result<()> {
        assert!(model_file.ends_with(".model"));
        let file = File::open(model_file)?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines();

        if let Some(first_line) = lines.next() {
            self.pattern = first_line?.trim().to_string();
        }
        let mut merges = HashMap::new();
        let mut idx = 256;

        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(idx1), Ok(idx2)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    merges.insert((idx1, idx2), idx);
                    idx += 1;
                }
            }
        }
        self.merges = merges;
        self.build_vocab();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile::{tempdir, TempDir};

    fn create_temp_tokenizer() -> Tokenizer {
        let mut tokenizer = Tokenizer::new();
        tokenizer.merges.insert((1, 2), 256);
        tokenizer.pattern = "some pattern".into();
        tokenizer
    }

    #[test]
    fn test_new_tokenizer() {
        let tokenizer = Tokenizer::new();
        assert!(tokenizer.merges.is_empty());
        assert_eq!(tokenizer.vocab.len(), 256);
        assert!(tokenizer.pattern.is_empty());
    }

    #[test]
    fn test_build_vocab_basic() {
        let mut tokenizer = Tokenizer::new();
        tokenizer.build_vocab();
        assert_eq!(tokenizer.vocab.len(), 256);
    }

    #[test]
    fn test_save_and_load() -> io::Result<()> {
        let temp_dir = tempdir()?;

        let file_prefix = temp_dir.path().join("tokenizer_test");
        let tokenizer = create_temp_tokenizer();
        tokenizer.save(file_prefix.to_str().unwrap()).unwrap();

        let mut load_tokenizer = Tokenizer::new();
        load_tokenizer
            .load(file_prefix.with_extension("model").to_str().unwrap())
            .unwrap();
        assert_eq!(load_tokenizer.merges, tokenizer.merges);
        assert_eq!(load_tokenizer.pattern, tokenizer.pattern);

        Ok(())
    }
}
