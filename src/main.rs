use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
struct Record {
	patient: String,
	minute: Option<String>,
	calories: Option<f32>,
	distance: Option<f32>,
	floors: Option<u32>,
	heart: Option<u32>,
	steps: Option<u32>,
	sleep_level: Option<u8>,
}

fn example() -> Result<(), Box<dyn Error>> {
	let mut reader = csv::Reader::from_reader(io::stdin());

	for result in reader.deserialize() {
		let record: Record = result?;
		println!("{:?}", record);
	}

	Ok(())
}

fn main() {
	if let Err(err) = example() {
		println!("error running example: {}", err);
		process::exit(1);
	}
}
