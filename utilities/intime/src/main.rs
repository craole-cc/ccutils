mod cli;

use intime::Info;

fn main() {
	let info = Info::default();

	println!("Hello, world from intime!");
	println!("Raw {:#?}", info);
	println!("{}", info);
}
