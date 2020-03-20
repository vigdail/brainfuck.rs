pub const MEM_SIZE: usize = 30000;

#[derive(Debug)]
pub struct VM {
	pub program: Vec<BFCode>,
	pub ram: Vec<u8>,
	pub program_pointer: usize,
	pub mem_pointer: usize,
	pub stack: Vec<usize>,
}

#[derive(Copy, Clone, Debug)]
pub enum BFCode {
	Next,
	Prev,
	Inc,
	Dec,
	Print,
	Read,
	While,
	EndWhile,
}
