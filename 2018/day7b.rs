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
	let mut working_on = vec![("", 0); 5];
	let mut time = 0;
	let mut under_construction:Vec<&str> = Vec::new();
	
	loop
	{
		for i in 0..working_on.len()
		{
			working_on[i].1 -= 1;
			if working_on[i].0 != "" && working_on[i].1 == 0
			{
				answer.push(working_on[i].0);
				steps.remove(working_on[i].0);
				working_on[i].0 = "";
			}
		}
		if steps.is_empty()
		{
			break;
		}
		time += 1;
		let keys:Vec<_> = steps.keys().cloned().collect();
		for key in keys
		{
			if let Some(prerequisites) = steps.get_mut(key)
			{
				for i in 0..answer.len()
				{
					prerequisites.retain(|&x| x!= answer[i]);
				}
				if prerequisites.is_empty() && !under_construction.contains(&key)
				{
					for i in 0..working_on.len()
					{
						if working_on[i].0 == ""
						{
							working_on[i].0 = key;
							working_on[i].1 = (key.chars().next().unwrap() as i32) - 64 + 60;
							under_construction.push(key);
							break;
						}
					}
				}
			}
		}
	}
	
	println!("{:?}", time);
}

