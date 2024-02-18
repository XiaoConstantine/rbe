use std::collections::HashMap;

pub fn get_stats(ids: &[u32]) -> HashMap<(u32, u32), u32> {
    let mut counts = HashMap::new();
    for window in ids.windows(2) {
        *counts.entry((window[0], window[1])).or_insert(0) += 1;
    }
    counts
}

pub fn merge(ids: Vec<u32>, pair: (u32, u32), idx: u32) -> Vec<u32> {
    let mut new_ids = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        if i + 1 < ids.len() && ids[i] == pair.0 && ids[i + 1] == pair.1 {
            new_ids.push(idx);
            i += 2;
        } else {
            new_ids.push(ids[i]);
            i += 1;
        }
    }
    new_ids
}

pub fn replace_control_characters(s: &str) -> String {
    s.chars()
        .map(|ch| {
            if ch.is_control() {
                format!("\\u{:04x}", ch as u32)
            } else {
                ch.to_string()
            }
        })
        .collect()
}

pub fn render_token(token: &[u8]) -> String {
    let mut result = String::new();
    for &byte in token {
        match byte {
            0x00..=0x1F | 0x7F => result.push_str(&format!("\\x{:02x}", byte)),
            _ => result.push(char::from(byte)),
        }
    }
    result
}
