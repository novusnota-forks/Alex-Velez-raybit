use std::{
    collections::HashMap,
    io::{Read, Write},
};

type RbfType = i32;

mod raylib_map;

// floats: significant + (1 / (base ^ exponent))

fn main() {
    // open file and read to string
    let mut file = std::fs::File::open("main.rbf").unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    // setup program
    let program = clean_source(file_contents);
    let loop_table = generate_loop_table(&program);
    let mut memory: Vec<RbfType> = Vec::from([0]);
    let mut pointer: usize = 0;
    let mut index: usize = 0;

    // run program
    while index < program.len() {
        match program[index] {
            '>' => {
                // increment pointer
                pointer += 1;
                // extend memory when needed
                adjust_size(&mut memory, pointer);
            }
            '<' => {
                if pointer == 0 {
                    // loop to end of memeory
                    pointer = memory.len();
                } else {
                    // decrement pointer
                    pointer -= 1;
                }
            }
            '+' => {
                // increment memory at pointer
                memory[pointer] += 1;
            }
            '-' => {
                if memory[pointer] == RbfType::MIN {
                    // loop at min
                    memory[pointer] = RbfType::MAX;
                } else {
                    // decrement memory at pointer
                    memory[pointer] -= 1;
                }
            }
            '.' => {
                // print ascii
                putch(memory[pointer] as core::ffi::c_char);
                // flush after print
                let _ = std::io::stdout().flush();
            }
            '?' => {
                // print value at pointer cell
                println!("{}: [{}]", pointer, memory[pointer]);

                // flush after print
                let _ = std::io::stdout().flush();
            }

            ',' => {
                // get 1 character input
                memory[pointer] = getch() as RbfType;
            }

            '[' => {
                if memory[pointer] == 0 {
                    // jump to ]
                    index = *loop_table.get(&index).unwrap();
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    // jump to [
                    index = *loop_table.get(&index).unwrap();
                }
            }
            '@' => unsafe {
                // call raylib func with inputs
                raylib_map::call_raylib(&mut memory, pointer);
            },
            '#' => {
                // print entire memory
                for x in 0..memory.len() {
                    print!("[{:?}]", memory[x]);
                }
                println!()
            }
            '!' => {
                // stop program
                break;
            }
            '$' => {}
            '&' => {}
            '"' => {}
            '%' => {}
            '^' => {}
            '~' => {}
            '`' => {}
            '\'' => {}
            '|' => {}
            '\\' => {}
            '/' => {}
            ':' => {}
            ';' => {}
            '(' => {}
            ')' => {}
            '{' => {}
            '}' => {}
            '_' => {}
            _ => {}
        }
        index += 1;
    }
}

extern "C" {
    fn _getch() -> core::ffi::c_char;
    fn _putch(c_char: core::ffi::c_char) -> core::ffi::c_void;
}

fn getch() -> core::ffi::c_char {
    unsafe { _getch() }
}

fn putch(c_char: core::ffi::c_char) -> core::ffi::c_void {
    unsafe { _putch(c_char) }
}

pub fn adjust_size(memory: &mut Vec<RbfType>, pointer: usize) {
    while memory.len() <= pointer {
        memory.push(0);
    }
}

pub fn generate_loop_table(program: &Vec<char>) -> HashMap<usize, usize> {
    let mut loop_table: HashMap<usize, usize> = HashMap::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    for (index, instruction) in program.iter().enumerate() {
        if instruction == &'[' {
            loop_stack.push(index);
        } else if instruction == &']' {
            let loop_beggining_index = loop_stack.pop().unwrap();
            loop_table.insert(loop_beggining_index, index);
            loop_table.insert(index, loop_beggining_index);
        }
    }
    return loop_table;
}

pub fn clean_source(file_contents: String) -> Vec<char> {
    file_contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !(line.is_empty() || line.starts_with("//")))
        .map(|line| line.split_once("//").unwrap_or((line, "")).0.trim())
        .collect::<String>()
        .chars()
        .collect()
}
