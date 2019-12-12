use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

const EX1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

#[derive(Debug, Default, PartialEq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    fn simulate(&mut self, other: &mut Moon) {
        self.x += 1;
        other.x -= 1;
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

fn main() {
    assert_eq!(
        Ok(Moon {
            x: 7,
            y: 10,
            z: 17,
            vx: 0,
            vy: 0,
            vz: 0
        }),
        "<x=7, y=10, z=17>".parse()
    );

    let mut moons: Vec<Moon> = EX1.lines().map(|l| l.parse().unwrap()).collect();

    simulate(&mut moons);
    simulate(&mut moons);

    // dbg!(moons);
}

fn simulate(moons: &mut [Moon]) {
    for (a, b) in (0..moons.len()).tuple_combinations() {
        let Moon {
            x: x1,
            y: y1,
            z: z1,
            vx: mut vx1,
            vy: mut vy1,
            vz: mut vz1,
        } = moons[a];
        let Moon {
            x: x2,
            y: y2,
            z: z2,
            vx: mut vx2,
            vy: mut vy2,
            vz: mut vz2,
        } = moons[b];

        // apply gravity
        if x1 < x2 {
            moons[a].vx += 1;
            vx2 -= 1;
        } else if x1 > x2 {
            vx1 -= 1;
            vx2 += 1;
        }

        if y1 < y2 {
            vy1 += 1;
            vy2 -= 1;
        } else if y1 > y2 {
            vy1 -= 1;
            vy2 += 1;
        }

        if z1 < z2 {
            vz1 += 1;
            vz2 -= 1;
        } else if z1 > z2 {
            vz1 -= 1;
            vz2 += 1;
        }

        // apply velocity
        moons[a].x += moons[a].vx;
        moons[a].y += moons[a].vy;
        moons[a].z += moons[a].vz;
        moons[b].x += moons[a].vx;
        moons[b].y += moons[a].vy;
        moons[b].z += moons[a].vz;
    }
}
