mod vocabulary_filtered;
mod vocabulary;

use std::collections::HashMap;
use std::io::{self, Read};
use clap::Parser;

/// Emoji encoder that converts text to emojis
#[derive(Parser, Debug)]
#[command(name = "emojicoder")]
#[command(about = "A CLI tool to convert text to emojis", long_about = None)]
struct Args {
    /// Use the full vocabulary instead of the filtered one
    #[arg(short, long)]
    full: bool,
}

// Linguistic categories for filtering
#[derive(Debug, Clone, Copy, PartialEq)]
enum WordCategory {
    ConcreteNoun,    // Physical objects, animals, people
    ActionVerb,      // Actions that can be visualized
    VisualAdjective, // Properties that can be seen/felt
    Essential,       // Critical grammar words (I, you, is, etc.)
    Spatial,         // Direction, location, time
    Skip,            // Abstract words that don't translate well
}

fn get_word_category(word: &str) -> WordCategory {
    // Essential pronouns and basic grammar
    let essential_words = [
        "i", "you", "we", "he", "she", "they", "me", "us", "my", "our",
        "is", "are", "am", "was", "were", "have", "has", "had",
        "this", "that", "here", "there", "now", "yes", "no"
    ];
    
    // Abstract concepts that don't visualize well
    let abstract_skip = [
        "so", "been", "told", "some", "choose", "believe", "know", "wrong",
        "connection", "and", "the", "a", "an", "to", "from",
        "it", "of", "for", "by", "with", "as", "than", "but", "or", "if",
        "when", "while", "because", "though", "just", "only", "even", "still",
        "also", "too", "very", "really", "much", "more", "less", "most", "least",
        "will", "would", "could", "should", "must", "might", "may", "can",
        "well", "find", "think", "believe", "feel", "seem", "become", "make",
        "get", "go", "come", "say", "tell", "ask", "answer", "call", "try",
        "use", "put", "keep", "let", "turn", "start", "stop", "end", "begin"
    ];
    
    // Concrete visual nouns (we'll check our filtered vocab)
    let concrete_nouns = [
        "cat", "dog", "bird", "tree", "house", "car", "sun", "moon", "book",
        "food", "water", "fire", "mountain", "river", "flower", "star", "man",
        "woman", "child", "phone", "computer", "door", "table", "chair"
    ];
    
    // Action verbs that are easily visualized
    let action_verbs = [
        "run", "jump", "eat", "sleep", "dance", "swim", "fly", "walk",
        "sit", "stand", "look", "see", "hear", "love", "help", "build",
        "read", "write", "draw", "sing", "cook", "drive", "talk", "meet"
    ];
    
    // Visual adjectives and emotions
    let visual_descriptors = [
        "big", "small", "red", "blue", "happy", "sad", "hot", "cold",
        "fast", "slow", "beautiful", "strong", "young", "old", "tall",
        "short", "angry", "excited", "tired", "scared", "green", "yellow"
    ];
    
    // Spatial and temporal concepts
    let spatial_temporal = [
        "up", "down", "left", "right", "in", "out", "on", "under",
        "before", "after", "tomorrow", "yesterday", "morning", "night",
        "day", "time", "here", "there", "near", "far", "inside", "outside"
    ];
    
    if essential_words.contains(&word) {
        WordCategory::Essential
    } else if abstract_skip.contains(&word) {
        WordCategory::Skip
    } else if concrete_nouns.contains(&word) {
        WordCategory::ConcreteNoun
    } else if action_verbs.contains(&word) {
        WordCategory::ActionVerb
    } else if visual_descriptors.contains(&word) {
        WordCategory::VisualAdjective
    } else if spatial_temporal.contains(&word) {
        WordCategory::Spatial
    } else {
        // Default decision based on word characteristics
        if word.ends_with("ing") || word.ends_with("ed") {
            WordCategory::ActionVerb
        } else if word.ends_with("ly") {
            WordCategory::Skip // Most adverbs are abstract
        } else {
            WordCategory::Skip // Unknown words default to skip
        }
    }
}

fn should_include_word(word: &str, vocab: &HashMap<&str, &str>) -> bool {
    // Check if we have a mapping through normalization AND it's in a good category
    if find_word_in_vocab(word, vocab).is_none() {
        return false;
    }
    
    // Check category using the original word, but also check normalized forms
    let candidates = normalize_word(word);
    for candidate in &candidates {
        let category = get_word_category(candidate);
        if !matches!(category, WordCategory::Skip) {
            return true;
        }
    }
    
    false
}

