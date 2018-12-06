use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
		
	let mut vec:Vec<String> = Vec::new();
	for line in contents.split_whitespace()
	{
		vec.push(line.to_string());
	}
		
	'outer: for i in 0..vec.len()
	{
		let i_chars:Vec<char> = vec[i].chars().collect();
		'inner: for j in 0..vec.len()
		{
			let j_chars:Vec<char> = vec[j].chars().collect();
			let mut different = 0;
			let mut index = 0;
			for k in 0..i_chars.len()
			{
				if i_chars[k] != j_chars[k]
				{
					different += 1;
					index = k;
				}
			}
			if different == 1
			{
				vec[i].remove(index);
				println!("common: {:?}", vec[i]);
				break 'outer;
			}
		}
	}
}
