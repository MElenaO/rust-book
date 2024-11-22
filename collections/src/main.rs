use std::collections::HashMap;

fn pig_latin(word: String) -> String {
    let first = &word[0..1];
    let mut pig_word = word.clone();
    if (first != "a") && (first != "e") && (first != "i") && (first != "o") && (first != "u") {
        pig_word = word[1..word.len()].to_string();
        pig_word.push_str("-");
        pig_word.push_str(first)
    } else {
        pig_word.push_str("-h")
    }
    pig_word.push_str("ay");
    pig_word
}

fn main() {

    let mut intArray = vec![5, 6, 1, 4, 9, 0, 9, 1, 1];
    intArray.sort();
    println!("{intArray:?}");
    let median = &intArray[intArray.len()/2];
    println!("median: {median}");

    let mut map = HashMap::new();
    for i in intArray {
        let count = map.entry(i).or_insert(0);
        *count +=1;
    }
    println!("{map:?}");

    let mut mode = -1;
    let mut ocurrence = 0;
    for (key, value) in map {
        if value > ocurrence {
            ocurrence = value;
            mode = key;
        }
    }
    println!("mode: {mode}");

    // pig latin

    let word1 = "first".to_string();
    println!("pig lating word: {}", pig_latin(word1));
    let word2 = "apple".to_string();
    println!("pig lating word: {}", pig_latin(word2));

}
