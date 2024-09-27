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
    Right,
}

enum LogicOp {
    And,
    Or,
}

struct DirectProvidedWire {
    provided_val: u16,
    id: String,
}

struct ShiftedWire {
    dir: ShiftDir,
    shift_value: u16,
    shifted_wire: String,
    id: String,
}

struct LogicWire {
    op: LogicOp,
    left_wire: String,
    right_wire: String,
    id: String,
}

struct ComplementWire {
    depend_wire: String,
    id: String,
}

impl WireResolver for DirectProvidedWire {
    fn resolve_wire<T: WireResolver>(&self, wires: &Vec<&T>) -> u16 {
        return self.provided_val;
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl WireResolver for LogicWire {
    fn resolve_wire<T: WireResolver>(&self, wires: &Vec<&T>) -> u16 {
        let mut left_wire: Option<&T> = None;
        let mut right_wire: Option<&T> = None;

        for wire in wires {
            if wire.get_id() == self.left_wire {
                left_wire = Some(wire);
            } else if wire.get_id() == self.right_wire {
                right_wire = Some(wire);
            }
        }

        if left_wire.is_none() || right_wire.is_none() {
            panic!("Expected to found wire!");
        }

        let left_wire = left_wire.unwrap();
        let right_wire = right_wire.unwrap();

        return match self.op {
            LogicOp::And => left_wire.resolve_wire(wires) & right_wire.resolve_wire(wires),
            LogicOp::Or => left_wire.resolve_wire(wires) | right_wire.resolve_wire(wires),
        };
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl WireResolver for ShiftedWire {
    fn resolve_wire<T: WireResolver>(&self, wires: &Vec<&T>) -> u16 {
        let mut shifted_wire: Option<&T> = None;

        for wire in wires.iter() {
            if wire.get_id() == self.shifted_wire {
                shifted_wire = Some(wire);
            }
        }

        if shifted_wire.is_none() {
            panic!("Expected to found wire");
        }

        return match self.dir {
            ShiftDir::Left => shifted_wire.unwrap().resolve_wire(wires) << self.shift_value,
            ShiftDir::Right => shifted_wire.unwrap().resolve_wire(wires) >> self.shift_value,
        };
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl WireResolver for ComplementWire {
    fn resolve_wire<T: WireResolver>(&self, wires: &Vec<&T>) -> u16 {
        let mut depend_wire: Option<&T> = None;

        for wire in wires.iter() {
            if wire.get_id() == self.depend_wire {
                depend_wire = Some(wire);
            }
        }

        if depend_wire.is_none() {
            panic!("Expected to found wire");
        }

        return !depend_wire.unwrap().resolve_wire(wires);
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

trait WireResolver {
    fn resolve_wire<T: WireResolver>(&self, wires: &Vec<&T>) -> u16;
    fn get_id(&self) -> String;
}
