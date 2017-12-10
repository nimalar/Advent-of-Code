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
	let mut lengths:Vec<u8> = Vec::new();
	for char in contents.chars()
	{
		lengths.push(char as u8);
	}
	lengths.push(17);
	lengths.push(31);
	lengths.push(73);
	lengths.push(47);
	lengths.push(23);
	
	// list of positions
	let mut list:Vec<i32> = Vec::new();
	for i in 0..256
	{
		list.push(i);
	}	
	
	let mut position:usize = 0;
	let mut skip_size = 0;
	
	for _x in 0..64
	{
		for y in 0..lengths.len()
		{
			let length_int:i32 = lengths[y] as i32;
			let mut new_list:Vec<i32> = Vec::new();
			
			// reverse
			for i in (position..position+length_int as usize).rev()
			{
				let mut index:usize = i as usize;
				while index >= list.len()
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
				while index >= list.len()
				{
					index -= list.len();
				}
				list[index] = new_list[j];
				j += 1;
			}
			
			position += length_int as usize + skip_size;
			while position >= list.len()
			{
				position -= list.len();
			}
			skip_size += 1;
		}
	}
	// xor hash
	let mut xor:Vec<i32> = Vec::new();
	for x in 0..16
	{
		let mut xorred = 0;
		for y in 0..16
		{
			xorred ^= list[x*16+y as usize];
		}
		xor.push(xorred);
	}
	for value in xor
	{
		print!("{:02x}", value);
	}
}
