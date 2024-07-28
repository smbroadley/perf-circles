#![feature(test)]
extern crate test;
use rand::Rng;
use std::vec::Vec;

#[allow(dead_code, reason = "This code is under test, only.")]
#[derive(Copy, Clone)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

#[allow(dead_code, reason = "This code is under test, only.")]
impl Circle {
    pub fn dist_sq(&self, other: &Circle) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx) + (dy * dy)
    }

    pub fn intersects_sq(&self, other: &Circle) -> bool {
        self.dist_sq(other) < (self.r + other.r) * (self.r + other.r)
    }

    pub fn dist(&self, other: &Circle) -> f32 {
        self.dist_sq(other).sqrt()
    }

    pub fn intersects(&self, other: &Circle) -> bool {
        self.dist(other) < (self.r + other.r)
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen_range(0.0..10.0),
            y: rng.gen_range(0.0..10.0),
            r: rng.gen_range(0.1..1.0),
        }
    }

    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Self { x, y, r }
    }
}

#[allow(dead_code, reason = "This code is under test, only.")]
trait AllPairs<'a, T> {
    fn all_pairs(&'a self) -> PairsIter<'a, T>;
}

impl<'a, T> AllPairs<'a, T> for Vec<T> {
    fn all_pairs(&'a self) -> PairsIter<'a, T> {
        PairsIter::new(self)
    }
}

impl<'a, T> AllPairs<'a, T> for &'a [T] {
    fn all_pairs(&'a self) -> PairsIter<'a, T> {
        PairsIter::new(self)
    }
}

struct PairsIter<'a, T> {
    x: usize,
    y: usize,
    slice: &'a [T],
}

impl<'a, T> PairsIter<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        Self { x: 0, y: 0, slice }
    }
}

impl<'a, T> Iterator for PairsIter<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        // iteration logic
        //
        self.y += 1;

        if self.y == self.slice.len() {
            self.x += 1;
            self.y = self.x + 1;
        }

        // exit logic
        //
        if self.x >= self.slice.len().saturating_sub(1) {
            return None;
        }

        // return a pair of refs into the slice
        //
        let (l, r) = self.slice.split_at(self.x + 1);
        let result = (&l[self.x], &r[self.y - self.x - 1]);

        Some(result)
    }
}

fn main() {
    println!("");
    println!("╭───────────────────────────────────────────────────────────────────────────╮");
    println!("│ This crate only contains tests/benches; run 'cargo test' or 'cargo bench' │");
    println!("╰───────────────────────────────────────────────────────────────────────────╯");
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    #[test]
    fn test_pairs_none() {
        let arr = vec![1];
        let prs = arr.all_pairs().collect::<Vec<_>>();

        assert_eq!(prs.len(), 0);
    }

    #[test]
    fn test_pairs_one() {
        let arr = vec![1, 2];
        let prs = arr.all_pairs().collect::<Vec<_>>();

        assert_eq!(prs, [(&1, &2)]);
    }

    #[test]
    fn test_pairs() {
        let arr = vec![1, 2, 3, 4];
        let prs = arr.all_pairs().collect::<Vec<_>>();

        assert_eq!(
            prs,
            [(&1, &2), (&1, &3), (&1, &4), (&2, &3), (&2, &4), (&3, &4)]
        );
    }

    static CIRCLES: LazyLock<Vec<Circle>> = LazyLock::new(|| {
        const COUNT: usize = 1000;
        let mut c: Vec<Circle> = Vec::new();
        c.resize_with(COUNT, || Circle::random());

        c
    });

    fn get_circles() -> &'static Vec<Circle> {
        &CIRCLES
    }

    #[test]
    fn test_intersections_agree() {
        for (c1, c2) in get_circles().all_pairs() {
            let i1 = c1.intersects(&c2);
            let i2 = c1.intersects_sq(&c2);

            assert_eq!(i1, i2);
        }
    }

    #[bench]
    fn bench_intersect(b: &mut test::Bencher) {
        b.iter(|| {
            let circles = get_circles();

            let c = circles.len() * (circles.len() - 1) / 2;
            let mut i = 0;

            for (c1, c2) in circles.all_pairs() {
                if c1.intersects(&c2) {
                    i += 1;
                }
            }

            println!(
                "intersections: {} / {} ({}%)",
                i,
                c,
                (100.0 * i as f32 / c as f32)
            );
        });
    }

    #[bench]
    fn bench_intersect_sq(b: &mut test::Bencher) {
        b.iter(|| {
            let circles = get_circles();

            let c = circles.len() * (circles.len() - 1) / 2;
            let mut i = 0;

            for (c1, c2) in circles.all_pairs() {
                if c1.intersects_sq(&c2) {
                    i += 1;
                }
            }

            println!(
                "intersections: {} / {} ({}%)",
                i,
                c,
                (100.0 * i as f32 / c as f32)
            );
        });
    }
}
