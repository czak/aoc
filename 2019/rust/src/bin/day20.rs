use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashMap;

type Pos = (usize, usize);

const EX1: &str = include_str!("../../../in/day20.ex1");
const EX2: &str = include_str!("../../../in/day20.ex2");

fn main() {
    let (start, end, graph) = parse(EX1);
    assert_eq!(23, find_path(start, end, &graph));

    let (start, end, graph) = parse(EX2);
    assert_eq!(58, find_path(start, end, &graph));
}

fn find_path(start: Pos, end: Pos, graph: &HashMap<Pos, Vec<Pos>>) -> usize {
    dijkstra(
        &start,
        |pos| {
            graph
                .get(pos)
                .map_or(vec![], |v| v.iter().copied().map(|p| (p, 1)).collect())
        },
        |pos| *pos == end,
    )
    .unwrap()
    .1
}

#[derive(Debug)]
enum Edge {
    OuterNorth,
    OuterEast,
    OuterSouth,
    OuterWest,
    InnerNorth,
    InnerEast,
    InnerSouth,
    InnerWest,
}

fn parse(s: &str) -> (Pos, Pos, HashMap<Pos, Vec<Pos>>) {
    let map: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

    let width = map[0].len() - 4;
    let height = map.len() - 4;
    let thickness = (2..).take_while(|&i| map[i][i] != ' ').last().unwrap() - 1;

    let edge = |x: usize, y: usize| {
        use Edge::*;
        match (x, y) {
            // outer edge
            (_, 2) => Some(OuterNorth),
            (_, y) if y == 2 + height - 1 => Some(OuterSouth),
            (2, _) => Some(OuterWest),
            (x, _) if x == 2 + width - 1 => Some(OuterEast),
            // inner edge
            (x, y) if y >= 2 + thickness && y < 2 + height - thickness => {
                if x == 2 + thickness - 1 {
                    Some(InnerWest)
                } else if x == 2 + width - thickness {
                    Some(InnerEast)
                } else {
                    None
                }
            }
            (x, y) if x >= 2 + thickness && x < 2 + width - thickness => {
                if y == 2 + thickness - 1 {
                    Some(InnerNorth)
                } else if y == 2 + height - thickness {
                    Some(InnerSouth)
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    let mut graph: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let mut portals: HashMap<String, Vec<Pos>> = HashMap::new();

    for y in 2..height + 2 {
        for x in 2..width + 2 {
            if map[y][x] == '.' {
                let mut v = vec![];
                // connect to open ones around me
                if map[y - 1][x] == '.' {
                    v.push((x, y - 1));
                }
                if map[y + 1][x] == '.' {
                    v.push((x, y + 1));
                }
                if map[y][x - 1] == '.' {
                    v.push((x - 1, y));
                }
                if map[y][x + 1] == '.' {
                    v.push((x + 1, y));
                }
                graph.insert((x, y), v);

                // is it a portal?
                if let Some(edge) = edge(x, y) {
                    // read label
                    let ((ax, ay), (bx, by)) = label_for(edge, x, y);
                    let label: String = [map[ay][ax], map[by][bx]].iter().collect();
                    portals.entry(label).or_insert(vec![]).push((x, y));
                }
            }
        }
    }

    // add portals to graph
    for (label, positions) in &portals {
        if label == "AA" || label == "ZZ" {
            continue;
        }

        // there are 2 elements, connect both ways
        let a = positions[0];
        let b = positions[1];
        graph.entry(a).or_insert(vec![]).push(b);
        graph.entry(b).or_insert(vec![]).push(a);
    }

    (
        portals.get("AA").unwrap()[0],
        portals.get("ZZ").unwrap()[0],
        graph,
    )
}

fn label_for(edge: Edge, x: usize, y: usize) -> (Pos, Pos) {
    use Edge::*;

    match edge {
        OuterNorth | InnerSouth => ((x, y - 2), (x, y - 1)),
        OuterEast | InnerWest => ((x + 1, y), (x + 2, y)),
        OuterSouth | InnerNorth => ((x, y + 1), (x, y + 2)),
        OuterWest | InnerEast => ((x - 2, y), (x - 1, y)),
    }
}
