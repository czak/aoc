use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashMap;

type Pos = (usize, usize);
type Pos3 = (usize, usize, usize);

const EX3: &str = include_str!("../../../in/day20.ex3");

fn main() {
    let ((ax, ay), (bx, by), graph, portals) = parse(EX3);
    // same level
    assert_eq!(10, find_path((13, 8, 0), (6, 5, 0), &graph, &portals));
    // 1 level down
    assert_eq!(8, find_path((13, 8, 0), (21, 29, 1), &graph, &portals));
    // 2 levels down
    assert_eq!(12, find_path((13, 8, 0), (18, 3, 2), &graph, &portals));
    // 2 levels down, 1 level up
    assert_eq!(16, find_path((13, 8, 0), (37, 13, 1), &graph, &portals));

    // // example
    // assert_eq!(396, find_path((ax, ay, 0), (bx, by, 0), &graph, &portals));

    // let ((ax, ay), (bx, by), graph, portals) = parse(include_str!("../../../in/day20.in"));
    // println!(
    //     "Part 2: {}",
    //     find_path((ax, ay, 0), (bx, by, 0), &graph, &portals)
    // );
}

fn find_path(
    (ax, ay, alevel): Pos3,
    (bx, by, blevel): Pos3,
    graph: &HashMap<Pos, Vec<Pos>>,
    portals: &HashMap<Pos, (Pos, Edge)>,
) -> usize {
    let successors = |&(x, y, level): &Pos3| {
        // internal neighbors
        let mut v: Vec<(Pos3, usize)> = graph.get(&(x, y)).map_or(vec![], |v| {
            v.iter().copied().map(|(x, y)| ((x, y, level), 1)).collect()
        });

        // is there a portal at this position
        // - inner => to level + 1
        // - outer => to level - 1
        if let Some(&((dx, dy), edge)) = portals.get(&(x, y)) {
            use Edge::*;
            match edge {
                InnerNorth | InnerEast | InnerSouth | InnerWest => {
                    // println!("p from {:?} to {:?}", (x, y, level), (dx, dy, level + 1));
                    // limit depth
                    if level < 20 {
                        v.push(((dx, dy, level + 1), 1));
                    }
                }
                OuterNorth | OuterEast | OuterSouth | OuterWest => {
                    // println!("p from {:?} to {:?}", (x, y, level), (dx, dy, level + 1));
                    // can't move higher than level 0
                    if level > 1 {
                        v.push(((dx, dy, level - 1), 1));
                    }
                }
            }
        }

        v
    };

    dijkstra(
        &(ax, ay, alevel),
        |pos| successors(pos),
        |&(x, y, level)| (x, y, level) == (bx, by, blevel),
    )
    .unwrap()
    .1
}

#[derive(Debug, Clone, Copy)]
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

fn parse(s: &str) -> (Pos, Pos, HashMap<Pos, Vec<Pos>>, HashMap<Pos, (Pos, Edge)>) {
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
    let mut portal_graph: HashMap<Pos, (Pos, Edge)> = HashMap::new();

    for (label, positions) in &portals {
        if label == "AA" || label == "ZZ" {
            continue;
        }

        // there are 2 elements, connect both ways
        let a = positions[0];
        let b = positions[1];
        portal_graph.insert(a, (b, edge(a.0, a.1).unwrap()));
        portal_graph.insert(b, (a, edge(b.0, b.1).unwrap()));
    }

    (
        portals.get("AA").unwrap()[0],
        portals.get("ZZ").unwrap()[0],
        graph,
        portal_graph,
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
