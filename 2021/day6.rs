use std::env;
use std::fs::read_to_string;

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");	
	let mut fish:Vec<usize> = vec![0; 9];
	
	for value in contents.split(',')
	{
		fish[value.parse::<usize>().unwrap()] += 1;
	}
	
	let mut current = 0;
	for day in 0..256
	{
		if day == 80
		{
			println!("First star: {}", fish.iter().sum::<usize>());
		}
		fish[(current + 7) % 9] += fish[current];
		current = (current + 1) % 9;
	}
	
	println!("Second star: {}", fish.iter().sum::<usize>());
}
