use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

#[allow(dead_code)]
const EX1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    fn energy(&self) -> i32 {
        (self.x.abs() + self.y.abs() + self.z.abs())
            * (self.vx.abs() + self.vy.abs() + self.vz.abs())
    }
}

impl FromStr for Moon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // <x=7, y=10, z=17>
        let re = Regex::new(r"x=(-?\d+), y=(-?\d+), z=(-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Moon {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            z: caps[3].parse().unwrap(),
            ..Default::default()
        })
    }
}

type System1D = (i32, i32, i32, i32, i32, i32, i32, i32);

fn components(moons: &[Moon]) -> (System1D, System1D, System1D) {
    (
        (
            moons[0].x,
            moons[0].vx,
            moons[1].x,
            moons[1].vx,
            moons[2].x,
            moons[2].vx,
            moons[3].x,
            moons[3].vx,
        ),
        (
            moons[0].y,
            moons[0].vy,
            moons[1].y,
            moons[1].vy,
            moons[2].y,
            moons[2].vy,
            moons[3].y,
            moons[3].vy,
        ),
        (
            moons[0].z,
            moons[0].vz,
            moons[1].z,
            moons[1].vz,
            moons[2].z,
            moons[2].vz,
            moons[3].z,
            moons[3].vz,
        ),
    )
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    while n != 0 {
        if n < m {
            let t = n;
            n = m;
            m = t;
        }
        n = n % m;
    }
    m
}

fn main() {
    assert_eq!(
        Ok(Moon {
            x: 7,
            y: 10,
            z: 17,
            vx: 0,
            vy: 0,
            vz: 0,
        }),
        "<x=7, y=10, z=17>".parse()
    );

    // Part 1

    let mut moons: Vec<Moon> = include_str!("../../../in/day12.in")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    for _ in 1..=1000 {
        simulate(&mut moons);
    }

    println!(
        "Part 1: {}",
        moons.iter().fold(0, |acc, m| acc + m.energy())
    );

    // Part 2

    let mut moons: Vec<Moon> = include_str!("../../../in/day12.in")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let (initx, inity, initz) = components(&moons);
    let (mut foundx, mut foundy, mut foundz) = (0, 0, 0);

    for i in 1..=250000 {
        simulate(&mut moons);

        let (x, y, z) = components(&moons);

        if foundx == 0 && x == initx {
            foundx = i;
        }
        if foundy == 0 && y == inity {
            foundy = i;
        }
        if foundz == 0 && z == initz {
            foundz = i;
        }
    }

    let lcmxy = foundx * foundy / gcd(foundx, foundy);
    let lcmxyz = lcmxy * foundz / gcd(lcmxy, foundz);

    println!("Part 2: {}", lcmxyz);
}

fn simulate(moons: &mut [Moon]) {
    // apply gravity
    for (a, b) in (0..moons.len()).tuple_combinations() {
        if moons[a].x < moons[b].x {
            moons[a].vx += 1;
            moons[b].vx -= 1;
        } else if moons[a].x > moons[b].x {
            moons[a].vx -= 1;
            moons[b].vx += 1;
        }

        if moons[a].y < moons[b].y {
            moons[a].vy += 1;
            moons[b].vy -= 1;
        } else if moons[a].y > moons[b].y {
            moons[a].vy -= 1;
            moons[b].vy += 1;
        }

        if moons[a].z < moons[b].z {
            moons[a].vz += 1;
            moons[b].vz -= 1;
        } else if moons[a].z > moons[b].z {
            moons[a].vz -= 1;
            moons[b].vz += 1;
        }
    }

    // apply velocity
    for moon in moons {
        moon.x += moon.vx;
        moon.y += moon.vy;
        moon.z += moon.vz;
    }
}
