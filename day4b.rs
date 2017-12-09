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
				if word.len() != old_word.len()
				{
					// different lengths, all okay
				}
				else
				{
					// otherwise compare char by char
					let chars:Vec<char> = word.chars().collect();
					let mut old_chars:Vec<char> = old_word.chars().collect();
					
					for x in 0..chars.len()
					{
						for y in 0..old_chars.len()
						{
							if chars[x] == old_chars[y]
							{
								old_chars.remove(y);
								break;
							}
						}
					}
					// after all chars, if old_chars is empty, it's a anagram
					if old_chars.len() == 0
					{
						failed = true;
					}
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
