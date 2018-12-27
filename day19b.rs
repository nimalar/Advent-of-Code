
fn main() 
{
	let mut registers:Vec<usize> = vec![1, 0, 0, 0, 0, 0];
	
	registers[2] += 16;	// 16
	registers[2] += 1;	// 17
	
	registers[5] += 2;	// 2
	registers[2] += 1;	// 18
	
	registers[5] = registers[5] * registers[5];	// 4
	registers[2] += 1;	// 19
	
	registers[5] = registers[5] * registers[2];	// 76
	registers[2] += 1;	// 20
	
	registers[5] = registers[5] * 11;	// 836
	registers[2] += 1;	// 21
	
	registers[4] += 5;	// 5
	registers[2] += 1;	// 22
	
	registers[4] = registers[4] * registers[2];	// 110
	registers[2] += 1;	// 23
	
	// addi 4 9 4
	registers[4] += 9;	// 119
	registers[2] += 1;	// 24
	
	// addr 5 4 5
	registers[5] = registers[5] + registers[4];	// 955
	registers[2] += 1;	// 25
	
	// addr 2 0 2
	registers[2] = registers[2] + registers[0];	// 26
	registers[2] += 1;	// 27
	
	// seti 0 0 2
	// skip
	
	// setr 2 3 4
	registers[4] = registers[2]; // 27
	registers[2] += 1;	// 28
	
	// mulr 4 2 4
	registers[4] = registers[4] * registers[2];	// 756
	registers[2] += 1;	// 29
	
	// addr 2 4 4
	registers[4] = registers[4] + registers[2];	// 785
	registers[2] += 1;	// 30
	
	// mulr 2 4 4
	registers[4] = registers[2] * registers[4];	// 23550
	registers[2] += 1;	// 31
	
	// muli 4 14 4
	registers[4] = registers[4] * 14;	// 329700
	registers[2] += 1;	// 32
	
	// mulr 4 2 4
	registers[4] = registers[4] * registers[2];	// 10550400
	registers[2] += 1;	// 33
	
	// addr 5 4 5
	registers[5] = registers[5] + registers[4];	// 10551355
	registers[2] += 1;	// 34
	
	// seti 0 6 0
	registers[0] = 0;
	registers[2] += 1;	// 35
	
	// seti 0 3 2
	registers[2] = 0;
	registers[2] += 1;	// 1
	
	// => riville 1
	// 1: seti 1 0 1
	registers[1] = 1;
	registers[2] += 1;	// 2
	
	// 2: seti 1 4 3
	registers[3] = 1;
	registers[2] += 1;	// 3
	
	let mut answer = 0;
	
	// loop
	for i in 1..10551356
	{
		if 10551355 % i == 0
		{
			answer += i;
		}
	}
	println!("{}", answer);
}
