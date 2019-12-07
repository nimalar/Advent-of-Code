use std::env;
use std::fs::File;
use std::io::prelude::*;

const VEC_SIZE: usize = 20;

#[derive(Default, Clone)]
struct Group<'a>
{
	team: &'a str,
	units: i32,
	hitpoints: i32,
	weaknesses: Vec<&'a str>,
	immunities: Vec<&'a str>,
	damage: i32,
	damage_type: &'a str,
	initiative: i32,
	target: i32,
}

fn get_damage(attack:&Group, target:&Group) -> i32
{
	let mut damage = attack.units * attack.damage;
	if target.immunities.contains(&attack.damage_type) || target.units == 0
	{
		damage = 0;
	}
	else if target.weaknesses.contains(&attack.damage_type)
	{
		damage *= 2;
	}	
	return damage;
}

fn part(boosted:bool) 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut team = "";
	let mut original_warriors:Vec<Group> = vec![Group::default(); VEC_SIZE];
	let mut warriors:Vec<Group>;
	
	// init warriors
	for line in contents.lines()
	{
		let mut weaknesses:Vec<&str> = Vec::new();
		let mut immunities:Vec<&str> = Vec::new();
		// team name
		if line.contains(":")
		{
			team = line.trim().trim_matches(':');
		}
		else if !line.trim().is_empty()
		{
			let mut index = line.find("units").unwrap();
			let units:i32 = line[..index].trim().parse().unwrap();
			
			let mut end_index = line.find(" hit points").unwrap();
			index = line[..end_index].rfind(' ').unwrap();
			let hitpoints:i32 = line[index..end_index].trim().parse().unwrap();
			
			index = line.find("weak to ").unwrap_or_default();
			if index > 0
			{
				end_index = line[index..].find(|c: char| (c == ';') || (c == ')')).unwrap();
				weaknesses = line[index+"weak to ".len()..index+end_index].split(", ").collect::<Vec<&str>>();
			}
			index = line.find("immune to ").unwrap_or_default();
			if index > 0
			{
				end_index = line[index..].find(|c: char| (c == ';') || (c == ')')).unwrap();
				immunities = line[index+"immune to ".len()..index+end_index].split(", ").collect::<Vec<&str>>();
			}
			index = line.find("that does ").unwrap() + "that does ".len();
			end_index = line[index..].find(' ').unwrap() + index;
			let damage:i32 = line[index..end_index].trim().parse().unwrap();
			
			index = line[end_index+1..].find(' ').unwrap() + end_index + 1;
			let damage_type = line[end_index+1..index].trim();
			
			index = line.find("initiative ").unwrap() + "initiative ".len();
			let initiative:i32 = line[index..].trim().parse().unwrap();
			let target = -1;
						
			original_warriors[VEC_SIZE - initiative as usize] = Group{team, units, hitpoints, weaknesses, immunities, damage, damage_type, initiative, target};
		}
	}
	
	let mut boost = 0;
	
	loop
	{
		let mut immune_system = 0;
		let mut infection = 0;
		warriors = original_warriors.to_vec();
		for i in 0..warriors.len()
		{
			if warriors[i].team == "Immune System"
			{
				immune_system += 1;
				warriors[i].damage += boost;
			}
			else
			{
				infection += 1;
			}
		}
		while immune_system > 0 && infection > 0
		{
			// target selection
			warriors.sort_by(|a, b| (b.units * b.damage).cmp(&(a.units * a.damage)));
			let mut targets:Vec<i32> = Vec::new();
			for i in 0..warriors.len()
			{
				let mut max_damage = 0;
				let mut best_enemy_index:i32 = -1;
				if warriors[i].units > 0
				{
					for j in 0..warriors.len()
					{
						if warriors[i].team != warriors[j].team && !targets.contains(&(VEC_SIZE as i32 - warriors[j].initiative))
						{
							let damage = get_damage(&warriors[i], &warriors[j]); 
							if best_enemy_index < 0 || damage > max_damage || (damage == max_damage && 
								warriors[j].units * warriors[j].damage > warriors[best_enemy_index as usize].units * warriors[best_enemy_index as usize].damage) || (damage == max_damage && 
								warriors[j].units * warriors[j].damage == warriors[best_enemy_index as usize].units * warriors[best_enemy_index as usize].damage && warriors[j].initiative > warriors[best_enemy_index as usize].initiative)
							{
								max_damage = damage;
								best_enemy_index = j as i32;
							}
						}
					}
				}
				if max_damage == 0
				{
					warriors[i].target = -1;
				}
				else
				{
					warriors[i].target = VEC_SIZE as i32 - warriors[best_enemy_index as usize].initiative;
					targets.push(VEC_SIZE as i32 - warriors[best_enemy_index as usize].initiative);
				}
			}
			
			// attack phase
			warriors.sort_by(|a, b| (b.initiative).cmp(&(a.initiative)));
			let mut unable_to_attack = 0;
			for i in 0..warriors.len()
			{
				let target = warriors[i].target;
				if target >= 0 && warriors[i].units > 0
				{
					let damage = get_damage(&warriors[i], &warriors[target as usize]);
					let units_lost = damage / warriors[target as usize].hitpoints;
					if units_lost == 0
					{
						unable_to_attack += 1;
					}
					else
					{
						warriors[target as usize].units -= units_lost;
						if warriors[target as usize].units <= 0
						{
							warriors[target as usize].units = 0;
							if warriors[target as usize].team == "Immune System"
							{
								immune_system -= 1;
							}
							else
							{
								infection -= 1;
							}
						}
					}
				}
				else
				{
					unable_to_attack += 1;
				}
			}
			if unable_to_attack == warriors.len()
			{
				break;
			}
		}
	
		if !boosted || infection == 0
		{
			break;
		}
		else
		{
			boost += 1;
		}
	}
	
	let mut result = 0;

	for i in 0..VEC_SIZE
	{
		result += warriors[i].units;
	}
	println!("{:?}", result);
}

fn main()
{
	part(false);
	part(true);
}
