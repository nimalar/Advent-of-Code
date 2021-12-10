
fn main() {
	let mut gen_a:u64 = 883;
	let mut gen_b:u64 = 879;
	let mut judge = 0;
	let mut judge_a:Vec<u64> = Vec::new();
	let mut judge_b:Vec<u64> = Vec::new();
	let mut judge_part2 = 0;
	let mut judged = 0;
	
	while judged < 5000000
	//for x in 0..40000000
	{
		gen_a = (gen_a * 16807) % 2147483647;
		gen_b = (gen_b * 48271) % 2147483647;
		
		// ditch uninportant part
		let match_a = gen_a & 0xffff;
		let match_b = gen_b & 0xffff;
		if match_a == match_b
		{
		    judge += 1;
		}
		
		if gen_a % 4 == 0
		{
		    judge_a.push(match_a);
		}
		if gen_b % 8 == 0
		{
		    judge_b.push(match_b);
		}
		
		if judge_a.len() > judged && judge_b.len() > judged
		{
		    if judge_a[judged] == judge_b[judged]
		    {
		        judge_part2 += 1;
		    }
		    judged += 1;
		}
	}
	println!("{}", judge);
	println!("{}", judge_part2);
}
