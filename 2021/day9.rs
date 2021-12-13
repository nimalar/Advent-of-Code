use std::env;
use std::fs::read_to_string;
use std::collections::HashSet;

fn calculate_basin(i:usize, j:usize, ref points:&Vec<Vec<u32>>, ref mut indices:&mut HashSet<(usize, usize)>)
{
	if points[i][j] != 9 && !indices.contains(&(i,j))
	{
		indices.insert((i,j));
		if i > 0
		{
			calculate_basin(i-1, j, &points, indices)
		}
		if j > 0
		{
			calculate_basin(i, j-1, &points, indices)
		}
		if i < points.len() - 1
		{
			calculate_basin(i+1, j, &points, indices)
		}
		if j < points[i].len() - 1
		{
			calculate_basin(i, j+1, &points, indices)
		}
	}
}

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");	
	let mut points:Vec<Vec<u32>> = Vec::new();
	let mut basins:Vec<usize> = Vec::new();
	let mut result_1 = 0;
	let mut result_2 = 1;
	
	for row in contents.lines()
	{
		points.push(row.chars().map(|value| value.to_digit(10).unwrap()).collect());
	}
	
	for i in 0..points.len()
	{
		for j in 0..points[i].len()
		{
			if (i <= 0 || points[i][j] < points[i-1][j]) &&
				(j <= 0 || points[i][j] < points[i][j-1]) &&
				(i >= points.len()-1 || points[i][j] < points[i+1][j]) &&
				(j >= points[i].len()-1 || points[i][j] < points[i][j+1])
				{
					result_1 += 1 + points[i][j];
					let mut indices:HashSet<(usize, usize)> = HashSet::new();
					calculate_basin(i, j, &points, &mut indices);
					basins.push(indices.len());
				}
		}
	}
	basins.sort();
	for _i in 0..3
	{
		result_2 *= basins.pop().unwrap();
	}
	
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
