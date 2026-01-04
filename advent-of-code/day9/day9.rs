
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn calculate_area(t1 : &(i64,i64), t2 : &(i64,i64)) -> i64 {

    let (t1x, t1y) = t1;
    let (t2x, t2y) = t2;
    return (t1x - t2x + 1).abs() * (t1y - t2y + 1).abs();

}

fn get_perimeter_tiles(grid : &[i64]) -> [i64] {

}


fn in_shape(t1 : &(i64,i64), t2 : &(i64,i64), grid : &[(i64,i64)]) -> bool {
    let (t1x, t1y) = t1;
    let (t2x, t2y) = t2;

    // get other two corners to check if they are within the green/red tiles
    let (t3x, t3y) = (t1x, t2y);
    let (t4x, t4y) = (t2x, t1y);

    // method of checking:
    //  for each of the two other corners:
    //      traverse in the opposite directions from the red tiles,
    //      if row/column in the direction of inspection contains another red tile
    //          point is in shape
    //      else point is not in shape

    for (tx,ty) in grid {
        
    }


}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("advent-of-code/day9/tiles.txt") {

        let mut tiles : Vec<(i64,i64)> = Vec::new();
        let mut max_area : i64 = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let x : i64 = line.split(",").nth(0).unwrap().parse::<i64>().unwrap();
            let y : i64 = line.split(",").nth(1).unwrap().parse::<i64>().unwrap();
            
            tiles.push((x,y));

        }

        perimeter : Vec<(i64,i64)> = get_perimeter_tiles(tiles);
        
        for tile1 in tiles.iter() {
            for tile2 in tiles.iter() {
                if in_shape(tile,tile2,perimeter) {
                    let area : i64 = calculate_area(tile1, tile2);
                    if area > max_area {
                        max_area = area;
                    }
                }
                // println!("{:?} * {:?} = {} compared {}", tile1, tile2, area, max_area);
            }  
        }
            
        println!("{}",max_area);

    }
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}