use std::env;
use std::fs::read_to_string;

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");	
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
