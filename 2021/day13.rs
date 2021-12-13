use std::env;
use std::fs::read_to_string;
use std::collections::HashSet;

fn fold(ref fold:(&str, usize), dots:HashSet<(usize, usize)>) -> HashSet<(usize, usize)>
{
	let mut new_dots:HashSet<(usize, usize)> = HashSet::new();
	for dot in dots.iter()
	{
		if fold.0 == "x" && dot.0 > fold.1
		{
			new_dots.insert((dot.0 - 2 * (dot.0 - fold.1), dot.1));
		}
		else if fold.0 == "y" && dot.1 > fold.1
		{
			new_dots.insert((dot.0, dot.1 - 2 * (dot.1 - fold.1)));
		}
		else
		{
			new_dots.insert(*dot);
		}
	}
	return new_dots;
}

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");
	let mut dots:HashSet<(usize, usize)> = HashSet::new();
	let mut folds:Vec<(&str, usize)> = Vec::new();
	
	for row in contents.lines()
	{
		if row.contains("fold")
		{
			let instruction:Vec<&str> = row.trim_start_matches("fold along ").split("=").collect();
			folds.push((instruction[0], instruction[1].parse().unwrap()));
			continue;
		}
		if !row.is_empty()
		{
			let parts:Vec<usize> = row.split(",").map(|value| value.parse().unwrap()).collect();
			dots.insert((parts[0], parts[1]));
		}
	}
	for i in 0..folds.len()
	{
		dots = fold(folds[i], dots);
		if i == 0
		{
			println!("First star: {:?}", dots.len());
		}
	}
	let mut result:Vec<Vec<char>> = vec![vec![' '; 40]; 6];
	for dot in dots
	{
		result[dot.1][dot.0] = '█';
	}
	println!("Second star:");
	for i in 0..result.len()
	{
		println!("{:?}", result[i].iter().collect::<String>());
	}
}
