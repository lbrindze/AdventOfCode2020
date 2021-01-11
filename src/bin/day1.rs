use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read_ints<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn sum_of_two(filename: &str) -> std::io::Result<u32> {
    let mut val_map: HashMap<u32, u32> = HashMap::new();

    for num in read_ints(File::open(filename)?)? {
        if val_map.contains_key(&num) {
            return Ok(val_map.get(&num).unwrap() * num);
        }
        val_map.insert(2020 - num, num);
    }
    Ok(0)
}

fn sum_of_three(filename: &str) -> std::io::Result<u32> {
    let mut val_map: HashMap<u32, u32> = HashMap::new();
    let nums = read_ints(File::open(filename)?)?;

    for i in &nums {
        for j in &nums {
            // first check if the number is in our HashMap of possible valid answers
            // take the complement
            let num = 2020 - i;
            // then look in our HashMap
            if val_map.contains_key(&num) {
                // return the product of the values
                return Ok(val_map.get(&num).unwrap() * i);
            } else {
                // Save the partial sum as a candidate pair
                let partial_sum = i + j;
                // we only care, of course if the sum is smalller than 2020
                // otherwise the solution is eliminated
                if 2020 > partial_sum {
                    val_map.insert(partial_sum, i * j);
                }
            }
        }
    }
    Ok(0)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let sum_count = &args[2].parse::<u32>().unwrap();
    let solution = match sum_count {
        2 => sum_of_two(filename).unwrap(),
        3 => sum_of_three(filename).unwrap(),
        _ => 0,
    };
    if solution > 0 {
        println!("Solution is {}.", solution);
    } else {
        println!("No solution found, sorry...");
    }

    Ok(())
}
