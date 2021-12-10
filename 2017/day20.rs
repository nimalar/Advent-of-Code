use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Particle
{
	vector : Vec<Vec<i32>>,
	distance : i32,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);
	let mut particles:Vec<Particle> = Vec::new();
	
	// read lines to vector
	for line in reader.lines()
	{	
		let mut particle_vector:Vec<Vec<i32>> = Vec::new();
		let l = line.unwrap();
		let input:Vec<&str> = l.split('<').collect();
		for i in 1..input.len()
		{
			let mut vector_part:Vec<i32> = Vec::new();
			let values:Vec<&str> = input[i].split(',').collect();
			for j in 0..3
			{
				let rubbish:&[_] = &['<', '>'];
				vector_part.push(values[j].trim_matches(rubbish).parse().unwrap());
			}
			particle_vector.push(vector_part);
		}
		particles.push(Particle{vector:particle_vector, distance:0});
	}
	
	// start simulation
	loop
	{
		let mut min_dist:i32 = -1;
		let mut min_part:i32 = -1;
		let mut destroyed:Vec<i32> = Vec::new();
		let mut i = 0;
		while i < particles.len()
		{
			particles[i].vector[1][0] += particles[i].vector[2][0];
			particles[i].vector[1][1] += particles[i].vector[2][1];
			particles[i].vector[1][2] += particles[i].vector[2][2];
			particles[i].vector[0][0] += particles[i].vector[1][0];
			particles[i].vector[0][1] += particles[i].vector[1][1];
			particles[i].vector[0][2] += particles[i].vector[1][2];
			particles[i].distance = particles[i].vector[0][0].abs() 
				+ particles[i].vector[0][1].abs() 
				+ particles[i].vector[0][2].abs();
			if min_dist == -1 || particles[i].distance < min_dist
			{
				min_dist = particles[i].distance;
				min_part = i as i32;
			}
			for j in 0..i
			{
				if particles[i].vector[0][0] == particles[j].vector[0][0]
					&& particles[i].vector[0][1] == particles[j].vector[0][1]
					&& particles[i].vector[0][2] == particles[j].vector[0][2]
				{
					if !destroyed.contains(&(j as i32))
					{
						destroyed.push(j as i32);
					}
					particles.remove(i);
					i -= 1;
					break;
				}
			}
			i += 1;
			
		}
		for x in (0..destroyed.len()).rev()
		{
			particles.remove(destroyed[x] as usize);
		}		
		println!("{} {}", min_part, particles.len());
	}
}
