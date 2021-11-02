use std::cmp;

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
    let n: usize = cin().parse()?;

    struct Fuse {
        // [cm]
        length: f64,
        // [cm/s]
        burning_speed: f64,
    }

    impl Fuse {
        fn secs_to_burn_out(&self) -> f64 {
            self.length / self.burning_speed
        }
    }

    let fuses = (0..n)
        .map(|_| {
            let ab = cin_vec().to_2();
            Fuse {
                length: ab.0,
                burning_speed: ab.1,
            }
        })
        .collect::<Vec<Fuse>>();

    let total_secs_to_burn_out = fuses
        .iter()
        .fold(0.0, |acc, fuse| acc + fuse.secs_to_burn_out());

    let mut ans = 0.0;
    let mut i = 0;
    let mut acc_secs_to_burn = 0.0;

    loop {
        if acc_secs_to_burn + fuses[i].secs_to_burn_out() > total_secs_to_burn_out / 2.0 {
            break;
        }
        acc_secs_to_burn += fuses[i].secs_to_burn_out();
        ans += fuses[i].length;
        i += 1;
    }
    ans += (total_secs_to_burn_out / 2.0 - acc_secs_to_burn) * fuses[i].burning_speed;
    println!("{}", ans);
    Ok(())
}

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
