use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);
	let mut summa:i32 = 0;

	for line in reader.lines()
	{
		let l = line.unwrap();
		let mut vec:Vec<i32> = Vec::new();
		for number in l.split_whitespace()
		{
			vec.push(number.parse().unwrap());
		}
		for x in 0..vec.len()
		{
			for y in 0..vec.len()
			{
				if vec[x] != vec[y] && vec[x]%vec[y] == 0
				{
					let result = vec[x]/vec[y];
					summa += result as i32;
				}
			}
		}
	}
	println!("summa:{}", summa);
}

