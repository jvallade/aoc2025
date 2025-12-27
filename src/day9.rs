use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, u64};
use nom::multi::separated_list1;

#[derive(Debug, Eq, PartialOrd, Ord, Hash, Clone, Default)]
struct Tile {
    x: u64,
    y: u64,
    x_compressed: u64,
    y_compressed: u64,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile [{} - {}]", self.x, self.y)
    }
}

impl Tile {
    fn new(x: u64, y: u64) -> Self {
        Tile {
            x,
            y,
            ..Default::default()
        }
    }
    fn new_compressed(
        x: u64,
        y: u64,
        x_map_inv: &HashMap<u64, u64>,
        y_map_inv: &HashMap<u64, u64>,
    ) -> Self {
        Tile {
            x: x_map_inv[&x],
            y: y_map_inv[&y],
            x_compressed: x,
            y_compressed: y,
        }
    }

    fn apply_compression(&mut self, x_map: &HashMap<u64, u64>, y_map: &HashMap<u64, u64>) {
        self.x_compressed = x_map[&self.x];
        self.y_compressed = y_map[&self.y];
    }

    fn area_size(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    fn is_on_polygon_edge(&self, polygon: &[Tile]) -> bool {
        let n = polygon.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let xi = polygon[i].x_compressed;
            let yi = polygon[i].y_compressed;
            let xj = polygon[j].x_compressed;
            let yj = polygon[j].y_compressed;

            if (xi == xj
                && xi == self.x_compressed
                && self.y_compressed >= yi.min(yj)
                && self.y_compressed <= yi.max(yj))
                || (yi == yj
                    && yi == self.y_compressed
                    && self.x_compressed >= xi.min(xj)
                    && self.x_compressed <= xi.max(xj))
            {
                return true;
            }
        }
        false
    }

    fn is_in_polygon(&self, polygon: &[Tile]) -> Result<bool, EdgeAligned> {
        let mut inside = false;
        let n = polygon.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let xi = polygon[i].x_compressed;
            let yi = polygon[i].y_compressed;
            let xj = polygon[j].x_compressed;
            let yj = polygon[j].y_compressed;

            if (xi == xj
                && xi == self.x_compressed
                && self.y_compressed >= yi.min(yj)
                && self.y_compressed <= yi.max(yj))
                || (yi == yj
                    && yi == self.y_compressed
                    && self.x_compressed >= xi.min(xj)
                    && self.x_compressed <= xi.max(xj))
            {
                return Ok(true);
            }

            if xi == xj
                && self.y_compressed >= yi.min(yj)
                && self.y_compressed <= yi.max(yj)
                && self.x_compressed < xi
            {
                inside = !inside;
            } else if yi == yj && self.y_compressed == yi && self.x_compressed < xi.min(xj) {
                println!("WARNING : This is the edge case where we don't know");
                return Err(EdgeAligned {});
            }
        }
        Ok(inside)
    }

    fn get_edges(&self, other: &Self) -> Vec<Tile> {
        let mut res = Vec::new();
        for i in self.x.min(other.x)..self.x.max(other.x) {
            res.push(Self::new(i, self.y));
            res.push(Self::new(i, other.y));
        }
        for i in self.y.min(other.y)..self.y.max(other.y) {
            res.push(Self::new(self.x, i));
            res.push(Self::new(other.x, i));
        }
        res
    }

    fn get_compressed_edges(
        &self,
        other: &Self,
        x_map_inv: &HashMap<u64, u64>,
        y_map_inv: &HashMap<u64, u64>,
    ) -> Vec<Tile> {
        let mut res = Vec::new();
        for i in
            self.x_compressed.min(other.x_compressed)..=self.x_compressed.max(other.x_compressed)
        {
            res.push(Self::new_compressed(
                i,
                self.y_compressed,
                x_map_inv,
                y_map_inv,
            ));
            res.push(Self::new_compressed(
                i,
                other.y_compressed,
                x_map_inv,
                y_map_inv,
            ));
        }
        for i in
            self.y_compressed.min(other.y_compressed)..=self.y_compressed.max(other.y_compressed)
        {
            res.push(Self::new_compressed(
                self.x_compressed,
                i,
                x_map_inv,
                y_map_inv,
            ));
            res.push(Self::new_compressed(
                other.x_compressed,
                i,
                x_map_inv,
                y_map_inv,
            ));
        }
        res
    }
}

