use rand::prelude::*;
use std::collections::HashMap;

struct MajorityChecker {
    pos: HashMap<i32, Vec<usize>>,
    a: Vec<i32>,
    try_bound: usize,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut pos = HashMap::<i32, Vec<_>>::new();
        for (i, &num) in arr.iter().enumerate() {
            pos.entry(num).or_default().push(i);
        }
        Self {
            pos,
            a: arr,
            try_bound: 20,
        }
    }

    fn get_occurrence(&self, num: i32, l: usize, r: usize) -> Option<usize> {
        let vec = self.pos.get(&num)?;
        let iter_l = vec.iter().position(|&x| x >= l)?;
        let iter_r = vec.iter().position(|&x| x > r).unwrap_or(vec.len());
        Some(iter_r - iter_l)
    }

    fn get_random(&self, l: usize, r: usize) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(l, (r + 1))
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let threshold = threshold as usize;
        for _ in 0..self.try_bound {
            let index = self.get_random(left, right);
            let elem = self.a[index];
            if self.get_occurrence(elem, left, right).unwrap_or(0) >= threshold {
                return elem;
            }
        }
        -1
    }
}