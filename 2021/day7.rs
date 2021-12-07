use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let crabs:Vec<usize> = contents.split(',').map(|value| value.parse().unwrap()).collect();
	let mut costs:Vec<isize> = Vec::new();
	let mut actual_costs:Vec<isize> = Vec::new();
	
	let min = *crabs.iter().min().unwrap();
	let max = *crabs.iter().max().unwrap();

	for position in min..max
	{
		let mut cost = 0;
		let mut actual_cost = 0;
		for crab in &crabs
		{
			let distance = (*crab as isize - position as isize).abs();
			cost += distance;
			actual_cost += distance + distance * (distance - 1) / 2;
		}
		costs.push(cost);
		actual_costs.push(actual_cost);
	}
	
	println!("First star: {:?}", costs.iter().min().unwrap());
	println!("Second star: {:?}", actual_costs.iter().min().unwrap());
}
