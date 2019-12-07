use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashMap;

fn breadth_first_search(squares:&Vec<Vec<(char, i32, i32)>>, targets:&[(usize,usize)], i:usize, j:usize) -> Vec<(usize, usize)>
{
	// a FIFO open_set
	let mut open_set:VecDeque<(usize, usize)> = VecDeque::new();
	
	// an empty set to maintain visited nodes
	let mut closed_set:Vec<(usize, usize)> = Vec::new();
	
	// a dictionary to maintain meta information (used for path formation)
	// key -> (parent state, action to reach child)
	let mut meta: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
	
	// initialize
	let root = (i, j);
	meta.insert(root, root);
	open_set.push_back(root);
	let mut action_list:Vec<(usize, usize)> = Vec::new();
	let mut first_found = false;
	
	// for each node on the current level expand and process, if no children (leaf) then unwind
	while !open_set.is_empty()
	{
		let subtree_root = open_set.pop_front().unwrap();
		
		// we found the node we wanted so stop and emit a path
		if targets.contains(&subtree_root)
		{
			// produce a backtrace of the actions taken to find the goal node, using the recorded meta data dictionary
			let mut action_list_new:Vec<(usize, usize)> = Vec::new();
			
			let mut state = &subtree_root;
			// continue until you reach root meta data
			loop
			{
				let result = meta.get(state).unwrap();
				if state.0 == i && state.1 == j
				{
					break;
				}
				else
				{
					action_list_new.push(*state);
					state = result;
				}
			}
			// no previous list, this is the shortest atm or:
			// same length, determine which is better (y is smaller or y is the same but x is smaller)
			if (!first_found && action_list.is_empty()) || (!action_list.is_empty() && (action_list_new.len() < action_list.len()) || ((action_list_new.len() == action_list.len()) && (action_list_new[0].1 < action_list[0].1 || (action_list_new[0].1 == action_list[0].1 && action_list_new[0].0 < action_list[0].0))))
			{
				action_list = action_list_new.to_vec();
			}
			first_found = true;
		}
		
		// get successors
		
		let x = subtree_root.0;
		let y = subtree_root.1;
		let mut successors:Vec<(usize, usize)> = Vec::new();
		if y > 0 && squares[y-1][x].0 == '.'
		{
			successors.push((x, y-1));
		}
		if x > 0 && squares[y][x-1].0 == '.'
		{
			successors.push((x-1, y));
		}
		if x < squares[y].len() - 1 && squares[y][x+1].0 == '.'
		{
			successors.push((x+1, y));
		}
		if y < squares.len() - 1 && squares[y+1][x].0 == '.'
		{
			successors.push((x, y+1));
		}
		
		// for each child of the current tree process
		for child in successors
		{
			// the node has already been processed so skip over it
			if closed_set.contains(&child)
			{
				continue;
			}
			
			// the child is not enqueued to be processed so enqueue this level of children to be expanded
			if !(open_set.contains(&child))
			{
				meta.insert(child, subtree_root); // create metadata for these nodes
				open_set.push_back(child); // enqueue these nodes
			}
		}
		
		// we finished processing the root of this subtree, so add it to the closed set
		closed_set.push(subtree_root);
	}
	action_list.reverse();
	return action_list;
}


fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	 
	let mut squares:Vec<Vec<(char, i32, i32)>>; //char, tick, hp
	let mut tick;
	let mut attack_power = 4;
	
	'outer:loop
	{
		let mut j = 0;
		squares = Vec::new();
		let mut elves:Vec<(usize, usize)> = Vec::new(); // pos_x, pos_y
		let mut goblins:Vec<(usize, usize)> = Vec::new(); // pos_x, pos_y
		for line in contents.lines()
		{
			let mut chars:Vec<(char, i32, i32)> = Vec::new();
			let mut i = 0;
			for c in line.chars()
			{
				if c == 'E'
				{
					elves.push((i,j));
				}
				else if c  == 'G'
				{
					goblins.push((i,j));
				}
				chars.push((c, 0, 200));
				i += 1;
			}
			squares.push(chars);
			j += 1;
		}
		tick = 0;
		'inner:loop
		{
			for j in 0..squares.len()
			{
				for i in 0..squares[j].len()
				{	
					let mut fighter = squares[j][i];
					if (fighter.0 == 'E' || fighter.0 == 'G') && fighter.1 == tick
					{
						squares[j][i].1 += 1;
						
						// identify targets
						if (fighter.0 == 'E' && goblins.len() == 0)
							|| (fighter.0 == 'G' && elves.len() == 0)
						{
							break 'outer;
						}
						
						// find all squares near targets
						let mut enemies:Vec<(usize, usize)>;
						let mut enemy_char;
						if fighter.0 == 'E'
						{
							enemies = goblins.to_vec();
							enemy_char = 'G';
						}
						else
						{
							enemies = elves.to_vec();
							enemy_char = 'E';
						}
						let mut targets:Vec<(usize,usize)> = Vec::new();
						for k in 0..enemies.len()
						{	
							let x = enemies[k].0;
							let y = enemies[k].1;
							
							if y > 0 && (squares[y-1][x].0 == '.' || (y-1 == j && x == i))
							{
								targets.push((x, y-1));
							}
							if x > 0 && (squares[y][x-1].0 == '.' || (y == j && x-1 == i))
							{
								targets.push((x-1, y));
							}
							if x < squares[y].len() - 1 && (squares[y][x+1].0 == '.' || (y == j && x+1 == i))
							{
								targets.push((x+1, y));
							}
							if y < squares.len() - 1 && (squares[y+1][x].0 == '.' || (y+1 == j && x == i))
							{
								targets.push((x, y+1));
							}
						}
						
						let shortest:Vec<(usize, usize)> = breadth_first_search(&squares, &targets, i, j);
						let mut new_index = (i, j);
						if !shortest.is_empty()
						{
							new_index = *shortest.first().unwrap();
							squares[new_index.1][new_index.0] = squares[j][i];
							squares[j][i] = ('.', tick, 200);
							if enemy_char == 'G'
							{
								elves.retain(|&x| x != (i,j));
								elves.push(new_index);
							}
							else
							{
								goblins.retain(|&x| x != (i,j));
								goblins.push(new_index);
							}
						}
						
						let mut attack = (i, j);
						let mut lowest_hp = -1;
						// attack if possible
						if new_index.1 > 0 && squares[new_index.1 - 1][new_index.0].0 == enemy_char
							&& (squares[new_index.1 - 1][new_index.0].2 < lowest_hp || lowest_hp < 0)
						{
							lowest_hp = squares[new_index.1 - 1][new_index.0].2;
							attack = (new_index.0, new_index.1 - 1);
						}
						if new_index.0 > 0 && squares[new_index.1][new_index.0 - 1].0 == enemy_char
							&& (squares[new_index.1][new_index.0 - 1].2 < lowest_hp || lowest_hp < 0)
						{
							lowest_hp = squares[new_index.1][new_index.0 - 1].2;
							attack = (new_index.0 - 1, new_index.1);
						}
						if new_index.0 < squares[new_index.1].len() - 1 && squares[new_index.1][new_index.0 + 1].0 == enemy_char
							&& (squares[new_index.1][new_index.0 + 1].2 < lowest_hp || lowest_hp < 0)
						{
							lowest_hp = squares[new_index.1][new_index.0 + 1].2;
							attack = (new_index.0 + 1, new_index.1);
						}
						if new_index.1 < squares.len() - 1 && squares[new_index.1 + 1][new_index.0].0 == enemy_char
							&& (squares[new_index.1 + 1][new_index.0].2 < lowest_hp || lowest_hp < 0)
						{
							lowest_hp = squares[new_index.1 + 1][new_index.0].2;
							attack = (new_index.0, new_index.1 + 1);
						}					
						
						if lowest_hp > 0
						{
							let mut damage = 3;
							if enemy_char == 'G'
							{	
								damage = attack_power;
							}
							squares[attack.1][attack.0].2 -= damage;
							if squares[attack.1][attack.0].2 <= 0
							{
								squares[attack.1][attack.0] = ('.', 1, 200);
								if enemy_char == 'G'
								{
									goblins.retain(|&x| x != attack);
								}
								else
								{
									attack_power += 1;
									break 'inner;
									//elves.retain(|&x| x != attack);
								}
							}
						}
					}
				}
			}
			tick += 1;		
		}
	}
	let mut result = 0;
	for h in 0..squares.len()
	{
		for g in 0..squares[h].len()
		{
			if squares[h][g].0 == 'G' || squares[h][g].0 == 'E'
			{
				result += squares[h][g].2;
			}
		}
	}
	println!("{:?}", result * tick);
}
	