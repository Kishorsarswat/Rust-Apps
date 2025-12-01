use std::collections::HashSet;
use std::collections::HashMap;

pub fn are_anagrams(a:String, b:String) -> bool {
    
    if *a == *b{
        return false;
    }
    
    let mut map:HashMap<char, i32> = HashMap::new();
    
    for c in a.chars() {
         map.entry(c).and_modify(|count| *count += 1).or_insert(1); 
    }
    
    for c in b.chars() {
         map.entry(c).and_modify(|count| *count -= 1).or_insert(-1);
    }
    
    for (_,v) in map {
        if v != 0{
            println!{" false"};
            return false;
        }
    }
    
    true
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&str> = HashSet::new();
    for s in possible_anagrams {
        if are_anagrams(word.to_lowercase(), s.to_lowercase()){
            set.insert(s);
        }
    }
    set
}
