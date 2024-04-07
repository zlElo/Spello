use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io;
use std::collections::HashSet;
use std::process;

fn main() {
    let file = File::open("wordlists/english_wordlist.txt").unwrap_or_else(|error| {
        eprintln!("Failed to open file: {}", error);
        process::exit(1);
    });
    let reader = BufReader::new(file);

    // Read word out
    println!("Enter a word:");
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap_or_else(|error| {
        eprintln!("Failed to read from stdin: {}", error);
        process::exit(1);
    });
    word = word.trim().to_lowercase();

    // Search word in file
    
    let found = search(reader, &word).unwrap_or_else(|error| {
        eprintln!("Failed to search for word: {}", error);
        process::exit(1);
    });

    // test if word is already in the list
    if found {
        // word already correct
        return
    } else {
        // search in tree for closer words
        let closer_word = deeper(&word).unwrap_or_else(|error| {
            eprintln!("Failed to find closer words: {}", error);
            process::exit(1);
        });
        println!("Did you mean: {:?}", closer_word);
    }
}

fn search(reader: BufReader<File>, word: &str) -> Result<bool> {
    let mut found = false;

    for line in reader.lines() {
        let line = line?;
        if line.trim().to_lowercase() == word {
            found = true;
            break;
        }
    }

    Ok(found)
}

fn deeper(word: &str) -> Result<HashSet<String>> {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut edits = HashSet::new();

    for i in 0..=word.len() {
        let (left, right) = word.split_at(i);
        if !right.is_empty() {
            edits.insert(format!("{}{}", left, &right[1..]));
        }
        for c in letters.chars() {
            if !right.is_empty() {
                edits.insert(format!("{}{}{}", left, c, &right[1..]));
            }
        }
    }

    for i in 0..word.len() - 1 {
        let (left, right) = word.split_at(i);
        if right.len() > 1 {
            edits.insert(format!("{}{}{}", left, &right[1..2], &right[0..1]));
        }
    }

    // Reading the file into a HashSet for faster lookup
    let mut wordlist = HashSet::new();
    let file = File::open("wordlists/english_wordlist.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        wordlist.insert(line);
    }

    let mut words_to_remove = Vec::new();

    for word in &edits {
        if !wordlist.contains(word) {
            words_to_remove.push(word.clone());
        }
    }

    for word in words_to_remove {
        edits.remove(&word);
    }

    Ok(edits)

}
