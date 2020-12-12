use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Ship
{
	directions : Vec<i32>,
	waypoint : Vec<i32>,
	facing : char,
}

fn get_direction_index(direction:char) -> usize
{
	match direction
	{
		'E' => return 0,
		'S' => return 1,
		'W' => return 2,
		'N' => return 3,
		_ => return 4,
	}
}

fn turn_ship(ship:&Ship, clockwise:bool, value:i32) -> usize
{
	let mut turn_amount = value / 90;
	if !clockwise
	{
		turn_amount = -1 * turn_amount + 4;
	}
	return (get_direction_index(ship.facing) + turn_amount as usize) % 4;
}

fn turn_waypoint(ship:&Ship, clockwise:bool, value:i32) -> Vec<i32>
{
	let mut turn_amount = value / 90;
	if !clockwise
	{
		turn_amount = -1 * turn_amount + 4;
	}
	let mut result:Vec<i32> = vec![0; 4];
	for i in 0..4
	{
		result[(i + turn_amount as usize) % 4] = ship.waypoint[i];
	}
	return result;
}

fn calculate_distance(distances:&Vec<i32>) -> i32
{
	return (distances[0] - distances[2]).abs() + (distances[1] - distances[3]).abs();
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let directions:Vec<char> = vec!['E', 'S', 'W', 'N'];
	let mut ship_1 = Ship { directions:vec![0; 4], waypoint:vec![0; 4], facing:'E'};
	let mut ship_2 = Ship { directions:vec![0; 4], waypoint:vec![10, 0, 0, 1], facing:'E'};
	
	for row in contents.lines()
	{	
		let direction:char = row[0..1].parse::<char>().unwrap();
		let value:i32 = row[1..].parse().unwrap();
		
		if directions.contains(&direction)
		{
			ship_1.directions[get_direction_index(direction)] += value;
			ship_2.waypoint[get_direction_index(direction)] += value;
		}
		else
		{
			match direction
			{
				'F' => { 
							ship_1.directions[get_direction_index(ship_1.facing)] += value;
							for i in 0..4
							{
								ship_2.directions[i] += value * ship_2.waypoint[i];
							}
						}
				'L' => {
							ship_1.facing = directions[turn_ship(&ship_1, false, value)];
							ship_2.waypoint = turn_waypoint(&ship_2, false, value)
						}
				'R' => {
							ship_1.facing = directions[turn_ship(&ship_1, true, value)];
							ship_2.waypoint = turn_waypoint(&ship_2, true, value)
						}
				_	=> return,
			}
		}
	}

	println!("First star: {}", calculate_distance(&ship_1.directions));
	println!("Second star: {}", calculate_distance(&ship_2.directions));	
}
