use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp;

fn match_vertical(to_be_matched:Vec<Vec<char>>, ref mut matching:&mut Vec<Vec<char>>, index:usize, can_rotate:bool) -> bool
{
	let mut rotated = false;
	loop
	{
		if to_be_matched[index] == matching[matching.len() - 1 - index]
		{
			return true;
		}
		else if to_be_matched[index].iter().eq(matching[matching.len() - 1 - index].iter().rev())
		{
			for i in 0..matching.len()
			{
				matching[i].reverse();
			}
			return true;
		}
		else if to_be_matched[index] == matching[index]
		{
			let matching_copy = matching.to_vec();
			for i in 0..matching.len()
			{
				matching[matching_copy.len() - i - 1] = matching_copy[i].to_vec();
			}
			return true;
		}
		else if to_be_matched[index].iter().eq(matching[index].iter().rev())
		{
			let matching_copy = matching.to_vec();
			for i in 0..matching.len()
			{
				matching[matching_copy.len() - i - 1] = matching_copy[i].to_vec();
				matching[matching_copy.len() - i - 1].reverse();
			}
			return true;
		}
		else if can_rotate && !rotated
		{
			let matching_copy = matching.to_vec();
			for i in 0..matching.len()
			{
				for j in 0..matching.len()
				{
					matching[i][j] = matching_copy[j][i];
				}
			}
			rotated = true;
		}
		else if !can_rotate || rotated
		{
			break;
		}
	}
	return false;	
}

fn match_horizontal(to_be_matched:Vec<Vec<char>>, ref mut matching:&mut Vec<Vec<char>>, index:usize, can_rotate:bool) -> bool
{
	let mut rotated = false;
	loop
	{
		let mut to_be_matched_long:Vec<char> = Vec::new();
		let mut matching_long_far:Vec<char> = Vec::new();
		let mut matching_long_near:Vec<char> = Vec::new();
		for i in 0..to_be_matched[index].len()
		{
			to_be_matched_long.push(to_be_matched[i][index]);
			matching_long_far.push(matching[i][index]);
			matching_long_near.push(matching[i][to_be_matched.len() - 1 - index]);
		}

		if to_be_matched_long == matching_long_near
		{
			return true;
		}
		else if to_be_matched_long.iter().eq(matching_long_near.iter().rev())
		{
			let matching_copy = matching.to_vec();
			for i in 0..matching.len()
				{
				matching[i] = matching_copy[matching_copy.len() - 1 - i].to_vec();
			}
			return true;
		}
		else if to_be_matched_long == matching_long_far
		{
			for i in 0..matching.len()
			{
				matching[i].reverse();
			}
			return true;
		}
		else if to_be_matched_long.iter().eq(matching_long_far.iter().rev())
		{
			let matching_copy = matching.to_vec();
			for i in 0..matching.len()
			{
				matching[matching_copy.len() - i - 1] = matching_copy[i].to_vec();
				matching[matching_copy.len() - i - 1].reverse();
			}
			return true;
		}
		else if can_rotate && !rotated
		{
			let matching_copy = matching.to_vec();
			for i in 0..matching.len()
			{
				for j in 0..matching.len()
				{
					matching[i][j] = matching_copy[j][i];
				}
			}
			rotated = true;
			continue;
		}
		else if !can_rotate || rotated
		{
			break;
		}
	}
	return false;
}



fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut id = 0;
	let mut photos:HashMap<usize, Vec<Vec<char>>> = HashMap::new();
	let mut square:HashMap<(isize, isize), usize> = HashMap::new();
	let mut matched:HashMap<usize, Vec<(isize, isize)>> = HashMap::new();
	let mut keys:Vec<usize> = Vec::new();
	let mut min_index = (0, 0);
	let mut max_index = (0, 0);
	
	for row in contents.lines()
	{
		if row.contains("Tile")
		{
			id = row.trim().trim_start_matches("Tile ").trim_end_matches(":").parse().unwrap();
			photos.insert(id, Vec::new());
			keys.push(id);
			continue;
		}
		else if row.trim() != ""
		{
			photos.get_mut(&id).unwrap().push(row.trim().chars().collect());
		}
	}
	loop
	{
		'outer: for key in 0..keys.len()
		{
			let mut index = (0, 0);
			if square.is_empty()
			{
				square.insert(index, keys[key]);
				continue;
			}
			match square.values().find(|&&x| x == keys[key])
			{
				Some(_x) => continue,
				None => {},
			}
			let mut found = false;
			for square_key in square.keys()
			{
				let to_be_matched = photos.get(square.get(&square_key).unwrap()).unwrap().to_vec();
				let mut matching = photos.get_mut(&keys[key]).unwrap();
				let mut already_tested:Vec<(isize, isize)> = Vec::new();
				match matched.get(&key)
				{
					Some(x) => already_tested = x.to_vec(),
					None => {},
				}
				if !square.contains_key(&(square_key.0, square_key.1 - 1)) && !already_tested.contains(&(square_key.0, square_key.1 - 1))
				{
					index = (square_key.0, square_key.1 - 1);
					found = match_vertical(to_be_matched.to_vec(), &mut *matching, 0, true);
				}
				if !found && !square.contains_key(&(square_key.0, square_key.1 + 1)) && !already_tested.contains(&(square_key.0, square_key.1 + 1))
				{
					index = (square_key.0, square_key.1 + 1);
					found = match_vertical(to_be_matched.to_vec(), &mut *matching, 10 - 1, true);
				}
				if !found && !square.contains_key(&(square_key.0 - 1, square_key.1)) && !already_tested.contains(&(square_key.0 - 1, square_key.1))
				{
					index = (square_key.0 - 1, square_key.1);
					found = match_horizontal(to_be_matched.to_vec(), &mut *matching, 0, true);
				}
				if !found && !square.contains_key(&(square_key.0 + 1, square_key.1)) && !already_tested.contains(&(square_key.0 + 1, square_key.1))
				{
					index = (square_key.0 + 1, square_key.1);
					found = match_horizontal(to_be_matched.to_vec(), &mut *matching, 10 - 1, true);
				}
				if found
				{
					break;
				}
			}
			if found
			{
				for i in 0..3
				{
					for j in 0..3
					{
						if (i == 1) != (j == 1)
						{
							let square_index = (index.0 + i - 1, index.1 + j - 1);
							if square.contains_key(&(square_index))
							{							
								let to_be_matched_2 = photos.get(square.get(&square_index).unwrap()).unwrap().to_vec();
								let mut matching = photos.get_mut(&keys[key]).unwrap();
								if i == 1 && j == 2
								{
									found = match_vertical(to_be_matched_2.to_vec(), &mut *matching, 0, false);
								}
								if i == 1 && j == 0
								{
									found = match_vertical(to_be_matched_2.to_vec(), &mut *matching, 9, false);
								}
								if i == 2 && j == 1
								{
									found = match_horizontal(to_be_matched_2.to_vec(), &mut *matching, 0, false);
								}
								if i == 0 && j == 1
								{
									found = match_horizontal(to_be_matched_2.to_vec(), &mut *matching, 9, false);
								}
							}
						}
					}
				}
				if !found
				{
					let vecs = matched.entry(keys[key]).or_insert(Vec::new());
					vecs.push(index);
				}
			}
			if found && !square.contains_key(&index)
			{
				square.insert(index, keys[key]);
				max_index = (cmp::max(index.0, max_index.0), cmp::max(index.1, max_index.1));
				min_index = (cmp::min(index.0, min_index.0), cmp::min(index.1, min_index.1));
			}			
		}
		if square.len() == photos.len()
		{
			break;
		}
	}
	println!("First star: {:?}", square.get(&min_index).unwrap() * square.get(&max_index).unwrap() * square.get(&(min_index.0, max_index.1)).unwrap() * square.get(&(max_index.0, min_index.1)).unwrap() );

	let mut ultimate:Vec<Vec<char>> = Vec::new();
	for i in min_index.1..max_index.1+1
	{
		for j in min_index.0..max_index.0+1
		{
			let rows = photos.get(&square.get(&(j, i)).unwrap()).unwrap();
			for row in 1..rows.len()-1
			{
				if j == min_index.0
				{
					ultimate.push(rows[row][1..9].to_vec());
				}
				else
				{
					let row_index:usize = ((i - min_index.1) * (rows.len() - 2) as isize + row as isize - 1) as usize;
					ultimate[row_index].extend_from_slice(&rows[row][1..9]);
				}
			}
		}
	}
	
	//                   # 
	// #    ##    ##    ###
	//  #  #  #  #  #  #   
	
	loop
	{
		let mut monsters = 0;
		let mut hashtags = 0;
		for i in 0..ultimate.len()
		{
			for j in 0..ultimate[i].len()
			{
				if i > 0 && i < ultimate.len() - 1 && j < ultimate[i].len() - 20
				{
					if ultimate[i][j] == '#' && ultimate[i+1][j+1] == '#' && ultimate[i+1][j+4] == '#' && ultimate[i][j+5] == '#' && ultimate[i][j+6] == '#' && ultimate[i+1][j+7] == '#' && ultimate[i+1][j+10] == '#' && ultimate[i][j+11] == '#' && ultimate[i][j+12] == '#' && ultimate[i+1][j+13] == '#' && ultimate[i+1][j+16] == '#' && ultimate[i][j+17] == '#' && ultimate[i][j+18] == '#' && ultimate[i][j+19] == '#' && ultimate[i-1][j+18] == '#'
					{
						monsters += 1;
					}
				}
				if ultimate[i][j] == '#'
				{
					hashtags += 1;
				}
			}
		}
		
		if monsters == 0
		{			
			for i in 0..ultimate.len()
			{
				ultimate[i].reverse();
			}
			let ultimate_copy = ultimate.to_vec();
			for i in 0..ultimate.len()
			{
				for j in 0..ultimate[i].len()
				{
					ultimate[i][j] = ultimate_copy[j][i];
				}
			}
		}
		else
		{
			println!("Second star: {}", hashtags - 15 * monsters);
			break;
		}
	}
}
