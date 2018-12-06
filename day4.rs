use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() 
{
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents:String = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
		
	let mut lines:Vec<&str> = contents.split("\r\n").collect();
	
	lines.sort();
	
	let mut guard = 0;
	let mut start_minute = 0;
	let mut guards:HashMap<i32, Vec<i32>> = HashMap::new();
	
	for line in lines
	{
		let words:Vec<&str> = line.split(' ').collect();
		
		// new guard?
		if words[3].contains('#')
		{
			guard = words[3].trim_matches('#').parse().unwrap();
		}
		else if words[2] == "falls"
		{
			let times:Vec<&str> = words[1].split(':').collect();
			start_minute = times[1].trim_matches(']').parse().unwrap();
		}
		else // wakes up
		{
			let times:Vec<&str> = words[1].split(':').collect();
			let end_minute:i32 = times[1].trim_matches(']').parse().unwrap();
			for i in start_minute..end_minute
			{
				guards.entry(guard).or_insert(Vec::new()).push(i);
			}
		}
	
	}

	let mut guard_no = 0;
	let mut minutes_vec = Vec::new();
	
	for (key, minutes) in &guards
	{
		if minutes.len() > minutes_vec.len()
		{
			guard_no = *key;
			minutes_vec = minutes.to_vec();
		}
	}
	
	let mut current_minute = -1;
	let mut current_count = -1;
	let mut max_count = -1;
	let mut max_minute = -1;
	
	minutes_vec.sort();	

	for minute in minutes_vec
	{
		if minute == current_minute
		{
			current_count += 1;
			if current_count > max_count
			{
				max_count = current_count;
				max_minute = minute;
			}
		}
		else
		{
			current_minute = minute;
			current_count = 1;
		}
	}
	
	
	println!("{:?}", guard_no * max_minute);
			
}

