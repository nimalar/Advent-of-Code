use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_route(start:String, ref caves:&HashMap<&str, Vec<&str>>, ref mut route:&mut Vec<String>, more_caves:bool) -> usize
{
	if start == "end".to_string()
	{
		return 1;
	}
	let mut result = 0;
	let options = caves.get(&start as &str).unwrap();
	route.push(start);
	for i in options
	{
		let mut new_route = route.to_vec();
		if !(route.contains(&i.to_string()) && i.chars().all(|x| x.is_lowercase()))
		{
			result += get_route(i.to_string(), &caves, &mut new_route, more_caves);
		}
		else if more_caves && i != &"start"
		{
			result += get_route(i.to_string(), &caves, &mut new_route, false);
		}
	}
	return result;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut caves:HashMap<&str, Vec<&str>> = HashMap::new();
	
	for row in contents.lines()
	{
		let parts:Vec<&str> = row.split("-").collect();
		caves.entry(parts[0]).or_insert(Vec::new()).push(parts[1]);
		caves.entry(parts[1]).or_insert(Vec::new()).push(parts[0]);
	}
	
	let mut route:Vec<String> = Vec::new();
	println!("First star: {}", get_route("start".to_string(), &caves, &mut route, false));
	println!("Second star: {}", get_route("start".to_string(), &caves, &mut route, true));
}
