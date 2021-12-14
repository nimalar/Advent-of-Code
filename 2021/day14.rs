use std::env;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp;

fn calculate_result(ref counters:&HashMap<&str, usize>) -> usize
{
	let mut result:HashMap<char, usize> = HashMap::new();
	let mut result_competitive:HashMap<char, usize> = HashMap::new();
	for (key, val) in counters.iter()
	{
		let mut it = key.chars();
		*result.entry(it.next().unwrap()).or_insert(0) += val;
		*result_competitive.entry(it.next().unwrap()).or_insert(0) += val;	
	}
	return cmp::max(result.values().max().unwrap(), result_competitive.values().max().unwrap()) - cmp::min(result.values().min().unwrap(), result_competitive.values().min().unwrap());
}

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");
	let mut rules:HashMap<String, Vec<String>> = HashMap::new();
	let mut counters:HashMap<&str, usize> = HashMap::new();
	
	for row in contents.lines()
	{
		if row.contains("->")
		{
			let parts:Vec<&str> = row.split(" -> ").collect();
			let first = parts[0][..1].to_string() + &parts[1];
			let second = parts[1].to_string() + &parts[0][1..].to_string();
			rules.insert(parts[0].to_string(), vec![first, second]);
		}
		else if !row.is_empty()
		{
			for i in 0..row.len() - 1
			{
				*counters.entry(&row[i..i+2]).or_insert(0) += 1;
			}
		}
	}
	
	for step in 0..40
	{
		let mut new_counters:HashMap<&str, usize> = HashMap::new();
		for (key, val) in counters.iter()
		{
			for m in rules.get(*key).unwrap()
			{
				*new_counters.entry(m).or_insert(0) += val;
			}
		}
		if step == 9
		{
			println!("First star: {:?}", calculate_result(&new_counters));
		}
		counters = new_counters;
	}
	println!("Second star: {:?}", calculate_result(&counters));
}
