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
