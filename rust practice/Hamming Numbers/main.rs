fn recursive_hammer(n: usize, i: u64, j: u64, k: u64, mut prod: u64, count: usize) -> u64
{
	let a : u64 = 2;
	let b : u64 = 3;
	let c : u64 = 5;
	if count <= n
	{
		let ap : u64 = prod * a.pow(i as u32 + 1);
		let bp : u64 = prod * b.pow(j as u32 + 1);
		let cp : u64 = prod * c.pow(k as u32 + 1);
		println!("{} {} {}", i, j, k);
		if ap < bp && ap < cp
		{
			prod = recursive_hammer(n, i + 1, j, k, ap, count + 1);
		}
		else if bp < ap && bp < cp
		{
			prod = recursive_hammer(n, i, j + 1, k, bp, count + 1);
		}
		else
		{
			prod = recursive_hammer(n, i, j, k + 1, cp, count + 1);
		}
		println!("{} {}", prod, count);
	}
	prod
}

fn hamming(n: usize) -> u64 {
    recursive_hammer(n, 0, 0, 0, 1, 1)
}

fn main () {
	let ham = hamming(10);
	println!("{}", ham);
}