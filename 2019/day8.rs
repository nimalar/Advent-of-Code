use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut zero_digits = 0;
	let mut one_digits = 0;
	let mut two_digits = 0;
	let layer_size = 25 * 6;
	let mut pixels:Vec<Vec<char>> = vec![vec!['2'; 25]; 6];
	
	for i in 0..contents.chars().count() / layer_size
	{
		let mut layer:Vec<char> = contents[i*layer_size..(i+1)*layer_size].chars().collect();
		let mut zeros = layer.to_vec();
		zeros.retain(|&x| x == '0');
		if zero_digits == 0 || zeros.len() < zero_digits
		{
			zero_digits = zeros.len();
			let mut ones = layer.to_vec();
			let mut twos = layer.to_vec();
			ones.retain(|&x| x == '1');
			twos.retain(|&x| x == '2');
			one_digits = ones.len();
			two_digits = twos.len();
		}
		for j in 0..layer.len()
		{
			let x = j % 25;
			let y = j / 25;
			if pixels[y][x] == '2'
			{
				if layer[j] == '0' // black
				{
					pixels[y][x] = ' ';
				}
				else if layer[j] == '1' // white
				{
					pixels[y][x] = '#';
				}
				else // transparent
				{
					pixels[y][x] = layer[j];
				}
			}
		}
	}
	
	let result = one_digits * two_digits;
	
	println!("First star: {}", result);
	println!("Second star:");
	for k in 0..6
	{
		println!("{:?}", pixels[k].to_vec().into_iter().collect::<String>());
	}
}
