use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::aoc;
use itertools::Itertools;
use ndarray::Array2;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map_res,
    error::VerboseError,
    sequence::tuple,
};

#[derive(Debug, PartialEq, Eq)]
struct ParserDistanceError;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Distance {
    from: String,
    to: String,
    distance: u32,
}

impl FromStr for Distance {
    type Err = ParserDistanceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (from, _, to, _, distance)) = tuple::<_, _, VerboseError<&str>, _>((
            alpha1,
            tag(" to "),
            alpha1,
            tag(" = "),
            map_res(digit1, |s: &str| s.parse::<u32>()),
        ))(s)
        .map_err(|_| ParserDistanceError)?;

        Ok(Distance {
            from: from.to_string(),
            to: to.to_string(),
            distance,
        })
    }
}

impl From<&str> for Distance {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

struct DistanceMatrix {
    distances: Array2<u32>,
    city_index: HashMap<String, usize>,
}

impl DistanceMatrix {
    fn new(distances: Vec<Distance>) -> Self {
        let mut city_index = HashMap::new();
        let mut index: usize = 0;
        for Distance { from, to, .. } in &distances {
            city_index.entry(from.clone()).or_insert_with(|| {
                let current_index = index;
                index += 1;
                current_index
            });
            city_index.entry(to.clone()).or_insert_with(|| {
                let current_index = index;
                index += 1;
                current_index
            });
        }

        let n = city_index.len();
        let mut distance_matrix = Array2::<u32>::from_elem((n, n), u32::MAX);

        for Distance { from, to, distance } in distances {
            let from_idx = city_index[&from];
            let to_idx = city_index[&to];
            distance_matrix[[from_idx, to_idx]] = distance;
            distance_matrix[[to_idx, from_idx]] = distance;
        }

        Self {
            distances: distance_matrix,
            city_index,
        }
    }

    #[allow(unused)]
    fn show_possible_routes(&self) {
        let index_to_city: Vec<_> = self
            .city_index
            .iter()
            .sorted_by_key(|&(_, &index)| index)
            .map(|(city, _)| city.clone())
            .collect();

        for perm in index_to_city.iter().permutations(index_to_city.len()) {
            let route = perm.into_iter().cloned().collect::<Vec<_>>();
            let distance = self.calculate_route_distance(&route);
            let route_string = route.join(" -> ");
            println!("{} = {}", route_string, distance);
        }
    }

    fn get_shortest_route(&self) -> u32 {
        let mut shortest_distance = u32::MAX;
        for perm in self.city_index.keys().permutations(self.city_index.len()) {
            let route = perm.into_iter().cloned().collect::<Vec<_>>();
            let distance = self.calculate_route_distance(&route);
            if distance < shortest_distance {
                shortest_distance = distance;
            }
        }
        shortest_distance
    }

    fn get_longest_route(&self) -> u32 {
        let mut longest_distance = u32::MIN;
        for perm in self.city_index.keys().permutations(self.city_index.len()) {
            let route = perm.into_iter().cloned().collect::<Vec<_>>();
            let distance = self.calculate_route_distance(&route);
            if distance > longest_distance {
                longest_distance = distance;
            }
        }
        longest_distance
    }

    fn calculate_route_distance(&self, route: &[String]) -> u32 {
        route
            .windows(2)
            .map(|pair| {
                let from_index = self.city_index[&pair[0]];
                let to_index = self.city_index[&pair[1]];
                self.distances[[from_index, to_index]]
            })
            .try_fold(0u32, |acc, distance| acc.checked_add(distance))
            .unwrap_or_else(|| panic!("Disconnected path"))
    }
}

#[aoc(day9, part1)]
fn solve_one(input: &str) -> u32 {
    let distances = input.lines().map(Distance::from).collect();
    let matrix = DistanceMatrix::new(distances);

    matrix.get_shortest_route()
}

#[aoc(day9, part2)]
fn solve_two(input: &str) -> u32 {
    let distances = input.lines().map(Distance::from).collect();
    let matrix = DistanceMatrix::new(distances);

    matrix.get_longest_route()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    #[test]
    fn example_part_one() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        assert_eq!(solve_one(input), 605);
    }

    #[test]
    fn simple_case_part_one() {
        let input = "A to B = 1\nB to C = 1\nC to A = 1";
        assert_eq!(solve_one(input), 2);
    }

    #[test]
    fn complex_case_part_one() {
        let input = "\
A to B = 1
A to C = 2
A to D = 3
B to C = 4
B to D = 5
C to D = 6"
            .trim();
        assert_eq!(solve_one(input), 8);
    }

    #[test]
    #[should_panic(expected = "Disconnected path")]
    fn disconnected_part_one() {
        // Test disconnected paths; expect handling of no complete path
        let input = "A to B = 5\nB to C = 6";
        solve_one(input);
    }

    #[test]
    fn edge_case_part_one() {
        // Test handling of a single line / city / degenerate cases
        let input = "A to A = 0";
        assert_eq!(solve_one(input), 0);
    }

    #[test]
    fn example_part_two() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        assert_eq!(solve_two(input), 982);
    }

    #[test]
    fn simple_case_part_two() {
        let input = "A to B = 1\nB to C = 1\nC to A = 1";
        assert_eq!(solve_two(input), 2);
    }

    #[test]
    fn complex_case_part_two() {
        let input = "\
A to B = 1
A to C = 2
A to D = 3
B to C = 4
B to D = 5
C to D = 6"
            .trim();
        assert_eq!(solve_two(input), 13);
    }

    #[test]
    #[should_panic(expected = "Disconnected path")]
    fn disconnected_part_two() {
        // Test disconnected paths; expect handling of no complete path
        let input = "A to B = 5\nB to C = 6";
        solve_two(input);
    }

    #[test]
    fn edge_case_part_two() {
        // Test handling of a single line / city / degenerate cases
        let input = "A to A = 0";
        assert_eq!(solve_two(input), 0);
    }

    const INPUT: &str = input!("day9");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT), 251);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT), 898);
    }
}
