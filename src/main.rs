#![allow(unused_variables)]
use std::{
    collections::HashMap,
    io::{Read, Write},
};

type BaseType = u8;

pub const WORD_SIZE: usize = 2;

mod flip_down;
mod flip_left;
mod flip_right;
mod flip_up;

fn main() {
    // get file from command line
    let file = read_file_arg();

    // open file and read to string
    let mut file = std::fs::File::open(file).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    // setup program
    let program = clean_source(file_contents);
    let loop_table = generate_loop_table(&program, '[', ']');
    let comment_table = generate_loop_table(&program, '{', '}');
    let mut memory: Vec<BaseType> = Vec::from([0]);
    let mut pointer: usize = 0;
    let mut index: usize = 0;

    // run program
    while index < program.len() {
        match program[index] {
            '>' => unsafe {
                // increment pointer
                pointer += 1;

                // extend memory when needed
                adjust_size(&mut memory, pointer);

                // check flip
                if index > 0 && program[index - 1] == '<' {
                    // flip left [<>]
                    flip_left::call(&mut memory, pointer);
                }
            },
            '<' => unsafe {
                if pointer == 0 {
                    // loop to end of memeory
                    pointer = memory.len();
                } else {
                    // decrement pointer
                    pointer -= 1;
                }

                // check flip
                if index > 0 && program[index - 1] == '>' {
                    // flip right [><]
                    flip_right::call(&mut memory, pointer);
                }
            },
            '+' => unsafe {
                // increment cell at pointer
                increment(&mut memory[pointer]);

                // check flip
                if index > 0 && program[index - 1] == '-' {
                    // flip down [-+]
                    flip_down::call(&mut memory, pointer);
                }
            },
            '-' => unsafe {
                // decrement cell at pointer
                decrement(&mut memory[pointer]);

                // check flip
                if index > 0 && program[index - 1] == '+' {
                    // flip up [+-]
                    flip_up::call(&mut memory, pointer);
                }
            },
            '.' => {
                // print ascii
                putch(memory[pointer] as core::ffi::c_char);
                // flush after print
                let _ = std::io::stdout().flush();
            }

            ',' => {
                // get 1 character input
                memory[pointer] = getch() as BaseType;
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
            '?' => {
                // print value at pointer cell
                println!("{}: [{}]", pointer, memory[pointer]);
            }
            '#' => {
                // print entire memory (horizontal)
                for x in 0..memory.len() {
                    print!("[{:?}]", memory[x]);
                }

                // flush after print
                let _ = std::io::stdout().flush();

                println!();
            }
            '$' => {
                // print entire memory (vertical)
                for x in 0..memory.len() {
                    println!("{:?}: [{:?}]", x, memory[x]);
                }
            }
            '!' => {
                // stop program
                break;
            }
            '{' => {
                // jump to }
                index = *comment_table.get(&index).unwrap();
            }
            '}' | ' ' | '|' => {}
            // '@' => {}
            // '&' => {}
            // '"' => {}
            // '%' => {}
            // '^' => {}
            // '~' => {}
            // '`' => {}
            // '\'' => {}
            // '\\' => {}
            // '/' => {}
            // ':' => {}
            // ';' => {}
            // '(' => {}
            // ')' => {}
            // '_' => {}
            _ => {}
        }
        index += 1;
    }
}

extern "C" {
    fn _getch() -> core::ffi::c_char;
    fn _putch(c_char: core::ffi::c_char) -> core::ffi::c_void;
}

// Read 1 byte input
fn getch() -> core::ffi::c_char {
    unsafe { _getch() }
}

// Write 1 byte ouput
fn putch(c_char: core::ffi::c_char) -> core::ffi::c_void {
    unsafe { _putch(c_char) }
}

// Expand memory
pub fn adjust_size(memory: &mut Vec<BaseType>, pointer: usize) {
    while memory.len() <= pointer {
        memory.push(0);
    }
}

// Match table generator
pub fn generate_loop_table(program: &Vec<char>, start: char, end: char) -> HashMap<usize, usize> {
    let mut loop_table: HashMap<usize, usize> = HashMap::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    for (index, instruction) in program.iter().enumerate() {
        if instruction == &start {
            loop_stack.push(index);
        } else if instruction == &end {
            let loop_beggining_index = loop_stack.pop().unwrap();
            loop_table.insert(loop_beggining_index, index);
            loop_table.insert(index, loop_beggining_index);
        }
    }
    return loop_table;
}

/// Clean source code from comments and extra whitespace. Keep single spaces.
pub fn clean_source(file_contents: String) -> Vec<char> {
    let mut cleansed = file_contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !(line.is_empty() || line.starts_with("//")))
        .map(|line| line.split_once("//").unwrap_or((line, "")).0.trim())
        .collect::<String>();
    cleansed.retain(|c| "<>+-,.[]?#$!{}| ".contains(c));
    cleansed.chars().collect()
}

