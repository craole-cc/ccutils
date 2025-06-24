use clap::Parser;

#[derive(Parser)]
struct Cli {
	number: String,
}

pub fn init() {
	let args = Cli::parse();
	let number = number::to_cardinal((args.number).parse().unwrap());

	println!("Received string: {}", args.number);
	println!("Number as cardinal: {}", number);
	println!("{}", 10_i32.to_cardinal());
}
