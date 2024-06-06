use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut subset = HashSet::new();
    let target_upper = word.to_uppercase();
    let t_chars = target_upper.chars().collect::<Vec<char>>();
    
    for &candidate in possible_anagrams {
        let candidate_upper = candidate.to_uppercase();
        if candidate_upper == target_upper { continue; }

        let mut valid = true;
        let mut tmp = t_chars.clone();
        let c_chars = candidate_upper.chars().collect::<Vec<char>>();
        for c in c_chars {
            if !tmp.contains(&c) {
                valid = false;
                break;
            }
            if let Some(index) = tmp.iter().position(|&t_char| t_char == c) {
                tmp.remove(index);
            } else { break; }
        }
        if valid && tmp.iter().count() == 0 { subset.insert(candidate); }
    }

    subset
}
