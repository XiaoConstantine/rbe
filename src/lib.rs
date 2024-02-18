pub mod tokenizers;
pub mod util;

use std::io;

pub trait TokenizerTrait {
    fn train(&mut self, text: &str, vocab_size: u32, verbose: bool);
    fn encode(&self, text: &str) -> Vec<u32>;
    fn decode(&self, ids: &[u32]) -> String;
    fn save(&self, file_prefix: &str) -> io::Result<()>;
    fn load(&mut self, model_file: &str) -> io::Result<()>;
}
