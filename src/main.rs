use rand::prelude::*;
use rand::distributions;

fn generate_list<T>(length: usize) -> Vec<T>
		where distributions::Standard: distributions::Distribution<T> {
	let mut list = Vec::<T>::with_capacity(length);
	let mut rng = rand::thread_rng();

	for _i in 0..length {list.push(rng.gen());}

	list
}

fn main() {
	println!("Hello, world!");
}
