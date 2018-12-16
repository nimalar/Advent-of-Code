use std::env;
use std::fs::File;
use std::io::prelude::*;

fn children(numbers:&[i32], start_index:usize, sum:&mut i32) -> usize
{
	let child_count = numbers[start_index];
	let meta_data_count = numbers[start_index+1] as usize;
	let mut new_index = start_index + 2;
	let mut meta_counts = meta_data_count;
	for _i in 0..child_count
	{
		let value = children(&numbers, new_index, sum);
		meta_counts += value + 2;
		new_index += value + 2;
	}
	for i in 0..meta_data_count
	{
		*sum += numbers[new_index+i];
	}
	return meta_counts;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	
	let mut f = File::open(filename).expect("File not found");
	
	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut numbers:Vec<i32> = Vec::new();
	
	for number in contents.split_whitespace()
	{
		numbers.push(number.parse().unwrap());
	}
	
	let mut sum = 0;
	
	children(&numbers, 0, &mut sum);
	
	println!("{}", sum);
}