fn normalize_word(word: &str) -> Vec<String> {
    let mut candidates = vec![word.to_string()];
    
    // Simple plural reduction - just strip common plural endings
    if word.len() > 2 {
        // Remove -s endings (most common plurals)
        if word.ends_with("s") && !word.ends_with("ss") {
            candidates.push(word[..word.len()-1].to_string());
        }
        
        // Remove -es endings
        if word.ends_with("es") {
            candidates.push(word[..word.len()-2].to_string());
        }
        
        // Remove -ies endings (but keep the 'i' as it might be part of the root)
        if word.ends_with("ies") {
            candidates.push(word[..word.len()-3].to_string());
            candidates.push(format!("{}y", &word[..word.len()-3]));
        }
        
        // Remove -ves endings
        if word.ends_with("ves") {
            candidates.push(word[..word.len()-3].to_string());
        }
    }
    
    // Simple verb form reduction - strip common endings
    if word.len() > 3 {
        // Remove -ing endings
        if word.ends_with("ing") {
            candidates.push(word[..word.len()-3].to_string());
        }
        
        // Remove -ed endings
        if word.ends_with("ed") {
            candidates.push(word[..word.len()-2].to_string());
        }
    }
    
    candidates
}

fn find_word_in_vocab<'a>(word: &str, vocab: &'a HashMap<&str, &str>) -> Option<&'a str> {
    let candidates = normalize_word(word);
    
    for candidate in &candidates {
        if vocab.contains_key(candidate.as_str()) {
            return vocab.get(candidate.as_str()).copied();
        }
    }
    
    None
}

fn compose_unknown(word: &str, vocab: &HashMap<&str, &str>) -> Option<String> {
    // First try to find the word through normalization
    if let Some(emoji) = find_word_in_vocab(word, vocab) {
        return Some(add_spaces_between_emojis(emoji));
    }
    
    // Try to split the word into two parts where both are in vocab AND should be included
    for i in 1..word.len() {
        let prefix = &word[0..i];
        let suffix = &word[i..];
        if should_include_word(prefix, vocab) && should_include_word(suffix, vocab) {
            if let (Some(&p_emoji), Some(&s_emoji)) = (vocab.get(prefix), vocab.get(suffix)) {
                return Some(format!("{} {}", add_spaces_between_emojis(p_emoji), add_spaces_between_emojis(s_emoji)));
            }
        }
    }
    None // Return None instead of ❓ for unknown words
}

fn add_spaces_between_emojis(s: &str) -> String {
    s.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    let args = Args::parse();
    
    // Choose vocabulary based on the flag
    let vocab = if args.full {
        vocabulary::get_vocabulary()
    } else {
        vocabulary_filtered::get_vocabulary()
    };
    
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    // Split by sentences first, then process each sentence
    let sentences: Vec<&str> = input.split(&['.', '!', '?', '\n'][..]).collect();
    
    let mut results = Vec::new();
    
    for sentence in sentences {
        if sentence.trim().is_empty() {
            continue;
        }
        
        let encoded: Vec<String> = sentence
            .split_whitespace()
            .filter_map(|word| {
                let clean_word: String = word
                    .chars()
                    .filter(|c| c.is_alphanumeric() || c == &'\'')
                    .collect::<String>()
                    .to_lowercase()
                    .replace("'", "")
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>();
                
                if args.full {
                    // For full vocabulary, include all words that have mappings
                    find_word_in_vocab(&clean_word, &vocab).map(|emoji| add_spaces_between_emojis(emoji))
                        .or_else(|| compose_unknown(&clean_word, &vocab))
                } else {
                    // For filtered vocabulary, only include words that should be visualized
                    if should_include_word(&clean_word, &vocab) {
                        find_word_in_vocab(&clean_word, &vocab).map(|emoji| add_spaces_between_emojis(emoji))
                    } else {
                        // Try composition for unknown words, but only if result would be meaningful
                        compose_unknown(&clean_word, &vocab)
                    }
                }
            })
            .collect();

        if !encoded.is_empty() {
            results.push(encoded.join(" "));
        }
    }
    
    // Join sentences with a sentence separator
    let result = if results.is_empty() {
        "".to_string()
    } else {
        results.join(" • ")
    };

    println!("{}", result);
}

