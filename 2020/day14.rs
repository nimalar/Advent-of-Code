use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn convert_address(ref address:&Vec<char>) -> Vec<usize>
{
	let mut result:Vec<usize> = Vec::new();
	if !address.contains(&'X')
	{
		result.push(usize::from_str_radix(address.iter().collect::<String>().as_str(), 2).unwrap());
		return result;
	}
	for i in 0..address.len()
	{
		if address[i] == 'X'
		{
			let digits:Vec<char> = vec!['0', '1'];
			let mut modified_address = address.to_vec();
			for j in 0..2
			{
				modified_address[i] = digits[j];
				result.append(&mut convert_address(&modified_address));
			}
			return result;
		}
	}
	return result;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut memory_1:HashMap<usize, usize> = HashMap::new();
	let mut memory_2:HashMap<usize, usize> = HashMap::new();
	let mut mask:Vec<char> = Vec::new();
	
	for row in contents.lines()
	{	
		let parts:Vec<&str> = row.split(" = ").collect();
		if parts[0].trim() == "mask"
		{
			mask = parts[1].chars().collect();
			continue;
		}
		let address:usize = parts[0].trim_start_matches("mem[").trim_end_matches("]").parse().unwrap();
		let number:usize = parts[1].trim().parse().unwrap();
		let mut number_as_bin:Vec<char> = format!("{:036b}", number).chars().collect();
		let mut address_as_bin:Vec<char> = format!("{:036b}", address).chars().collect();
		for i in 0..mask.len()
		{
			if mask[i] == '1' || mask[i] == '0'
			{
				number_as_bin[i] = mask[i];
			}
			if mask[i] == '1' || mask[i] == 'X'
			{
				address_as_bin[i] = mask[i];
			}
		}
		let result_addresses:Vec<usize> = convert_address(&address_as_bin);
		for i in 0..result_addresses.len()
		{
			memory_2.insert(result_addresses[i], number);
		}
		memory_1.insert(address, usize::from_str_radix(number_as_bin.into_iter().collect::<String>().as_str(), 2).unwrap());
	}
	
	let mut result = 0;
	for key in memory_1.keys()
	{
		result += memory_1.get(&key).unwrap();
	}
	println!("First star: {}", result);
	
	result = 0;
	for key in memory_2.keys()
	{
		result += memory_2.get(&key).unwrap();
	}
	println!("Second star: {}", result);
}
