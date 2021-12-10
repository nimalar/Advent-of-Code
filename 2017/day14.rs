use std::u8;

fn infect(mut rows:& mut[Vec<i32>], i:usize, j:usize, index:i32) -> bool
{
	rows[i][j] = index;
	if i > 0 && rows[i-1][j] < 0
	{
		infect(&mut rows, i-1, j, index);
	}
	if i < rows.len()-1 && rows[i+1][j] < 0
	{
		infect(&mut rows, i+1, j, index);
	}
	if j > 0 && rows[i][j-1] < 0
	{
		infect(&mut rows, i, j-1, index);
	}
	if j < rows[i].len()-1 && rows[i][j+1] < 0
	{
		infect(&mut rows, i, j+1, index);
	}
	return true;
}

fn main() {
	let mut rows:Vec<Vec<i32>> = Vec::new();
	let mut summa = 0;
	for x in 0..128
	{
		let mut contents = "jzgqcdpd-".to_string();
		contents.push_str(&x.to_string());
		
    	// the seq of lengths
    	let mut lengths:Vec<u8> = Vec::new();
    	for char in contents.chars()
    	{
    		lengths.push(char as u8);
    	}
    	lengths.push(17);
    	lengths.push(31);
    	lengths.push(73);
    	lengths.push(47);
    	lengths.push(23);
    	
    	// list of positions
    	let mut list:Vec<i32> = Vec::new();
    	for i in 0..256
    	{
    		list.push(i);
    	}	
    	
    	let mut position:usize = 0;
    	let mut skip_size = 0;
    	
    	for _x in 0..64
    	{
    		for y in 0..lengths.len()
    		{
    			let length_int:i32 = lengths[y] as i32;
    			let mut new_list:Vec<i32> = Vec::new();
    			
    			// reverse
    			for i in (position..position+length_int as usize).rev()
    			{
    				let mut index:usize = i as usize;
    				while index >= list.len()
    				{
    					index -= list.len();
    				}
    				new_list.push(list[index]);
    			}
    			// dump back
    			let mut j = 0;
    			for i in position..position+length_int as usize
    			{
    				let mut index:usize = i as usize;
    				while index >= list.len()
    				{
    					index -= list.len();
    				}
    				list[index] = new_list[j];
    				j += 1;
    			}
    			
    			position += length_int as usize + skip_size;
    			while position >= list.len()
    			{
    				position -= list.len();
    			}
    			skip_size += 1;
    		}
    	}
    	// xor hash
    	let mut xor:Vec<i32> = Vec::new();
    	for x in 0..16
    	{
    		let mut xorred = 0;
    		for y in 0..16
    		{
    			xorred ^= list[x*16+y as usize];
    		}
    		xor.push(xorred);
    	}
    	let mut hash = String::new();
    	for value in xor
    	{
    		hash.push_str(&format!("{:02x}", value));
    	}
    	
    	let mut row:Vec<i32> = Vec::new();
    	for char in hash.chars()
    	{
    	    let letter:u8 = u8::from_str_radix(&char.to_string(), 16).unwrap();
    	    let bit_char = format!("{:04b}", letter);
    	    for bit in bit_char.chars()
    	    {
    	        if bit == '1'
    	        {
    	            row.push(-1);
    	            summa += 1;
    	        }
    	        else
    	        {
    	            row.push(0);
    	        }
    	    }
    	}
    	rows.push(row);
    	
    }
    let mut index = 0;
    for i in 0..rows.len()
	{
	    for j in 0..rows[i].len()
	    {
	        if rows[i][j] == -1
	        {
	            // new area, new index
	            index += 1;
	            // infect everything nearby
	            infect(&mut rows, i, j, index);
	            
	        }
	    }
	}
	
    println!("osa1 {}", summa);
    println!("osa2 {}", index);
}
