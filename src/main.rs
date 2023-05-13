use galeshapley::{GaleShapley, Man, Stats, Woman};

use std::{collections::HashMap, sync::mpsc::Receiver};

#[derive(PartialEq, Debug)]
pub struct PrefWithNames {
    men_preferences: Vec<Vec<Woman>>,
    women_preferences: Vec<Vec<Man>>,

    men_names: Vec<String>,
    women_names: Vec<String>,
}

/// Parse a stable mariage problem from a textual representation
pub fn parse_input<R: std::io::BufRead>(r: R) -> PrefWithNames {
    let lines = r.lines().map(|line| line.unwrap());

    let mut man_names: HashMap<String, Man> = HashMap::new();
    let mut woman_names: HashMap<String, Woman> = HashMap::new();

    let mut men_preferences: Vec<Vec<Woman>> = vec![];
    let mut women_preferences: Vec<Vec<Man>> = vec![];

    let mut n = 0;

    for (i, line) in lines.enumerate() {
        if line.trim().is_empty() || n > 0 && i >= 2 * n {
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

fn invert_map(h: HashMap<String, usize>) -> Vec<String> {
    let mut names = vec!["".into(); h.len()];
    for (name, index) in h {
        names[index] = name;
    }
    names
}

fn run_from_parsed_stdin_problem() {
    let p = parse_input(std::io::stdin().lock());
    let mut algo: GaleShapley = GaleShapley::init(p.men_preferences, p.women_preferences);
    for (man, woman) in algo.find_stable_marriage() {
        println!("{}: {}", p.men_names[man], p.women_names[woman]);
    }
}

fn run_random(n: usize) {
    println!("Solving problems with {n} men and {n} women with random preferences.");
    println!("Success rate for the first man (got first choice / total samples) and 95% confidence interval :");
    let mut got_first_choice = 0;
    for total_tries in 1.. {
        let mut pb = GaleShapley::init_random(n);
        let preferred_woman = pb.best_woman_for(0);
        got_first_choice += pb.has_stable_mariage_with(0, preferred_woman) as usize;
        let rate = got_first_choice as f64 / total_tries as f64;
        let confidence = 100. * 1.96 * (rate * (1. - rate) / total_tries as f64).sqrt();
        let percentage = 100. * rate;
        print!(
            "\r{got_first_choice:^9}/{total_tries:^9} = {percentage:^6.2} ± {confidence:^4.1} %\r"
        )
    }
}

fn run_stats(n: usize) -> Stats {
    let stats = Stats::new(n);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                for _ in 0..10 {
                    let pb = GaleShapley::init_random(n);
                    stats.add_problem(pb);
                }
            });
        }
    });
    stats
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.get(1) == Some(&"stats".to_string()) {
        let n: usize = argv[2].parse().expect("invalid number");
        let stats = run_stats(n);
        println!("rank,men,women");
        for i in 0..n {
            println!("{},{:?},{:?}", i + 1, stats.men[i], stats.women[i]);
        }
    } else if argv.get(1) == Some(&"all".to_string()) {
        let n: usize = argv[2].parse().expect("invalid number");
        print_all_problems(n)
    } else if argv.len() == 2 {
        let n: usize = argv[1].parse().expect("invalid number");
        run_random(n)
    } else {
        run_from_parsed_stdin_problem()
    }
}

fn print_all_problems(size: usize) {
    for i in 1..=size {
        for j in 1..=size {
            print!("man_{}_preference_{},", i, j);
        }
    }
    for i in 1..=size {
        for j in 1..=size {
            print!("woman_{}_preference_{},", i, j);
        }
    }
    for i in 1..=size {
        print!("woman_{}_mariage,", i);
    }
    println!();
    for r in solve_all_problems(size) {
        for i in 0..size {
            for j in 0..size {
                print!("{},", r.men_preferences[i][j]);
            }
        }
        for i in 0..size {
            for j in 0..size {
                print!("{},", r.women_preferences[i][j]);
            }
        }
        for (i, &(m, w)) in r.mariages.iter().enumerate() {
            assert_eq!(i, w);
            print!("{},", m);
        }
        println!();
    }
}

