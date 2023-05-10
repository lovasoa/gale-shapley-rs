use std::sync::atomic::AtomicUsize;

pub type Man = usize;
pub type Woman = usize;

pub struct GaleShapley {
    free_men: Vec<Man>,

    /// men_preferences[m][N-i] is the ith prefered woman of m
    men_preferences: Vec<Vec<Woman>>,

    /// women_preferences[w][m] is the rank of m in w's preferences
    women_preferences: Vec<Vec<usize>>,

    /// women_engagement[w] is the man w is currently engaged to
    women_engagement: Vec<Option<Man>>,
}

impl GaleShapley {
    pub fn init(men_preferences: Vec<Vec<Woman>>, women_preferences: Vec<Vec<Man>>) -> GaleShapley {
        let num_men = men_preferences.len();
        let num_women = women_preferences.len();
        assert_eq!(num_men, num_women);

        GaleShapley {
            free_men: (0..num_men).rev().collect(),
            men_preferences: make_men_preferences(men_preferences),
            women_preferences: make_rank_matrix(women_preferences),
            women_engagement: vec![None; num_women],
        }
    }

    ///Creates a random Gale Shapley instance with n men and women
    pub fn init_random(n: usize) -> GaleShapley {
        GaleShapley {
            free_men: (0..n).rev().collect(),
            men_preferences: rand_pref_matrix(n),
            women_preferences: rand_pref_matrix(n),
            women_engagement: vec![None; n],
        }
    }

    fn next_free_man(&self) -> Option<Man> {
        self.free_men.last().copied()
    }

    /// Returns the woman that m wants currently wants the most
    pub fn best_woman_for(&self, m: Man) -> Woman {
        self.men_preferences[m]
            .last()
            .copied()
            .expect("internal error: man has no more preferences")
    }

    /// Returns the woman that m wants currently wants the most
    fn take_best_woman_for(&mut self, m: Man) -> Woman {
        self.men_preferences[m]
            .pop()
            .expect("internal error: man has no more preferences")
    }

    /// Returns the man that w is engaged to
    pub fn current_woman_engagement(&self, w: Woman) -> Option<Man> {
        self.women_engagement[w]
    }

    /// Whether w prefers m1 over m2
    fn woman_prefers(&self, w: Woman, m1: Man, m2: Man) -> bool {
        let prefs = &self.women_preferences[w];
        prefs[m1] < prefs[m2]
    }

    /// marks m and w as engaged
    fn engage(&mut self, m: Man, w: Woman) {
        self.women_engagement[w] = Some(m);
        let popped = self.free_men.pop();
        debug_assert_eq!(popped, Some(m));
    }

    /// removes the engagement between m and the woman he was engaged to
    fn free_from_engagement(&mut self, m: Man) {
        self.free_men.push(m);
    }

    /// Tries to engage the next free man. If we have reached a stable state,
    /// returns None, otherwise return the (man, woman) couple that proposed
    pub fn next_engagement_round(&mut self) -> Option<(Man, Woman)> {
        let m = self.next_free_man()?;
        let w = self.take_best_woman_for(m);
        if let Some(m2) = self.current_woman_engagement(w) {
            if self.woman_prefers(w, m, m2) {
                // w prefers m over her current partner m2
                self.engage(m, w);
                self.free_from_engagement(m2);
            }
        } else {
            self.engage(m, w);
        }
        Some((m, w))
    }

    /// Returns the final stable marriage
    pub fn find_stable_marriage(&mut self) -> impl Iterator<Item = (Man, Woman)> + '_ {
        while let Some((_m, _w)) = self.next_engagement_round() {
            // println!("{_m} proposes to {_w}. Engagements: {:?}. Free men: {:?}", self.women_engagement, self.free_men)
        }
        self.women_engagement
            .iter()
            .enumerate()
            .map(|(w, option_m)| (option_m.unwrap(), w))
    }

    /// Whether m and w have a stable marriage in the solution that would be returned by find_stable_marriage
    /// This is faster than calling find_stable_marriage and checking if the result contains (m, w)
    pub fn has_stable_mariage_with(&mut self, man: Man, woman: Woman) -> bool {
        let mut was_engaged = self.women_engagement[woman] == Some(man);
        while let Some((_m, _w)) = self.next_engagement_round() {
            if self.women_engagement[woman] == Some(man) {
                was_engaged = true;
            } else if was_engaged {
                return false; // unengaged after being engaged
            }
        }
        was_engaged
    }

    /// Reconstitute a matrix such that `men_rank_matrix[m][w]` is the rank of w in m's preferences
    pub fn men_rank_matrix(&self) -> Vec<Vec<usize>> {
        let n = self.size();
        self.men_preferences
            .iter()
            .map(|line| {
                assert_eq!(line.len(), n, "problem already solved, preferences lost");
                let mut rank = vec![0; line.len()];
                for (i, w) in line.iter().enumerate() {
                    rank[*w] = n - i - 1;
                }
                rank
            })
            .collect()
    }

    /// `women_preferences[w][m]` is the rank of m in w's preferences
    pub fn women_preferences(&self) -> &Vec<Vec<usize>> {
        &self.women_preferences
    }

    /// Number of men and women
    pub fn size(&self) -> usize {
        self.men_preferences.len()
    }
}

