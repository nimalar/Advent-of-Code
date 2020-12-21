use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut allergenes:HashMap<&str, Vec<&str>> = HashMap::new();
	let mut ingredients:HashMap<&str, usize> = HashMap::new();
	let mut result_1 = 0;
	let mut result_2 = "".to_string();
	
	for row in contents.lines()
	{
		let parts:Vec<&str> = row.trim().trim_end_matches(")").split(" (contains ").collect();
		let genes:Vec<&str> = parts[1].split(", ").collect();
		let foods:Vec<&str> = parts[0].split(" ").collect();
		
		for food in 0..foods.len()
		{
			let incls = ingredients.entry(foods[food]).or_insert(0);
			*incls += 1;
		}
		
		for gene in genes
		{
			if !allergenes.contains_key(gene)
			{
				allergenes.insert(gene, foods.to_vec());
				continue;
			}
			
			let mut gene_list = allergenes.get_mut(&gene).unwrap();
			let mut i = 0;
			while i < gene_list.len()
			{
				if !foods.contains(&gene_list[i])
				{
					gene_list.remove(i);
					continue;
				}
				i += 1;
			}
		}
	}

	let mut allergene_keys:Vec<&str> = allergenes.keys().cloned().collect();	

	loop
	{
		let mut lengths = 0;
		for i in 0..allergene_keys.len()
		{
			let gene = allergenes.get(allergene_keys[i]).unwrap().to_vec();
			if gene.len() == 1
			{
				lengths += 1;
				for j in 0..allergene_keys.len()
				{
					if i != j
					{
						let vector = allergenes.entry(&allergene_keys[j]).or_insert(Vec::new());
						vector.retain(|&x| x != gene[0]);
					}
				}
				ingredients.remove(&gene[0]);
			}
		}
		if lengths == allergenes.len()
		{
			break;
		}
	}
	
	for value in ingredients.values()
	{
		result_1 += value;
	}
	
	allergene_keys.sort();
		
	for key in allergene_keys
	{
		result_2.push_str(allergenes.get(key).unwrap()[0]);
		result_2.push_str(",");
	}
	
	println!("{}", result_1);
	println!("{}", result_2.trim_end_matches(","));
}
