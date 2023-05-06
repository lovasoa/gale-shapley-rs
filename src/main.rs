use galeshapley::{GaleShapley, Man, Woman};

use std::collections::HashMap;
use std::io::{self, BufRead};

struct PrefWithNames {
    men_preferences: Vec<Vec<Woman>>,
    women_preferences: Vec<Vec<Man>>,

    men_names: HashMap<Man, String>,
    women_names: HashMap<Woman, String>,
}

fn parse_input() -> PrefWithNames {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let mut man_names: HashMap<String, Man> = HashMap::new();
    let mut woman_names: HashMap<String, Woman> = HashMap::new();

    let mut men_preferences: Vec<Vec<Woman>> = vec![];
    let mut women_preferences: Vec<Vec<Man>> = vec![];

    let mut n = 0;

    for (i, line) in lines.enumerate() {
        if line.trim().is_empty() || (n > 0 && i >= 2 * n) {
            break;
        }
        let (person, preferences) = parse_line(&line)
            .expect("malformed line. Expected \"PersonName: Preferred SecondPreferred ...\"");
        if n == 0 {
            n = preferences.len();
        } else {
            assert_eq!(
                n,
                preferences.len(),
                "Expected {n} preferences on each line"
            )
        }
        if i == 0 {
            woman_names = preferences
                .iter()
                .enumerate()
                .map(|(i, &woman_name)| (woman_name.into(), i))
                .collect();
        }
        if i < n {
            man_names.insert(person.into(), i);
            add_pref_line(&mut men_preferences, preferences, &woman_names);
        } else {
            add_pref_line(&mut women_preferences, preferences, &man_names);
        }
    }

    assert_eq!(
        men_preferences.len(),
        women_preferences.len(),
        "Expected as many men as women"
    );
    PrefWithNames {
        men_preferences,
        women_preferences,
        men_names: invert_map(man_names),
        women_names: invert_map(woman_names),
    }
}

fn parse_line(line: &str) -> Option<(&str, Vec<&str>)> {
    let mut parts = line.split(':');
    let person = parts.next()?.trim();
    let pref_list = parts.next()?.trim();
    let prefs: Vec<&str> = pref_list.split_whitespace().collect();
    Some((person, prefs))
}

fn add_pref_line(
    prefs: &mut Vec<Vec<usize>>,
    pref_names: Vec<&str>,
    name_lookup: &HashMap<String, usize>,
) {
    prefs.push(
        pref_names
            .into_iter()
            .map(|name| name_lookup[name])
            .collect(),
    )
}

fn invert_map(h: HashMap<String, usize>) -> HashMap<usize, String> {
    h.into_iter().map(|(a, b)| (b, a)).collect()
}

fn main() {
    let p = parse_input();
    let mut algo: GaleShapley = GaleShapley::init(p.men_preferences, p.women_preferences);
    for (man, woman) in algo.find_stable_marriage() {
        println!("{}: {}", p.men_names[&man], p.women_names[&woman]);
    }
}
