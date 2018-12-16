use std::env;
use std::fs::File;
use std::io::prelude::*;

fn children(numbers:&[i32], start_index:usize) -> (usize, i32)
{
	let child_count = numbers[start_index];
	let meta_data_count = numbers[start_index+1] as usize;
	let mut new_index = start_index + 2;
	let mut meta_counts = meta_data_count;
	let mut child_values:Vec<i32> = Vec::new();
	for _i in 0..child_count
	{
		let (value, actual_value) = children(&numbers, new_index);
		child_values.push(actual_value);
		meta_counts += value + 2;
		new_index += value + 2;
	}
	let mut actual_value = 0;
	for i in 0..meta_data_count
	{
		let meta_data_value = numbers[new_index+i];
		if child_count == 0
		{
			actual_value += meta_data_value;
		}
		else
		{
			if numbers[new_index+i] <= child_count
			{
				actual_value += child_values[meta_data_value as usize - 1];
			}
		}
	}
	return (meta_counts, actual_value);
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
	
	let (_value, actual_value) = children(&numbers, 0);
	
	println!("{}", actual_value);
}

