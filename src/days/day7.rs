use crate::util::{read_input_file_lines, Solution};

pub struct Day7 {
    day: u32,
}

impl Default for Day7 {
    fn default() -> Self {
        Self { day: 7 }
    }
}

impl Solution for Day7 {
    fn solve_first_part(&self) -> i64 {
        let input = read_input_file_lines(self.day);
        let mut t = 0;

        return t as i64;
    }

    fn solve_second_part(&self) -> i64 {
        let input = read_input_file_lines(self.day);
        let mut t = 0;

        return t as i64;
    }
}

enum ShiftDir {
    Left,
    Right
}

enum LogicOp {
    And,
    Or
}

struct DirectProvidedWire {
    provided_val: u16
}

struct ShiftedWire {
    dir: ShiftDir,
    shift_value: u16,
    shifted_wire: String
}

struct LogicWire {
    op: LogicOp,
    left_wire: String,
    right_wire: String
}

struct ComplementWire {
    depend_wire: String
}

impl WireResolver for DirectProvidedWire {
    fn resolveWire<T: WireResolver>(&self, wires: Vec<T>) -> u16 {
        todo!()
    }
}

impl WireResolver for ShiftedWire {
    fn resolveWire<T: WireResolver>(&self, wires: Vec<T>) -> u16 {
        
    }
}


trait WireResolver {
    fn resolveWire<T: WireResolver>(&self, wires: Vec<T>) -> u16;
}