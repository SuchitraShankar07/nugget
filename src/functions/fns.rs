
use nucleo::{Matcher as crateMatcher, Utf32Str};
use crate::Contact;
pub trait Matcher {
    fn find_matches(&mut self, input: &str, contacts: &[Contact]) -> Vec<(String, u16)>;
}
use crate::map_vals;
pub fn make_names(seq: &str) ->Vec<String>{
    let mut poss = vec!["".to_string()];
    for digit in seq.chars(){
        if let Some(alphabets) = map_vals(digit){
            let mut new_poss=Vec :: new();
            for possibility in &poss {
                    for &letter in alphabets
                    {
                        // new_poss.push(format!("{}{}",possibility,letter));
                        new_poss.push(possibility.clone() + &letter.to_string());

                    }
            }
            poss = new_poss;
        }
    }
    poss
}


pub struct FuzzyMatcher {
    matcher: crateMatcher,
}

impl FuzzyMatcher{
    pub fn new() -> Self {
        Self {
            matcher: crateMatcher::default(),
        }
    }
}

pub struct GreedyMatcher {
    matcher: crateMatcher,
}

impl GreedyMatcher {
    pub fn new() -> Self {
        Self {
            matcher: crateMatcher::default(),
        }
    }
}

impl crate::Matcher for GreedyMatcher {
    fn find_matches(&mut self, input: &str, contacts: &[Contact]) -> Vec<(String, u16)> {
        let mut input_buf = input.chars().collect::<Vec<char>>(); //fromstr needs mutable buffer it seems
        // let needle = Utf32Str::from_str(input);
        let needle = Utf32Str::new(input, &mut input_buf); 
        //as i understand it, utf32str reqs a mutable buffer, and not just a str, so we use new constr and pass input along with a mutable buffer

        let mut results = vec![];

        for contact in contacts {
            let mut name_buf = contact.name.chars().collect::<Vec<char>>();
            let haystack = Utf32Str::new(&contact.name, &mut name_buf);
            if let Some(score) = self.matcher.fuzzy_match_greedy(haystack, needle) {
                results.push((contact.name.clone(), score));
            }
        }

        // results.sort_by(|a, b| b.1.cmp(&a.1)); //manual, inefficient
        results.sort_unstable_by_key(|(_, score)| -(*score as isize)); //faster i hear

        results
    }
}
impl crate::Matcher for FuzzyMatcher {
    fn find_matches(&mut self, input: &str, contacts: &[Contact]) -> Vec<(String, u16)> {
        let mut input_buf = input.chars().collect::<Vec<char>>(); //fromstr needs mutable buffer it seems
        // let needle = Utf32Str::from_str(input);
        let needle = Utf32Str::new(input, &mut input_buf); 
        //as i understand it, utf32str reqs a mutable buffer, and not just a str, so we use new constr and pass input along with a mutable buffer

        let mut results = vec![];

        for contact in contacts {
            let mut name_buf = contact.name.chars().collect::<Vec<char>>();
            let haystack = Utf32Str::new(&contact.name, &mut name_buf);
            if let Some(score) = self.matcher.fuzzy_match(haystack, needle) {
                results.push((contact.name.clone(), score));
            }
        }

        // results.sort_by(|a, b| b.1.cmp(&a.1)); //manual, inefficient
        results.sort_unstable_by_key(|(_, score)| -(*score as isize)); //faster i hear

        results
    }
}

// pub enum MatchMode{
//     Fuzzy,
//     Greedy,
// }
// pub struct Matcher {
//     matcher: crateMatcher,
//     mode: MatchMode,
// }
// impl Matcher {
//     pub fn new(mode: MatchMode) -> Self {
//         Self {
//             matcher: crateMatcher::default(),
//             mode,
//         }
//     }
// }
// pub fn find_matches(&mut self, input:&str, contacts: &[Contact])->Vec<(String, u16)>{
// let needle = Utf32Str::from_str(input);

// }