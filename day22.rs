use std::collections::HashMap;

fn dijkstra(graph:&Vec<Vec<usize>>, origin:(i32,i32, char), destination:(i32,i32, char))
{
	// Initializations
	let mut q:HashMap<(i32, i32, char), usize> = HashMap::new(); // (x, y, tool), dist
	let mut previous:HashMap<(i32, i32, char), (i32, i32, char)> = HashMap::new();
	for y in 0..graph.len()
	{
		for x in 0..graph[y].len()
		{
			// different nodes for all tool options
			let terrain = graph[y as usize][x as usize];
			if terrain % 2 == 0
			{
				q.insert((x as i32, y as i32, 't'), 9999); // "infinity"	
			}
			if terrain > 0
			{
				q.insert((x as i32, y as i32, 'n'), 9999); // "infinity"	
			}
			if terrain < 2
			{
				q.insert((x as i32, y as i32, 'c'), 9999); // "infinity"	
			}
			
			// previous node undefined
		}
	}
	// distance to source is 0 and first tool is torch
	q.insert(origin, 0);
	
	while !q.is_empty()
	{
		// find vertex with smallest dist
		let mut smallest = 9999;
		let mut u = (0, 0, 'x');
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
		else if u == destination
		{
			println!("{:?}", q.get(&u).unwrap());
			break; // destination reached
		}
		
		let current = q.remove(&u).unwrap();
		
		// for each neighbour of u, calculate distance
		for y in u.1 -1..u.1 +2
		{
			for x in u.0 -1..u.0 +2
			{
				for t in 0..3
				{
					let c:char;
					match t
					{
						0 => c = 't',
						1 => c = 'c',
						2 => c = 'n',
						_ => c = 'x',
					}
					if x >= 0 && y >= 0 && (x == u.0 || y == u.1) && q.contains_key(&(x, y, c))
					{
						let mut cost = current; // init with previous time
						let tool = u.2; // init with previous tool
						if tool != c && x == u.0 && y == u.1 // stay in same place, switch tool
						{
							cost += 7; // tool change costs 7 min
						}
						else if tool == c
						{
							cost += 1; // move without changing tool
						}
						else
						{
							continue;
						}
						// if found distance is smaller than previous, replace it
						if cost < *q.get(&(x, y, c)).unwrap()
						{
							q.insert((x, y, c), cost);
							previous.insert((x, y, c), u);
						}
					}
				}
			}
		}		
	}
}

fn main() 
{
	let depth = 11817;
	let x_target = 9;
	let y_target = 751;
	let x_size = x_target + 20;
	let y_size = y_target + 20;

	let mut geo:Vec<Vec<usize>> = vec![vec![0; x_size]; y_size];
	let mut cave:Vec<Vec<usize>> = vec![vec![0; x_size]; y_size];
	let mut geo_index:usize;
	let mut risk_level = 0;
	
	for y in 0..y_size
	{
		for x in 0..x_size
		{
			if (x == 0 && y == 0) || (x == x_target && y == y_target)
			{
				geo_index = 0;
			}
			else if y == 0
			{
				geo_index = 16807 * x;
			}
			else if x == 0
			{
				geo_index = 48271 * y;
			}
			else
			{
				geo_index = geo[y][x-1] * geo[y-1][x];
			}
			
			geo[y][x] = (geo_index + depth) % 20183;
			cave[y][x] = geo[y][x] % 3;
			if x <= x_target && y <= y_target
			{
				risk_level += cave[y][x];
			}
		}
	}
	println!("{:?}", risk_level);
	
	dijkstra(&cave, (0,0, 't'), (x_target as i32, y_target as i32, 't'));
}
