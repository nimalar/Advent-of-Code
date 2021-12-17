use std::env;
use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use std::cmp;

fn dijkstra(graph:&Vec<Vec<u32>>)
{
	// Initializations
	let mut q:HashMap<(i32, i32), u32> = HashMap::new(); // (x, y), dist
	let mut v:HashSet<(i32, i32)> = HashSet::new();

	// distance to source is 0
	q.insert((0,0), 0);
	
	while !q.is_empty()
	{
		// find vertex with smallest dist
		let mut smallest = 9999;
		let mut u = (0, 0);
		for (key, value) in &q
		{
			if value < &smallest
			{
				u = *key;
				smallest = *value;
			}
		}
		if smallest == 9999
		{
			break; // there is no route
		}
		else if u == (graph.len() as i32 - 1, graph[0].len() as i32 - 1)
		{
			println!("{}", q.get(&u).unwrap());
			break; // destination reached
		}
		
		let current = q.remove(&u).unwrap();
		v.insert(u);
		
		// for each neighbour of u, calculate distance
		for y in u.1 -1..u.1 +2
		{
			for x in u.0 -1..u.0 +2
			{
				if x >= 0 && y >= 0 && x < graph.len() as i32 && y < graph[0].len() as i32 && 
					((x == u.0) != (y == u.1)) && !v.contains(&(x, y))
				{
					let mut cost = current; // init with previous risk level
					cost += graph[x as usize][y as usize]; // move
					// if found distance is smaller than previous, replace it
					let entry = q.entry((x, y)).or_insert(9999);
					*entry = cmp::min(cost, *entry);
				}
			}
		}		
	}
}

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");
	let mut cave:Vec<Vec<u32>> = Vec::new();
	
	for row in contents.lines()
	{
		cave.push(row.chars().map(|value| value.to_digit(10).unwrap()).collect());
	}
	
	print!("First star: ");
	dijkstra(&cave);
	
	let cave_len = cave.len();
	let cave_row_len = cave[0].len();
	let mut risk = 1;
	for _x in 0..4
	{
		for i in 0..cave_len
		{
			for j in 0..cave_row_len
			{
				let mut value = cave[i][j] + risk;
				if value > 9
				{
					value -= 9;
				}
				cave[i].push(value);
			}
		}
		risk += 1;
	}
	risk = 1;
	for _y in 0..4
	{
		for i in 0..cave_len
		{
			let mut new_row = Vec::new();
			for j in 0..cave[0].len()
			{
				let mut value = cave[i][j] + risk;
				if value > 9
				{
					value -= 9;
				}
				new_row.push(value);
			}
			cave.push(new_row);
		}
		risk += 1;
	}
	
	print!("Second star: ");
	dijkstra(&cave);
}
