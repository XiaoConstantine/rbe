use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;
use std::{fs, io};

use clap::{App, Arg};
use rbpe::tokenizers::basic::Tokenizer;
use rbpe::tokenizers::regex::RegexTokenizer;
use rbpe::TokenizerTrait;

fn read_file_content(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let matches = App::new("CLI for tokenizer")
        .arg(
            Arg::with_name("tokenizer")
                .long("tokenizer")
                .value_name("TOKENIZER")
                .help("Choose a tokenizer for processing (e.g. basic, regex)")
                .takes_value(true),
        )
        .get_matches();

    let choices = matches.value_of("tokenizer").unwrap_or("regex");

    let mut tokenizer: Box<dyn TokenizerTrait> = match choices {
        "basic" => Box::new(Tokenizer::new()),
        "regex" => Box::new(RegexTokenizer::new()),
        _ => Box::new(RegexTokenizer::new()),
    };
    let training_input_path = "data/taylorswift.txt";
    let content = read_file_content(Path::new(training_input_path))?;
    fs::create_dir_all("models")?;

    // Time the performance
    let start = Instant::now();
    let file_prefix = Path::new("models").join(choices);

    tokenizer.train(&content, 512, true);
    if let Some(file_prefix_str) = file_prefix.to_str() {
        tokenizer.save(file_prefix_str)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Path contain invalid utf-8 characters",
        ));
    }
    let duration = start.elapsed();
    println!("Took {:.2}", duration.as_secs_f32());
    Ok(())
}
