use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

	let chars:Vec<char> = contents.chars().collect();
	let mut sum:u32 = 0;
	for x in 0..chars.len()
	{
		let mut y = x + chars.len()/2;
		if y >= chars.len()
		{
			y -= chars.len();
		}
		if chars[x] == chars[y]
		{
			let v = chars[x].to_digit(10).unwrap();
			sum = sum + v;
		}
	}
	println!("summa:{}", sum);
	
}
