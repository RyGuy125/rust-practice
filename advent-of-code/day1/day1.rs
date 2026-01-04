
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn checked_turn(pos: i16, spin : i16, flag : i16, counter: i16) -> (i16, i16) {

    if spin >= 100 {
        return checked_turn(pos, spin-100, flag, counter+1);
    }

    let result : i16 = pos + (flag*spin);
    match result {
        0      => (result, counter+1),
        ..0 | 100.. if pos != 0 => (result.rem_euclid(100), counter+1),
        _ => (result.rem_euclid(100), counter)
    }
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("advent-of-code/day1/combinations.txt") {

        let (mut dial_pos, mut count) = (50i16, 0i16);
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {

            let spin : i16 = line[1usize..].parse::<i16>().unwrap(); 
            let prev_dial = dial_pos;
            let direction = line.chars().nth(0).unwrap();

            (dial_pos, count) = match direction {
                'L' => checked_turn(dial_pos, spin, -1i16, count),
                _   => checked_turn(dial_pos, spin,  1i16, count)
            };

            // println!(" Current Number is: {},  Rotating: {}{}, New Number is {:?} with the overflow/0 count {:?})",prev_dial, direction, spin, dial_pos, count);
            println!("{} + {}{} = {:?}:{:?}", prev_dial, direction, spin, dial_pos, count);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}