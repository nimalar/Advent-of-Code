use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main()
{	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut input:Vec<u32> = Vec::new();
	let pattern:Vec<i32> = vec![0, 1, 0, -1];
	
	for value in contents.chars()
	{
		input.push(value.to_digit(10).unwrap());
	}
	
	let mut input_vec = input.to_vec();
	for _phase in 0..100
	{
		let mut new_input:Vec<u32> = Vec::new();
		for i in 0..input_vec.len()
		{
			let mut result = 0;
			for j in 0..input_vec.len()
			{
				result += input_vec[j] as i32 * pattern[((j + 1) / (i + 1)) % 4];
			}
			new_input.push((result.abs() % 10) as u32);
		}
		input_vec = new_input;
	}
	println!("First star: {:?}", &input_vec[0..8].into_iter().map(|i| i.to_string()).collect::<String>().parse::<i32>().unwrap());
	
	let offset:usize = input[0..7].into_iter().map(|i| i.to_string()).collect::<String>().parse().unwrap();
	let mut second_input:Vec<u32> = Vec::new();
	for _i in 0..10000
	{
		second_input.extend(input.to_vec());
	}
	let mut input_vec = second_input[offset..].to_vec();
	for _phase in 0..100
	{
		let mut new_input:Vec<u32> = Vec::new();
		let mut result = 0;
		for i in 0..input_vec.len()
		{
			result += input_vec[input_vec.len() - 1 - i] as i32;
			new_input.push((result.abs() % 10) as u32);
		}
		input_vec = new_input;
		input_vec.reverse();
	}
	println!("Second star: {:?}", &input_vec[0..8].into_iter().map(|i| i.to_string()).collect::<String>().parse::<i32>().unwrap());
}
