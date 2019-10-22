fn main() {
    println!("Hello, world!");
	
	let x = 5;
	println!("x = {}", x);

	let mut y = x;
	println!("y = {}", y);

	y = 10;
	println!("y = {}", y);

	y = y + 10;
	println!("y = {}", y);

	let z = x * y;
	println!("z = {}", z);
}