/// Increment value with looping
pub fn increment(val: &mut BaseType) {
    if *val == BaseType::MAX {
        // loop at max
        *val = BaseType::MIN;
    } else {
        *val += 1;
    }
}

/// Decrement value with looping
pub fn decrement(val: &mut BaseType) {
    if *val == BaseType::MIN {
        // loop at min
        *val = BaseType::MAX;
    } else {
        *val -= 1;
    }
}

/// Read x amount of cells from memory
pub fn get_input_cells(memory: &mut Vec<BaseType>, pointer: usize, cells: usize) -> Vec<BaseType> {
    memory[(pointer + 1)..(pointer + cells + 1)]
        .iter()
        .cloned()
        .collect()
}

/// Read String at pointer position x in memory
pub fn get_string(memory: &mut Vec<BaseType>, str_ptr: usize) -> std::ffi::CString {
    // [A-z]...[0]
    // read string array starting at str_ptr to null
    let mut str_bytes: Vec<BaseType> = Vec::new();
    let mut x = 0;
    loop {
        if memory[str_ptr + x] == 0 {
            break;
        }
        str_bytes.push(memory[str_ptr + x]);
        x += 1;
    }

    // turn string vec (i32) into string bytes (u8)
    // let ascii_bytes: Vec<u8> = str_bytes.iter().map(|&x| x as u8).collect();

    // turn string bytes into String
    let ascii_str: String = String::from_utf8_lossy(&str_bytes).to_string();

    // turn string into C string
    let c_str = std::ffi::CString::new(ascii_str).unwrap();

    // return C string
    return c_str;
}

/// Read Color at pointer position x in memory
pub fn get_color(memory: &mut Vec<BaseType>, color_ptr: usize) -> raylib::ffi::Color {
    // [r][g][b][a]
    raylib::ffi::Color {
        r: memory[color_ptr],
        g: memory[color_ptr + 1],
        b: memory[color_ptr + 2],
        a: memory[color_ptr + 3],
    }
}

/// Read Image at pointer position x in memory
pub fn get_image(memory: &mut Vec<BaseType>, image_ptr: usize) -> raylib::ffi::Image {
    // [width][width][height][height][mipmaps][mipmaps][format][format][data]...[r][g][b][a]
    let width = crate::cells_to_unsigned_u16(&memory[image_ptr..(image_ptr + 2)]);
    let height = crate::cells_to_unsigned_u16(&memory[(image_ptr + 2)..(image_ptr + 4)]);
    let mipmaps = crate::cells_to_unsigned_u16(&memory[(image_ptr + 4)..(image_ptr + 6)]);
    let format = crate::cells_to_unsigned_u16(&memory[(image_ptr + 6)..(image_ptr + 8)]);
    let mut data: Vec<u8> = Vec::new();
    for x in 0..(width * height * 4) as usize {
        data.push(memory[image_ptr + 8 + x]);
    }
    raylib::ffi::Image {
        data: data.as_mut_ptr() as *mut std::ffi::c_void,
        width: width as i32,
        height: height as i32,
        mipmaps: mipmaps as i32,
        format: format as i32,
    }
}

/// Convert array of cells to unisgned integer
pub fn cells_to_unsigned(cells: &[BaseType]) -> usize {
    let mut num = 0;
    for (x, val) in cells.iter().enumerate() {
        num += *val as usize * (BaseType::MAX as usize + 1).pow(x as u32);
    }
    num
}

pub const fn cells_to_unsigned_u16(cells: &[BaseType]) -> u16 {
    cells[0] as u16 + (cells[1] as u16 * 256)
}

/// Read file from command line arguments
pub fn read_file_arg() -> String {
    // get command line args
    let args: Vec<String> = std::env::args().collect();
    // validate command line
    match args.len() {
        x if x < 2 => panic!("No file defined for interpretation!"),
        x if x > 2 => panic!("Too many arguments!"),
        _ => {}
    }
    args[1].clone()
}