struct GaleResult {
    men_preferences: Vec<Vec<usize>>,
    women_preferences: Vec<Vec<usize>>,
    mariages: Vec<(usize, usize)>,
}

impl From<(Vec<Vec<usize>>, Vec<Vec<usize>>)> for GaleResult {
    fn from(pb: (Vec<Vec<usize>>, Vec<Vec<usize>>)) -> Self {
        Self {
            men_preferences: pb.0.clone(),
            women_preferences: pb.1.clone(),
            mariages: GaleShapley::init(pb.0, pb.1)
                .find_stable_marriage()
                .collect(),
        }
    }
}

fn solve_all_problems(size: usize) -> Receiver<GaleResult> {
    let (snd, rcv) = std::sync::mpsc::channel();
    let (prefs_send, prefs_rcv) = std::sync::mpsc::sync_channel(8);
    std::thread::spawn(move || {
        for men_preferences in all_possible_preferences(size, true) {
            for women_preferences in all_possible_preferences(size, false) {
                prefs_send
                    .send((men_preferences.clone(), women_preferences))
                    .unwrap()
            }
        }
    });
    std::thread::spawn(move || {
        for pb in prefs_rcv {
            snd.send(GaleResult::from(pb)).unwrap();
        }
    });
    rcv
}

fn all_possible_preferences(
    size: usize,
    fixed_first: bool,
) -> impl Iterator<Item = Vec<Vec<usize>>> {
    let mut p: Box<dyn Iterator<Item = Vec<Vec<usize>>>> = if fixed_first {
        Box::new(std::iter::once(vec![(0..size).collect()]))
    } else {
        Box::new(all_possible_individual_preferences(size).map(|p| vec![p]))
    };
    for _ in 1..size {
        p = Box::new(p.flat_map(move |v| {
            all_possible_individual_preferences(size).map(move |p| {
                let mut new_v = v.clone();
                new_v.push(p);
                new_v
            })
        }))
    }
    p
}

/// Recursive function that returns an iterator over all possible orderings of `size` elements.
fn all_possible_individual_preferences(size: usize) -> Box<dyn Iterator<Item = Vec<usize>>> {
    if size == 0 {
        return Box::new(std::iter::once(vec![]));
    }
    Box::new(
        all_possible_individual_preferences(size - 1).flat_map(move |v| {
            (0..size).map(move |i| {
                let mut new_v = Vec::with_capacity(size);
                new_v.extend_from_slice(&v[..i]);
                new_v.push(size - 1);
                new_v.extend_from_slice(&v[i..]);
                new_v
            })
        }),
    )
}

#[test]
fn test_all_possible() {
    let mut all = all_possible_individual_preferences(3).collect::<Vec<_>>();
    all.sort();
    assert_eq!(
        all,
        vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0]
        ]
    );
}

#[test]
fn test_all_possible_aggregated_preferences() {
    let mut all = all_possible_preferences(2, false).collect::<Vec<_>>();
    all.sort();
    assert_eq!(
        all,
        vec![
            vec![vec![0, 1], vec![0, 1]],
            vec![vec![0, 1], vec![1, 0]],
            vec![vec![1, 0], vec![0, 1]],
            vec![vec![1, 0], vec![1, 0]]
        ]
    );
}

#[test]
#[should_panic]
fn test_input_parsing() {
    parse_input(&b"nawak"[..]);
}

#[test]
fn test_parse() {
    let input = b"A: X Y \n\
                            B: Y X \n\
                            X: A B \n\
                            Y: B A";
    let p = parse_input(&input[..]);
    assert_eq!(
        p,
        PrefWithNames {
            men_preferences: vec![vec![0, 1], vec![1, 0]],
            women_preferences: vec![vec![0, 1], vec![1, 0]],
            men_names: vec!["A".into(), "B".into()],
            women_names: vec!["X".into(), "Y".into()]
        }
    );
}
