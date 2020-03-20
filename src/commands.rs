use crate::vm::{BFCode, VM};
use std::io::stdin;

pub fn next(state: VM) -> VM {
	let vm = VM {
		mem_pointer: state.mem_pointer + 1,
		program_pointer: state.program_pointer + 1,
		..state
	};

	vm
}

pub fn prev(state: VM) -> VM {
	VM {
		mem_pointer: state.mem_pointer - 1,
		program_pointer: state.program_pointer + 1,
		..state
	}
}

pub fn inc(state: VM) -> VM {
	let mut ram = state.ram;
	ram[state.mem_pointer] += 1;
	VM {
		ram,
		program_pointer: state.program_pointer + 1,
		..state
	}
}

pub fn dec(state: VM) -> VM {
	let mut ram = state.ram;
	ram[state.mem_pointer] -= 1;
	VM {
		ram,
		program_pointer: state.program_pointer + 1,
		..state
	}
}

pub fn print(state: VM) -> VM {
	print!("{}", state.ram[state.mem_pointer] as char);
	VM {
		program_pointer: state.program_pointer + 1,
		..state
	}
}

pub fn read(state: VM) -> VM {
	let mut s = String::new();
	stdin()
		.read_line(&mut s)
		.expect("failed to read from stdin");

	let num = s.trim_end().chars().nth(0).expect("failed to get input") as u8;

	let mut ram = state.ram;
	ram[state.mem_pointer] = num;
	VM {
		ram,
		program_pointer: state.program_pointer + 1,
		..state
	}
}

pub fn end_while(state: VM) -> VM {
	let mut stack = state.stack.clone();
	if let Some(new_program_pointer) = stack.pop() {
		return match state.ram[state.mem_pointer] {
			0 => VM {
				program_pointer: state.program_pointer + 1,
				stack,
				..state
			},
			_ => VM {
				program_pointer: new_program_pointer,
				stack,
				..state
			},
		};
	} else {
		unreachable!();
	};
}

pub fn start_while(state: VM) -> VM {
	match state.ram[state.mem_pointer] {
		0 => {
			let mut level = 0;
			let mut new_position = state.program_pointer + 1;
			for i in state.program_pointer + 1..state.program.len() - 1 {
				match state.program[i] {
					BFCode::While => level += 1,
					BFCode::EndWhile => {
						if level == 0 {
							new_position = i + 1;
							break;
						} else {
							level -= 1;
						}
					}
					_ => continue,
				}
			}
			VM {
				program_pointer: new_position,
				..state
			}
		}
		_ => {
			let mut stack = state.stack;
			stack.push(state.program_pointer);
			let res = VM {
				program_pointer: state.program_pointer + 1,
				stack,
				..state
			};
			res
		}
	}
}
