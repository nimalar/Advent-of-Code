use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn eventually_contains(ref fields:&HashMap<&str, HashMap<&str, usize>>, key:&str, to_be_found:&str) -> usize
{
	let inside_bags = fields.get(&key).unwrap();
	if inside_bags.contains_key(to_be_found)
	{
		return 1;
	}
	for inside_key in inside_bags.keys()
	{
		if eventually_contains(&fields, inside_key, to_be_found) > 0
		{
			return 1;
		}
	}
	return 0;
}

fn required_bags(ref fields:&HashMap<&str, HashMap<&str, usize>>, key:&str) -> usize
{
	let mut result = 1;
	let inside_bags = fields.get(&key).unwrap();
	for inside_key in inside_bags.keys()
	{
		result += inside_bags.get(inside_key).unwrap() * required_bags(&fields, inside_key);
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
	let mut bags:HashMap<&str, HashMap<&str, usize>> = HashMap::new();
	let mut result = 0;
	
	for row in contents.lines()
	{
		let outsides:Vec<&str> = row.trim().trim_end_matches('.').split(" contain ").collect();
		let insides:Vec<&str> = outsides[1].split(", ").collect();
		let mut inside_hash:HashMap<&str, usize> = HashMap::new();
		for i in 0..insides.len()
		{
			if insides[i] != "no other bags"
			{
				inside_hash.insert(&insides[i][2..].trim_end_matches('s'), insides[i][0..1].parse::<usize>().unwrap());
			}
		}
		bags.insert(outsides[0].trim_end_matches('s'), inside_hash);
	}
	for key in bags.keys()
	{
		 result += eventually_contains(&bags, key, "shiny gold bag");
	}
	
	println!("First star: {}", result);
	println!("Second star: {}", required_bags(&bags, "shiny gold bag") - 1);
}
