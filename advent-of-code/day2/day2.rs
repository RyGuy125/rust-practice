
use std::fs::read_to_string;

type Range = [u32; 2];
type Pair = [u64; 2];

/// Sets in part one.
const FIRST: [Range; 5] = [[2, 1], [4, 2], [6, 3], [8, 4], [10, 5]];
/// Sets in part two.
const SECOND: [Range; 6] = [[3, 1], [5, 1], [6, 2], [7, 1], [9, 3], [10, 2]];
/// Overlap between sets in part one and part two.
const THIRD: [Range; 2] = [[6, 1], [10, 1]];

pub fn parse(input: &str) -> Vec<Pair> {
    let ranges = input.split(",");
    let pairs : Vec<Pair> = vec::new();
    for range in ranges {
        pair.append(range.split("-").str_iter().parse::<u64>().unwrap());
    }
    pairs
}

pub fn part1(input: &[Pair]) -> u64 {
    sum(&FIRST, input)
}

pub fn part2(input: &[Pair]) -> u64 {
    sum(&FIRST, input) + sum(&SECOND, input) - sum(&THIRD, input)
}
fn sum(ranges: &[Range], input: &[Pair]) -> u64 {

    0
}

fn main() {

    if let Ok(lines) = read_to_string("advent-of-code/day2/ids.txt") {
        println!("{}",lines);
        // Consumes the iterator, returns an (Optional) String
        // for line in lines.map_while(Result::ok) {
        //     let ranges : Vec<&str> = line.split(",").collect();
        //     for range in ranges {
        //         let ids : Vec<&str> = range.split("-").collect();
        //         let lower_bound : u64 = ids[0].parse::<u64>().unwrap();
        //         let upper_bound : u64 = ids[1].parse::<u64>().unwrap();
        //         for n in lower_bound..upper_bound+1 {
        //             sum_invalid_ids += if is_invalid(&n) { n } else { 0 };
        //         }
        //     }
        // }
        // println!("Total: {}",sum_invalid_ids);
    }
}

// fn read_lines<P>(filename : P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }