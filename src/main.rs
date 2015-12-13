extern crate rand;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use rand::distributions::{IndependentSample, Range};

fn main(){
	let args: Vec<String> = env::args().collect();
	let mut f1 = open_file("first_names.txt").unwrap();
	let mut f2 = open_file("last_names.txt").unwrap();
	let f_names = read_file_lines(&mut f1).unwrap();
	let l_names = read_file_lines(&mut f2).unwrap();
	

	let lim = if args.len() > 1 {
		args[1].parse::<i32>().unwrap()
		}else{
			1
		};

	for _ in 0..lim{
		let name = generate_random_name(&f_names, &l_names);
		println!("{}", name);
	}
}

fn generate_random_name(fnames: &Vec<String>, lnames: &Vec<String>) -> String{
	let mut between = Range::new(0, fnames.len());
	let mut rng = rand::thread_rng();
	let first = fnames[between.ind_sample(&mut rng)].clone();
	between = Range::new(0, lnames.len());
	let last = lnames[between.ind_sample(&mut rng)].clone();
	return first + " " + &last;
}

fn open_file(f: &str) -> Result<File, io::Error> {
	let file = try!(File::open(f));
	return Ok(file);
}

fn read_file_lines(f: &mut File) -> Result<Vec<String>, io::Error> {
	let read = BufReader::new(f);
	let mut lines = Vec::<String>::new();
	for line in read.lines(){
		lines.push(line.unwrap());
	}
	return Ok(lines);
}