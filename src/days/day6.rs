use std::collections::HashMap;

use crate::util::{read_input_file_lines, Solution};

pub struct Day6 {
    day: u32,
}

impl Default for Day6 {
    fn default() -> Self {
        Self { day: 6 }
    }
}

impl Solution for Day6 {
    fn solve_first_part(&self) -> i64 {
        let input = read_input_file_lines(self.day);
        let mut t = 0;

        let mut instr_list: Vec<Instr> = Vec::new();

        for line in input {
            let splitted_parts = line.split(" ").collect::<Vec<&str>>();
            
            if splitted_parts.len() == 4 {
                // toggle
                let mode = InstrMode::Toggle;

                let (start_x, start_y) = splitted_parts[1].split_once(",").unwrap();
                let (end_x, end_y) = splitted_parts[3].split_once(",").unwrap();

                instr_list.push(Instr {start_x: start_x.parse::<u32>().unwrap(), start_y: start_y.parse::<u32>().unwrap(), end_x: end_x.parse::<u32>().unwrap(), end_y: end_y.parse::<u32>().unwrap(), instr_mode: mode});
            } else {
                // on or off
                let mode = match splitted_parts[1] {
                    "on" => InstrMode::On,
                    "off" => InstrMode::Off,
                    _ => panic!("Unexpected Instr-Mode")
                };

                let (start_x, start_y) = splitted_parts[2].split_once(",").unwrap();
                let (end_x, end_y) = splitted_parts[4].split_once(",").unwrap();
                
                instr_list.push(Instr {start_x: start_x.parse::<u32>().unwrap(), start_y: start_y.parse::<u32>().unwrap(), end_x: end_x.parse::<u32>().unwrap(), end_y: end_y.parse::<u32>().unwrap(), instr_mode: mode});
            }
        }

        for x in 0..1000 {
            for y in 0..1000 {
                let mut init_state: bool = false;

                for instr in instr_list.iter() {
                    if instr.start_x <= x && instr.end_x >= x && instr.start_y <= y && instr.end_y >= y{
                        init_state = match instr.instr_mode {
                            InstrMode::On => true,
                            InstrMode::Off => false,
                            InstrMode::Toggle => !init_state,
                        }
                    }
                }

                if init_state {
                    t += 1;
                }
            }
        }

        return t as i64;
    }

    fn solve_second_part(&self) -> i64 {
        let input = read_input_file_lines(self.day);
        let mut t = 0;

        let mut instr_list: Vec<Instr> = Vec::new();

        for line in input {
            let splitted_parts = line.split(" ").collect::<Vec<&str>>();
            
            if splitted_parts.len() == 4 {
                // toggle
                let mode = InstrMode::Toggle;

                let (start_x, start_y) = splitted_parts[1].split_once(",").unwrap();
                let (end_x, end_y) = splitted_parts[3].split_once(",").unwrap();

                instr_list.push(Instr {start_x: start_x.parse::<u32>().unwrap(), start_y: start_y.parse::<u32>().unwrap(), end_x: end_x.parse::<u32>().unwrap(), end_y: end_y.parse::<u32>().unwrap(), instr_mode: mode});
            } else {
                // on or off
                let mode = match splitted_parts[1] {
                    "on" => InstrMode::On,
                    "off" => InstrMode::Off,
                    _ => panic!("Unexpected Instr-Mode")
                };

                let (start_x, start_y) = splitted_parts[2].split_once(",").unwrap();
                let (end_x, end_y) = splitted_parts[4].split_once(",").unwrap();
                
                instr_list.push(Instr {start_x: start_x.parse::<u32>().unwrap(), start_y: start_y.parse::<u32>().unwrap(), end_x: end_x.parse::<u32>().unwrap(), end_y: end_y.parse::<u32>().unwrap(), instr_mode: mode});
            }
        }

        for x in 0..1000 {
            for y in 0..1000 {
                let mut brightness: i32 = 0;

                for instr in instr_list.iter() {
                    if instr.start_x <= x && instr.end_x >= x && instr.start_y <= y && instr.end_y >= y{
                        brightness += match instr.instr_mode {
                            InstrMode::On => 1,
                            InstrMode::Off => -1,
                            InstrMode::Toggle => 2,
                        };

                        if brightness < 0 {
                            brightness = 0;
                        }
                    }
                }

                t += brightness;
            }
        }

        return t as i64;
    }
}

enum InstrMode {
    On,
    Off,
    Toggle
}

struct Instr {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
    instr_mode: InstrMode
}