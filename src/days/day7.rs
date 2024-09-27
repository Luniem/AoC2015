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
        let mut wires: Vec<Wires> = Vec::new();

        for line in input {
            let splitted_parts = line.split(" -> ").collect::<Vec<&str>>();

            assert!(splitted_parts.len() > 1, "Input-Line in wrong format");

            let id = splitted_parts[1];

            if splitted_parts[0].contains("NOT") {
                let depended_wire = splitted_parts[0].split("NOT ").collect::<Vec<&str>>()[0];

                wires.push(Wires::ComplementWire(ComplementWire {
                    depend_wire: depended_wire.to_string(),
                    id: id.to_string(),
                }));
            } else if splitted_parts[0].contains("LSHIFT") {
                let (shifted_wire, shift_value) = splitted_parts[0].split_once(" LSHIFT ").unwrap();

                wires.push(Wires::ShiftedWire(ShiftedWire {
                    id: id.to_string(),
                    dir: ShiftDir::Left,
                    shifted_wire: shifted_wire.to_string(),
                    shift_value: shift_value.parse::<u16>().unwrap(),
                }));
            } else if splitted_parts[0].contains("RSHIFT") {
                let (shifted_wire, shift_value) = splitted_parts[0].split_once(" RSHIFT ").unwrap();

                wires.push(Wires::ShiftedWire(ShiftedWire {
                    id: id.to_string(),
                    dir: ShiftDir::Right,
                    shifted_wire: shifted_wire.to_string(),
                    shift_value: shift_value.parse::<u16>().unwrap(),
                }));
            } else if splitted_parts[0].contains("AND") {
                let (left_wire, right_wire) = splitted_parts[0].split_once(" AND ").unwrap();

                wires.push(Wires::LogicWire(LogicWire {
                    id: id.to_string(),
                    left_wire: left_wire.to_string(),
                    right_wire: right_wire.to_string(),
                    op: LogicOp::And,
                }));
            } else if splitted_parts[0].contains("OR") {
                let (left_wire, right_wire) = splitted_parts[0].split_once(" OR ").unwrap();

                wires.push(Wires::LogicWire(LogicWire {
                    id: id.to_string(),
                    left_wire: left_wire.to_string(),
                    right_wire: right_wire.to_string(),
                    op: LogicOp::Or,
                }));
            } else {
                let mut wire = DirectProvidedWire {
                    id: id.to_string(),
                    provided_val: None,
                    provided_wire: None,
                };

                let parsed_number = splitted_parts[0].parse::<u16>();

                if parsed_number.is_ok() {
                    wire.provided_val = Some(parsed_number.unwrap());
                } else {
                    wire.provided_wire = Some(splitted_parts[0].to_string());
                }

                wires.push(Wires::DirectWire(wire));
            }
        }

        let v: Vec<&Wires> = wires.iter().map(|a| a).collect();

        for wire in wires.iter() {
            if wire.get_id() == "a" {
                return wire.resolve_wire(&v) as i64;
            }
        }

        panic!("No :(");
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
    provided_val: Option<u16>,
    provided_wire: Option<String>,
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
    fn resolve_wire(&self, wires: &Vec<&Wires>) -> u16 {
        if self.provided_val.is_some() {
            return self.provided_val.unwrap();
        }

        assert!(
            self.provided_wire.is_some(),
            "Didn't expect to find no direct value from!"
        );

        let search_string = self.provided_wire.as_ref().unwrap();
        let mut searched_wire: Option<&Wires> = None;

        for wire in wires {
            if wire.get_id() == search_string.as_str() {
                searched_wire = Some(wire);
            }
        }

        assert!(searched_wire.is_some(), "Failed to find wire!");

        return searched_wire.unwrap().resolve_wire(wires);
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl WireResolver for LogicWire {
    fn resolve_wire(&self, wires: &Vec<&Wires>) -> u16 {
        let mut left_wire: Option<&Wires> = None;
        let mut right_wire: Option<&Wires> = None;

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
    fn resolve_wire(&self, wires: &Vec<&Wires>) -> u16 {
        let mut shifted_wire: Option<&Wires> = None;

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
    fn resolve_wire(&self, wires: &Vec<&Wires>) -> u16 {
        let mut depend_wire: Option<&Wires> = None;

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

enum Wires {
    ComplementWire(ComplementWire),
    LogicWire(LogicWire),
    ShiftedWire(ShiftedWire),
    DirectWire(DirectProvidedWire),
}

impl WireResolver for Wires {
    fn resolve_wire(&self, wires: &Vec<&Wires>) -> u16 {
        match self {
            Wires::ComplementWire(wire) => wire.resolve_wire(wires),
            Wires::LogicWire(wire) => wire.resolve_wire(wires),
            Wires::ShiftedWire(wire) => wire.resolve_wire(wires),
            Wires::DirectWire(wire) => wire.resolve_wire(wires),
        }
    }

    fn get_id(&self) -> String {
        match self {
            Wires::ComplementWire(wire) => wire.get_id(),
            Wires::LogicWire(wire) => wire.get_id(),
            Wires::ShiftedWire(wire) => wire.get_id(),
            Wires::DirectWire(wire) => wire.get_id(),
        }
    }
}

trait WireResolver {
    fn resolve_wire(&self, wires: &Vec<&Wires>) -> u16;
    fn get_id(&self) -> String;
}
