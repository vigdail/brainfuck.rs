use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod commands;
mod vm;

use vm::{BFCode, VM};

fn main() {
	let mut state = init_vm();
	loop {
		if state.program_pointer == state.program.len() {
			break;
		}
		state = execute(state);
	}
}

fn execute(state: VM) -> VM {
	let command = state.program[state.program_pointer];
	match command {
		BFCode::Next => commands::next(state),
		BFCode::Prev => commands::prev(state),
		BFCode::Inc => commands::inc(state),
		BFCode::Dec => commands::dec(state),
		BFCode::Print => commands::print(state),
		BFCode::Read => commands::read(state),
		BFCode::While => commands::start_while(state),
		BFCode::EndWhile => commands::end_while(state),
	}
}

fn init_vm() -> VM {
	VM {
		ram: vec![0; vm::MEM_SIZE],
		program_pointer: 0,
		mem_pointer: 0,
		program: read_program(),
	}
}

fn read_program() -> Vec<BFCode> {
	let filepath = env::args().skip(1).next().expect("No file path specified");
	let path = Path::new(filepath.as_str());
	let mut file = File::open(path).expect("Could not open the file");
	let mut content = String::new();
	file.read_to_string(&mut content)
		.expect("Could not read the file");

	content
		.into_bytes()
		.into_iter()
		.filter_map(|c| {
			let val = match c {
				b'>' => Some(BFCode::Next),
				b'<' => Some(BFCode::Prev),
				b'+' => Some(BFCode::Inc),
				b'-' => Some(BFCode::Dec),
				b'.' => Some(BFCode::Print),
				b',' => Some(BFCode::Read),
				b'[' => Some(BFCode::While),
				b']' => Some(BFCode::EndWhile),
				_ => None, //invalid command
			};

			val
		})
		.collect()
}
