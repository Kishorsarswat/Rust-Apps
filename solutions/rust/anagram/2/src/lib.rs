use std::collections::HashSet;

pub fn are_anagrams(a:String, b:String) -> bool {
    if a == b{
        return false;
    }
    
    let mut a = a.chars().collect::<Vec<_>>();
    let mut b = b.chars().collect::<Vec<_>>();

    a.sort();
    b.sort();
    
    a == b
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
