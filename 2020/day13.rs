use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Bus
{
	number : usize,
	wait_time : usize,
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut buses:Vec<usize> = Vec::new();
	let mut departure = 0;
	
	for row in contents.lines()
	{	
		if !row.contains(',')
		{
			departure = row.parse().unwrap();
			continue;
		}
		let parts:Vec<&str> = row.trim().split(",").collect();
		let mut best_bus:Bus = Bus { number:0, wait_time:0};
		for part in parts
		{
			if part != "x"
			{
				let number:usize = part.parse().unwrap();
				let bus = Bus { number:number, wait_time:(number - (departure % number))};
				if best_bus.number == 0 || bus.wait_time < best_bus.wait_time
				{
					best_bus = bus;
				}
				buses.push(number);
				continue;
			}
			buses.push(0);
		}
		println!("First star: {}", best_bus.wait_time * best_bus.number);
	}
	
	let mut time = 100000000000000;
	let mut time_difference = 1;
	let mut found:Vec<usize> = Vec::new();
	
	'outer: loop
	{
		for i in 0..buses.len()
		{
			if buses[i] != 0 && (time + i) % buses[i] != 0
			{
				time += time_difference;
				continue 'outer;
			}
			else if buses[i] != 0 && !found.contains(&buses[i])
			{
				found.push(buses[i]);
				time_difference *= buses[i];
			}
		}
		println!("Second star: {}", time);
		break;
	}
}
