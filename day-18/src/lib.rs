use std::collections::HashSet;

use glam::i32::IVec2;
use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    let edges = parse(input);
    let start = IVec2 { x: 0, y: 0 };

    let mut points = vec![start];
    let mut n_points = points.len();

    edges.iter().for_each(|edge| {
        let position = points[n_points - 1];
        n_points += edge.length as usize;

        match edge.direction {
            Direction::Up => (0..edge.length).for_each(|i| {
                points.push(IVec2 {
                    x: position.x,
                    y: position.y - (i + 1),
                })
            }),
            Direction::Down => (0..edge.length).for_each(|i| {
                points.push(IVec2 {
                    x: position.x,
                    y: position.y + i + 1,
                })
            }),
            Direction::Left => (0..edge.length).for_each(|i| {
                points.push(IVec2 {
                    x: position.x - (i + 1),
                    y: position.y,
                })
            }),
            Direction::Right => (0..edge.length).for_each(|i| {
                points.push(IVec2 {
                    x: position.x + i + 1,
                    y: position.y,
                })
            }),
        };
    });

    points.sort_unstable_by_key(|edge| (edge.y, edge.x));

    // let points: HashSet<IVec2> = HashSet::from_iter(points);

    points
        .iter()
        .group_by(|x| x.y)
        .into_iter()
        .map(|(_x, group)| {
            dbg!(_x);
            count_line(group.sorted_by_key(|x| x.x).map(|v| *v).collect())
        })
        .sum::<usize>()
        - 1
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Edge<'a> {
    direction: Direction,
    length: i32,
    _color: &'a str,
}

fn parse(input: &str) -> Vec<Edge> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            Edge {
                direction: match iter.next().unwrap() {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    _ => panic!("Invalid direction"),
                },
                length: iter.next().unwrap().parse::<i32>().unwrap(),
                _color: iter.next().unwrap(),
            }
        })
        .collect()
}

fn count_line(line: Vec<IVec2>) -> usize {
    let mut total = line.len();

    dbg!(&line);

    let mut inside = true;
    let mut v = line[0].x;

    line[1..].iter().for_each(|x| {
        if x.x - 1 > v {
            if inside {
                total += (x.x - v - 1) as usize;
            } else {
                inside = !inside;
            }
        }
        v = x.x;
    });

    dbg!(total);

    total
}

pub fn part_two(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part_one(&input), 62)
    }

    // #[test]
    // fn test_part_two() {
    //     let input = "";
    //     assert_eq!(part_two(&input), 0)
    // }
}