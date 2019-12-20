use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Asteroid<>
{
	x: f64,
	y: f64,
	angles_right: Vec<f64>,
	angles_left: Vec<f64>,
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut asteroids:Vec<Asteroid> = Vec::new();
	
	let mut row:f64 = 0.0;
	for value in contents.split_whitespace()
	{
		let parts:Vec<char> = value.chars().collect();
		for i in 0..parts.len()
		{
			if parts[i] != '.'
			{
				asteroids.push(Asteroid{x:i as f64, y:row, angles_right:Vec::new(), angles_left:Vec::new()});
			}
		}
		row += 1.0;
	}
	
	let mut result = 0;
	let mut asteroid = 0;
	let mut final_negatives = 0;
	
	for i in 0..asteroids.len()
	{
		let mut negatives = 0;
		for j in 0..asteroids.len()
		{
			let angle:f64 = (asteroids[j].x - asteroids[i].x) as f64 / (asteroids[j].y - asteroids[i].y) as f64;
			if (asteroids[j].x > asteroids[i].x || (asteroids[j].x == asteroids[i].x && asteroids[j].y < asteroids[i].y)) && !asteroids[i].angles_right.contains(&angle)
			{
				asteroids[i].angles_right.push(angle);
			}
			else if (asteroids[j].x < asteroids[i].x || (asteroids[j].x == asteroids[i].x && asteroids[j].y > asteroids[i].y)) && !asteroids[i].angles_left.contains(&angle)
			{
				asteroids[i].angles_left.push(angle);
				if angle <= 0.0
				{
					negatives += 1;
				}
			}
		}
		let amount = asteroids[i].angles_right.len() + asteroids[i].angles_left.len();
		if amount > result
		{
			result = amount;
			asteroid = i;
			final_negatives = negatives;
		}
	}
	let nth = 200 - asteroids[asteroid].angles_right.len() - final_negatives;
	let mut sorted = asteroids[asteroid].angles_left.to_vec();
	sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
	let result_angle = sorted[nth - 1];
	let mut second_result = 0;
	
	for k in 0..asteroids.len()
	{
		let angle:f64 = (asteroids[k].x - asteroids[asteroid].x) as f64 / (asteroids[k].y - asteroids[asteroid].y) as f64;
		if (asteroids[k].x < asteroids[asteroid].x || (asteroids[k].x == asteroids[asteroid].x && asteroids[k].y > asteroids[asteroid].y)) && angle == result_angle
		{
			second_result = k;
		}
	}
	
	println!("First star: {:?}", result);
	println!("Second star: {:?}", (asteroids[second_result].x * 100.0 + asteroids[second_result].y) as i32);
}