#[derive(Debug)]
struct EdgeAligned {}

impl Display for EdgeAligned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point is aligne with an edge")
    }
}

impl std::error::Error for EdgeAligned {}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rectangle {
    tiles: (Tile, Tile),
    area: u64,
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area.partial_cmp(&other.area)
    }
}

impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

fn tile(input: &str) -> IResult<&str, Tile> {
    let (remainder, coords) = separated_list1(char(','), u64).parse(input)?;
    Ok((
        remainder,
        Tile::new(
            *coords.get(0).expect("X not found"),
            *coords.get(1).expect("Y not found"),
        ),
    ))
}

fn tiles(input: &str) -> IResult<&str, Vec<Tile>> {
    separated_list1(char('\n'), tile).parse(input)
}

fn get_compression(
    tiles: &[Tile],
) -> (
    HashMap<u64, u64>,
    HashMap<u64, u64>,
    HashMap<u64, u64>,
    HashMap<u64, u64>,
) {
    let mut x_values = HashSet::new();
    let mut y_values = HashSet::new();
    tiles.iter().for_each(|t| {
        x_values.insert(t.x);
        y_values.insert(t.y);
    });

    let mut x_values = Vec::from_iter(x_values);
    let mut y_values = Vec::from_iter(y_values);
    x_values.sort();
    y_values.sort();

    // compression maps
    let mut x_map = HashMap::new();
    let mut y_map = HashMap::new();

    // decompression maps
    let mut x_map_inv = HashMap::new();
    let mut y_map_inv = HashMap::new();

    x_values.iter().enumerate().for_each(|(i, x)| {
        x_map.insert(*x, i as u64);
        x_map_inv.insert(i as u64, *x);
    });
    y_values.iter().enumerate().for_each(|(i, y)| {
        y_map.insert(*y, i as u64);
        y_map_inv.insert(i as u64, *y);
    });

    return (x_map, y_map, x_map_inv, y_map_inv);
}

