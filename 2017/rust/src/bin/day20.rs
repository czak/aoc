use std::collections::{HashSet, HashMap};

#[derive(Default, Debug, PartialEq, Eq, Hash)]
struct Particle {
    n: usize,
    p: [i64; 3],
    v: [i64; 3],
    a: [i64; 3],
    alive: bool,
}

impl Particle {
    fn tick(&mut self) {
        self.v[0] += self.a[0];
        self.v[1] += self.a[1];
        self.v[2] += self.a[2];
        self.p[0] += self.v[0];
        self.p[1] += self.v[1];
        self.p[2] += self.v[2];
    }

    fn distance(&self) -> i64 {
        self.p[0].abs() + self.p[1].abs() + self.p[2].abs()
    }

    fn acceleration(&self) -> i64 {
        self.a[0].abs() + self.a[1].abs() + self.a[2].abs()
    }
}

fn parse_vec(s: &str) -> [i64; 3] {
    let mut res = [0; 3];
    for (n, s) in res.iter_mut().zip(s.split(',')) {
        *n = s.parse().unwrap();
    }
    res
}

fn main() {
    let s = include_str!("../../../in/day20.in");

    let mut particles = HashMap::new();

    for (n, line) in s.lines().enumerate() {
        let mut it = line.split(", ").map(|s| &s[3..s.len()-1]);
        particles.insert(n, Particle {
            n: n,
            p: parse_vec(it.next().unwrap()),
            v: parse_vec(it.next().unwrap()),
            a: parse_vec(it.next().unwrap()),
            alive: true,
        });
    }

    let min = particles.values().min_by(|p1, p2| p1.acceleration().cmp(&p2.acceleration())).unwrap();
    println!("{:?}, {}", min, min.distance());

    for _ in 0..100 {
        particles.values_mut().for_each(Particle::tick);

        // count clusters
        let mut clusters: HashMap<[i64; 3], Vec<usize>> = HashMap::new();
        particles.iter().for_each(|(n, p)| {
            if p.alive {
                clusters.entry(p.p)
                    .and_modify(|v| v.push(*n))
                    .or_insert(vec![*n]);
            }
        });

        for c in clusters.values() {
            if c.len() > 1 {
                for n in c {
                    particles.remove(n);
                }
            }
        }

        println!("{}", particles.len());
    }
}
