use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn main() 
{
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents:String = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
		
	let mut steps:BTreeMap<&str, Vec<&str>> = BTreeMap::new();
			
	for line in contents.lines()
	{		
		let words:Vec<&str> = line.split(" ").collect();
		let step = words[7];
		let prerequisite = words[1];
	
		steps.entry(step).or_insert(Vec::new()).push(prerequisite);
		steps.entry(prerequisite).or_insert(Vec::new());
	}
	
	let mut answer:Vec<&str> = Vec::new();
	
	loop
	{
		let keys:Vec<_> = steps.keys().cloned().collect();
		let mut to_be_removed = "";
		for key in keys
		{
			if let Some(prerequisites) = steps.get_mut(key)
			{
				for i in 0..answer.len()
				{
					prerequisites.retain(|&x| x!= answer[i]);
				}
				if prerequisites.is_empty()
				{
					answer.push(key);
					to_be_removed = key;
					break;
				}
			}
		}
		steps.remove(to_be_removed);
		if steps.is_empty()
		{
			break;
		}
	}
	
	println!("{:?}", answer.into_iter().collect::<String>());
}