fn draw(red_tiles: &[Tile]) {
    let max_x = red_tiles.iter().map(|t| t.x).max().unwrap();
    let max_y = red_tiles.iter().map(|t| t.y).max().unwrap();
    let min_x = red_tiles.iter().map(|t| t.x).min().unwrap();
    let min_y = red_tiles.iter().map(|t| t.y).min().unwrap();

    for j in min_y..=max_y {
        for i in min_x..=max_x {
            let t = Tile {
                x: i,
                y: j,
                ..Default::default()
            };
            if red_tiles.contains(&t) {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn draw_compressed(
    red_tiles: &[Tile],
    edges: Option<&[Tile]>,
    x_map_inv: &HashMap<u64, u64>,
    y_map_inv: &HashMap<u64, u64>,
) {
    let max_x = red_tiles.iter().map(|t| t.x_compressed).max().unwrap();
    let max_y = red_tiles.iter().map(|t| t.y_compressed).max().unwrap();
    let min_x = red_tiles.iter().map(|t| t.x_compressed).min().unwrap();
    let min_y = red_tiles.iter().map(|t| t.y_compressed).min().unwrap();

    let edges: Vec<Tile> = match edges {
        Some(e) => e
            .iter()
            .map(|ee| Tile {
                x: x_map_inv[&ee.x_compressed],
                y: y_map_inv[&ee.y_compressed],
                x_compressed: ee.x_compressed,
                y_compressed: ee.y_compressed,
            })
            .collect(),
        None => Vec::new(),
    };

    println!();
    for j in min_y..=max_y {
        for i in min_x..=max_x {
            let t = Tile {
                x: x_map_inv[&i],
                y: y_map_inv[&j],
                x_compressed: i,
                y_compressed: j,
            };
            if edges.contains(&t) && red_tiles.contains(&t) {
                print!("X");
            } else if edges.contains(&t) {
                print!("O");
            } else if red_tiles.contains(&t) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn draw_compressed_area(
    red_tiles: &[Tile],
    area: &[Tile],
    x_map_inv: &HashMap<u64, u64>,
    y_map_inv: &HashMap<u64, u64>,
) {
    let max_x = red_tiles.iter().map(|t| t.x_compressed).max().unwrap();
    let max_y = red_tiles.iter().map(|t| t.y_compressed).max().unwrap();
    let min_x = red_tiles.iter().map(|t| t.x_compressed).min().unwrap();
    let min_y = red_tiles.iter().map(|t| t.y_compressed).min().unwrap();

    println!();
    for j in min_y..=max_y {
        for i in min_x..=max_x {
            let t = Tile {
                x: x_map_inv[&i],
                y: y_map_inv[&j],
                x_compressed: i,
                y_compressed: j,
            };
            if area.contains(&t) && red_tiles.contains(&t) {
                print!("X");
            } else if area.contains(&t) {
                print!("O");
            } else if red_tiles.contains(&t) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

// Depth First Seek implementation
fn get_tiles_in_area(
    tiles: &[Tile],
    x_map_inv: &HashMap<u64, u64>,
    y_map_inv: &HashMap<u64, u64>,
) -> HashSet<Tile> {
    // find a point inside the polygon that is not on an edge
    // we do not consider points that are aligned with an horizontal edge
    let mut start = None;
    for i in 0..tiles.len() {
        let ne = Tile::new_compressed(
            tiles[i].x_compressed.saturating_add(1),
            tiles[i].y_compressed.saturating_sub(1),
            x_map_inv,
            y_map_inv,
        );
        let se = Tile::new_compressed(
            tiles[i].x_compressed.saturating_add(1),
            tiles[i].y_compressed.saturating_add(1),
            x_map_inv,
            y_map_inv,
        );
        let nw = Tile::new_compressed(
            tiles[i].x_compressed.saturating_sub(1),
            tiles[i].y_compressed.saturating_sub(1),
            x_map_inv,
            y_map_inv,
        );
        let sw = Tile::new_compressed(
            tiles[i].x_compressed.saturating_sub(1),
            tiles[i].y_compressed.saturating_add(1),
            x_map_inv,
            y_map_inv,
        );
        start = if ne.is_in_polygon(tiles).unwrap_or(false) && !ne.is_on_polygon_edge(tiles) {
            Some(ne)
        } else if se.is_in_polygon(tiles).unwrap_or(false) && !se.is_on_polygon_edge(tiles) {
            Some(se)
        } else if nw.is_in_polygon(tiles).unwrap_or(false) && !nw.is_on_polygon_edge(tiles) {
            Some(nw)
        } else if sw.is_in_polygon(tiles).unwrap_or(false) && !sw.is_on_polygon_edge(tiles) {
            Some(sw)
        } else {
            continue;
        };

        if let Some(_) = &start {
            break;
        }
    }
    let start = start.expect("Did not find a starting point");

    // And now we visit all the tiles and check if we reached an edge
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    while let Some(node) = stack.pop() {
        if visited.insert(node.clone()) {
            if !node.is_on_polygon_edge(tiles) {
                stack.push(Tile::new_compressed(
                    node.x_compressed.saturating_sub(1),
                    node.y_compressed,
                    x_map_inv,
                    y_map_inv,
                ));
                stack.push(Tile::new_compressed(
                    node.x_compressed.saturating_add(1),
                    node.y_compressed,
                    x_map_inv,
                    y_map_inv,
                ));
                stack.push(Tile::new_compressed(
                    node.x_compressed,
                    node.y_compressed.saturating_add(1),
                    x_map_inv,
                    y_map_inv,
                ));
                stack.push(Tile::new_compressed(
                    node.x_compressed,
                    node.y_compressed.saturating_sub(1),
                    x_map_inv,
                    y_map_inv,
                ));
                stack.push(Tile::new_compressed(
                    node.x_compressed.saturating_sub(1),
                    node.y_compressed.saturating_sub(1),
                    x_map_inv,
                    y_map_inv,
                ));
                stack.push(Tile::new_compressed(
                    node.x_compressed.saturating_add(1),
                    node.y_compressed.saturating_sub(1),
                    x_map_inv,
                    y_map_inv,
                ));
                stack.push(Tile::new_compressed(
                    node.x_compressed.saturating_sub(1),
                    node.y_compressed.saturating_add(1),
                    x_map_inv,
                    y_map_inv,
                ));
                stack.push(Tile::new_compressed(
                    node.x_compressed.saturating_add(1),
                    node.y_compressed.saturating_add(1),
                    x_map_inv,
                    y_map_inv,
                ));
            }
        }
    }
    visited
}

fn part1(tiles: &[Tile]) -> u64 {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    tiles.iter().for_each(|t1| {
        tiles.iter().for_each(|t2| {
            if t1 != t2 && seen.insert((t1, t2)) {
                heap.push(Rectangle {
                    tiles: ((*t1).clone(), (*t2).clone()),
                    area: t1.area_size(t2),
                });
                seen.insert((t1, t2));
                seen.insert((t2, t1));
            }
        });
    });

    heap.pop().unwrap().area
}

fn part2(tiles: &mut Vec<Tile>) -> u64 {
    // Compute compression maps
    let (x_map, y_map, x_map_inv, y_map_inv) = get_compression(&tiles);

    // Compress red tiles
    tiles
        .iter_mut()
        .for_each(|t| t.apply_compression(&x_map, &y_map));

    // Get the list of tiles in the area
    let area = get_tiles_in_area(tiles, &x_map_inv, &y_map_inv);

    // Compute the area of all the rectangles and sort them in order
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    let mut i = 0;
    tiles.iter().for_each(|t1| {
        tiles.iter().for_each(|t2| {
            println!("progress {} / {}", i, tiles.len().pow(2));
            if t1 != t2 && seen.insert((t1, t2)) {
                let edges = t1.get_compressed_edges(t2, &x_map_inv, &y_map_inv);
                if edges.iter().all(|t| area.contains(t)) {
                    heap.push(Rectangle {
                        tiles: ((*t1).clone(), (*t2).clone()),
                        area: t1.area_size(t2),
                    });
                }
                seen.insert((t1, t2));
                seen.insert((t2, t1));
            }
            i += 1;
        });
    });

    let rectangle = heap.pop().unwrap();
    rectangle.area
}

pub fn run(input: &str) {
    let (_, mut tiles) = tiles(input).expect("Could not parse input problems");
    println!("=> part1 : {}", part1(&tiles));
    println!("=> part2 : {}", part2(&mut tiles));
}

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::day9::{
        Tile, draw_compressed, draw_compressed_area, get_compression, get_tiles_in_area,
    };

    use super::{part1, part2, tiles};

    #[test]
    fn example_data() {
        let data = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let (_, mut tiles) = tiles(data).expect("Could not parse the input data");
        assert_eq!(part1(&tiles), 50);
        assert_eq!(part2(&mut tiles), 24);
    }

    #[test]
    fn debug() {
        let input = fs::read_to_string(format!("./resources/input9"))
            .expect("Could not load the input file");
        let (_, mut tiles) = tiles(&input).expect("Could not parse the input data");

        let (x_map, y_map, x_map_inv, y_map_inv) = get_compression(&tiles);

        // Compress red tiles
        tiles
            .iter_mut()
            .for_each(|t| t.apply_compression(&x_map, &y_map));
        let mut t1 = Tile {
            x: 11937,
            y: 77901,
            ..Default::default()
        };
        let mut t2 = Tile {
            x: 11204,
            y: 77297,
            ..Default::default()
        };

        t1.apply_compression(&x_map, &y_map);
        t2.apply_compression(&x_map, &y_map);

        let edges = t1.get_compressed_edges(&t2, &x_map_inv, &y_map_inv);
        draw_compressed(&tiles, Some(&edges), &x_map_inv, &y_map_inv);

        for t in edges {
            println!("{:?} => {}", t, t.is_in_polygon(&tiles).unwrap());
            assert!(t.is_in_polygon(&tiles).unwrap());
        }
    }

    #[test]
    fn fill_area() {
        let input = fs::read_to_string(format!("./resources/input9"))
            .expect("Could not load the input file");
        let (_, mut tiles) = tiles(&input).expect("Could not parse the input data");

        let (x_map, y_map, x_map_inv, y_map_inv) = get_compression(&tiles);
        tiles
            .iter_mut()
            .for_each(|t| t.apply_compression(&x_map, &y_map));

        let area = get_tiles_in_area(&tiles, &x_map_inv, &y_map_inv);

        draw_compressed_area(
            &tiles,
            &area.into_iter().collect::<Vec<_>>(),
            &x_map_inv,
            &y_map_inv,
        );
    }
}
