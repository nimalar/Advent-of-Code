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
		let mut min:i32 = -1;
		let mut max:i32 = -1;
		for number in l.split_whitespace()
		{
			let number_int:i32 = number.trim().parse().unwrap();
			if min < 0 || number_int < min
			{
				min = number_int;
			}
			if max < 0 || number_int > max
			{
				max = number_int;
			}
		}
		summa += max - min;
	}
	println!("summa:{}", summa);
}
