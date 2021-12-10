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
		let mut failed = false;
		let l = line.unwrap();
		let mut vec:Vec<String> = Vec::new();
		for word in l.split_whitespace()
		{
			for old_word in &vec
			{
				if old_word == word
				{
					failed = true;
					break;
				}
			}
			vec.push(word.to_string());
		}
		if !failed
		{
			summa += 1;
		}
	}
	println!("summa:{}", summa);
}
