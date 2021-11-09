use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

#[allow(dead_code)]
fn cin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

#[allow(dead_code)]
fn cin_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    cin()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn cin_vertical<T>(n: usize) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut res = Vec::new();
    for _ in 0..n {
        res.push(cin().parse().unwrap());
    }
    res
}

trait ToTuple<T> {
    fn to_2(self) -> (T, T);
    fn to_3(self) -> (T, T, T);
}

impl<T> ToTuple<T> for Vec<T>
where
    T: Clone,
{
    fn to_2(self) -> (T, T) {
        (self.get(0).unwrap().clone(), self.get(1).unwrap().clone())
    }

    fn to_3(self) -> (T, T, T) {
        (
            self.get(0).unwrap().clone(),
            self.get(1).unwrap().clone(),
            self.get(2).unwrap().clone(),
        )
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    struct WeaponLearning {
        id: usize,
        required_time: usize,
        pub required_weapon: Vec<usize>,
    }

    let n = cin().parse::<usize>().unwrap();
    let weapon_learnings = {
        let mut weapon_learings = HashMap::new();
        for i in 0..n {
            let (t, k, a) = {
                let t_k_a = cin_vec::<usize>();
                (t_k_a[0], t_k_a[1], t_k_a[2..].to_vec())
            };
            weapon_learings.insert(
                i + 1,
                WeaponLearning {
                    id: i + 1,
                    required_time: t,
                    required_weapon: a,
                },
            );
        }
        weapon_learings
    };

    let mut required_weapons_to_learn_ougi = HashSet::new();
    required_weapons_to_learn_ougi.insert(n);
    let mut required_weapon_id_queue = VecDeque::new();
    required_weapon_id_queue.push_back(n);

    while let Some(next_weapon_id) = required_weapon_id_queue.pop_front() {
        for required_weapon_id in weapon_learnings
            .get(&next_weapon_id)
            .unwrap()
            .required_weapon
            .iter()
        {
            if !required_weapons_to_learn_ougi.contains(required_weapon_id) {
                required_weapons_to_learn_ougi.insert(*required_weapon_id);
                required_weapon_id_queue.push_back(*required_weapon_id);
            }
        }
    }

    let ans: usize = required_weapons_to_learn_ougi
        .iter()
        .map(|weapon_id| weapon_learnings.get(weapon_id).unwrap().required_time)
        .sum();

    println!("{:#?}", ans);

    Ok(())
}

// 3
// 10 0
// 10 0

mod perm {
    pub struct PermutationIterator<T: Ord + Clone + Copy> {
        li: Vec<T>,
        idx: usize,
    }
    impl<T: Ord + Clone + Copy> PermutationIterator<T> {
        pub fn new(li: Vec<T>) -> PermutationIterator<T> {
            let idx = 0;
            PermutationIterator { li, idx }
        }
    }

    impl<T: Ord + Clone + Copy> Iterator for PermutationIterator<T> {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx == self.li.len().pow(self.li.len() as u32) {
                return None;
            }

            let res = {
                let mut res = Vec::with_capacity(self.li.len());
                let mut idx = self.idx;
                while res.len() < self.li.len() {
                    res.push(self.li[idx % self.li.len()]);
                    idx /= self.li.len();
                }
                res.reverse();
                res
            };
            self.idx += 1;
            Some(res)
        }
    }

    pub trait Permutation<T: Ord + Clone + Copy> {
        fn perutation(self) -> PermutationIterator<T>;
    }

    impl<T: Ord + Clone + Copy, I: IntoIterator<Item = T>> Permutation<T> for I {
        fn perutation(self) -> PermutationIterator<T> {
            PermutationIterator::new(self.into_iter().collect())
        }
    }
}