/// men_preferences[m][N-i] is the ith prefered woman of m
fn make_men_preferences(mut p: Vec<Vec<Woman>>) -> Vec<Vec<Woman>> {
    let len = p.len();
    for line in &mut p {
        assert_eq!(line.len(), len);
        line.reverse()
    }
    p
}

/// Takes a preference matrix and returns a rank matrix
/// takes a matrix M where M[w][i] is the man at rank i in w's preferences
/// and returns the T such as T[w][m] is the rank of m in w's preferences
pub fn make_rank_matrix(mut p: Vec<Vec<Man>>) -> Vec<Vec<usize>> {
    let len = p.len();
    for line in &mut p {
        assert_eq!(line.len(), len);
        for (idx, m) in line.clone().iter().enumerate() {
            line[*m] = idx;
        }
    }
    p
}

fn rand_pref_matrix(n: usize) -> Vec<Vec<usize>> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| rand::seq::index::sample(&mut rng, n, n).into_vec())
        .collect()
}

pub struct Stats {
    // Number of times men got their Nth choice
    pub men: Vec<AtomicUsize>,
    // Number of times women got their Nth choice
    pub women: Vec<AtomicUsize>,
}

impl Stats {
    pub fn new(size: usize) -> Self {
        Self {
            men: (0..size).map(|_| AtomicUsize::new(0)).collect(),
            women: (0..size).map(|_| AtomicUsize::new(0)).collect(),
        }
    }
    pub fn add_problem(&self, mut pb: GaleShapley) {
        let men_preference_ranks = pb.men_rank_matrix();
        let _ = pb.find_stable_marriage();
        for w in 0..pb.size() {
            let m = pb
                .current_woman_engagement(w)
                .expect("problem should be solved");
            let m_rank = pb.women_preferences()[w][m];
            let w_rank = men_preference_ranks[m][w];
            self.women[m_rank].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.men[w_rank].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_stable_marriage_1x1() {
        assert_eq!(
            GaleShapley::init(vec![vec![0]], vec![vec![0]])
                .find_stable_marriage()
                .collect::<Vec<_>>(),
            vec![(0, 0)]
        );
    }
    #[test]
    fn test_find_stable_marriage_2x2() {
        let men_preferences = vec![vec![0, 1], vec![0, 1]]; // both men prefer the first woman
        let women_preferences = vec![vec![1, 0], vec![1, 0]]; // both women prefer the second man
        let expected = vec![(1, 0), (0, 1)]; // The most preferred man ends up with the most preferred woman
        let actual: Vec<(Man, Woman)> = GaleShapley::init(men_preferences, women_preferences)
            .find_stable_marriage()
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_stable_marriage_3x3() {
        let men_preferences = vec![vec![0, 1, 2], vec![2, 1, 0], vec![1, 2, 0]];
        let women_preferences = vec![vec![0, 2, 1], vec![2, 1, 0], vec![2, 0, 1]];
        let expected = vec![(0, 0), (2, 1), (1, 2)];
        let actual: Vec<(Man, Woman)> = GaleShapley::init(men_preferences, women_preferences)
            .find_stable_marriage()
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_make_women_preferences() {
        assert_eq!(
            make_rank_matrix(vec![vec![0, 2, 1], vec![2, 1, 0], vec![2, 0, 1]]),
            [[0, 2, 1], [2, 1, 0], [1, 2, 0]]
        )
    }

    #[test]
    fn test_rand() {
        let n = 100;
        let mut men = std::collections::HashSet::new();
        let mut women = std::collections::HashSet::new();
        for (m, w) in GaleShapley::init_random(n).find_stable_marriage() {
            men.insert(m);
            women.insert(w);
        }
        for i in 0..n {
            assert!(men.contains(&i));
            assert!(women.contains(&i));
        }
    }

    #[test]
    fn test_has_stable_marriage_with_2x2() {
        let men_preferences = || vec![vec![0, 1], vec![0, 1]]; // both men prefer the first woman
        let women_preferences = || vec![vec![1, 0], vec![1, 0]]; // both women prefer the second man
        assert!(
            !GaleShapley::init(men_preferences(), women_preferences())
                .has_stable_mariage_with(0, 0) // first man does not mary first woman
        );
        assert!(
            GaleShapley::init(men_preferences(), women_preferences()).has_stable_mariage_with(1, 0) // second man maries first woman
        );
        assert!(
            GaleShapley::init(men_preferences(), women_preferences()).has_stable_mariage_with(0, 1) // first man maries second woman
        );
        assert!(
            !GaleShapley::init(men_preferences(), women_preferences())
                .has_stable_mariage_with(1, 1) // second man does not mary second woman
        );
    }

    #[test]
    fn stats() {
        let men_preferences = || vec![vec![0, 1], vec![0, 1]]; // both men prefer the first woman
        let women_preferences = || vec![vec![1, 0], vec![1, 0]]; // both women prefer the second man
        let stats = Stats::new(2);
        stats.add_problem(GaleShapley::init(men_preferences(), women_preferences()));
        let o = std::sync::atomic::Ordering::Acquire;
        assert_eq!(stats.men[0].load(o), 1); // 1 man got his first choice (the second man)
        assert_eq!(stats.men[1].load(o), 1); // 1 man got his second choice (the first man got the second woman)
        assert_eq!(stats.women[0].load(o), 1);
        assert_eq!(stats.women[1].load(o), 1);
    }
}
