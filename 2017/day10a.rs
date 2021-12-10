use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
		
	// the seq of lengths
	let lengths:Vec<&str> = contents.split(',').collect();
	
	// list of positions
	let mut list:Vec<i32> = Vec::new();
	for i in 0..256
	{
		list.push(i);
	}
	
	let mut position:usize = 0;
	let mut skip_size = 0;
	
	for length in lengths
	{
		let length_int:i32 = length.parse().unwrap();
		let mut new_list:Vec<i32> = Vec::new();
		
		// reverse
		for i in (position..position+length_int as usize).rev()
		{
			let mut index:usize = i as usize;
			if index >= list.len()
			{
				index -= list.len();
			}
			new_list.push(list[index]);
		}
		// dump back
		let mut j = 0;
		for i in position..position+length_int as usize
		{
			let mut index:usize = i as usize;
			if index >= list.len()
			{
				index -= list.len();
			}
			list[index] = new_list[j];
			j += 1;
		}
		
		position += length_int as usize + skip_size;
		if position >= list.len()
		{
			position -= list.len();
		}
		skip_size += 1;
	}
	println!("{}", list[0] * list[1]);
}
