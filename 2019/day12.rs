use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Moon<>
{
	position: Vec<i32>,
	velocity: Vec<i32>,
}

fn get_primes(mut value:u64) -> Vec<u64>
{
	let mut i = 2;
	let mut result = Vec::new();
	while i < value/2
	{
		if value % i == 0
		{
			value = value / i;
			result.push(i);
		}
		else
		{
			i += 1;
		}
	}
	result.push(value);
	return result;
}

fn main()
{	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut moons:Vec<Moon> = Vec::new();
	let mut original_moons:Vec<Moon> = Vec::new();
	
	for row in contents.lines()
	{
		let mut parts:Vec<&str> = row.split(", ").collect();
		for i in 0..parts.len()
		{
			parts[i] = parts[i].trim_matches(|c| c == 'x' || c == 'y' || c == 'z' || c == '<' || c == '>' || c == '=');
		}
		moons.push(Moon{position:vec![parts[0].parse().unwrap(),parts[1].parse().unwrap(),parts[2].parse().unwrap()], velocity:vec![0,0,0]});
		original_moons.push(Moon{position:vec![parts[0].parse().unwrap(),parts[1].parse().unwrap(),parts[2].parse().unwrap()], velocity:vec![0,0,0]});
		
	}
	
	let mut looper:u64 = 0;
	let mut second_result_vec = vec![0; 3];
	
	loop
	{
		looper += 1;
		// velocity
		for i in 0..moons.len()
		{
			for j in i+1..moons.len()
			{
				for k in 0..3
				{
					if moons[i].position[k] > moons[j].position[k]
					{
						moons[i].velocity[k] -= 1;
						moons[j].velocity[k] += 1;
					}
					else if moons[i].position[k] < moons[j].position[k]
					{
						moons[i].velocity[k] += 1;
						moons[j].velocity[k] -= 1;
					}
				}
			}
		}
		
		//position
		let mut found:Vec<bool> = vec![true, true, true];
		for i in 0..moons.len()
		{
			for k in 0..3
			{
				moons[i].position[k] += moons[i].velocity[k];
				if moons[i].velocity[k] != 0 || moons[i].position[k] != original_moons[i].position[k]
				{
					found[k] = false;
				}
			}
		}
		for k in 0..3
		{
			if found[k] && second_result_vec[k] == 0
			{
				second_result_vec[k] = looper;
			}
		}
		
		if looper == 1000
		{
			let mut result = 0;
			for i in 0..moons.len()
			{
				let mut potential = 0;
				let mut kinetic = 0;
				for k in 0..3
				{
					potential += moons[i].position[k].abs();
					kinetic += moons[i].velocity[k].abs();
				}
				result += potential * kinetic;
			}
			println!("First star: {}", result);
		}
		
		if !second_result_vec.contains(&0)
		{
			let mut primes = get_primes(second_result_vec[0]);
			for h in 1..second_result_vec.len()
			{
				let mut compare = get_primes(second_result_vec[h]);
				for p in 0..primes.len()
				{
					if compare.contains(&primes[p])
					{
						for q in 0..compare.len()
						{
							if compare[q] == primes[p]
							{
								compare.remove(q);
								break;
							}
						}
					}
				}
				primes.append(&mut compare);	
			}
			let mut second_result = 1;
			for i in 0..primes.len()
			{
				second_result *= primes[i];
			}
			println!("Second star: {}", second_result);
			break;
		}
	}
}
