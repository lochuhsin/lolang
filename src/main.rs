use clap::Parser;
use core::panic;
use lolang::chunk::Chunk;
use lolang::compiler::{compile, Compiler};
use lolang::vm::{InterpretResult, VirtualMachine};
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::process::exit;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_delimiter = ' ', num_args=1..)]
    path: Vec<PathBuf>,
}

fn trim_end(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();

        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn interpret(
    s: String,
    vm: &mut VirtualMachine,
    chunk: &mut Chunk,
    compiler: &mut Compiler,
) -> InterpretResult {
    if !compile(s, chunk, compiler) {
        return InterpretResult::InterpretCompileError;
    };
    vm.run(chunk)
}

fn run_prompt() {
    let mut vm = VirtualMachine::default();
    let mut chunk = Chunk::default();
    let mut compiler = Compiler::default();

    loop {
        print!(">> ");
        let _ = stdout().flush();
        let mut s = String::new();
        let r = std::io::stdin().read_line(&mut s);
        if r.is_err() {
            println!("Something went wrong while reading from prompt");
            break;
        }
        trim_end(&mut s);
        if s == *"exit" {
            break;
        }
        match interpret(s.clone(), &mut vm, &mut chunk, &mut compiler) {
            InterpretResult::InterpretOk => (),
            InterpretResult::InterpretCompileError => {
                println!("compile error, code: {}", 65);
                exit(65)
            }
            InterpretResult::InterpretRunTimeError => {
                println!("compile error, code: {}", 70);
                exit(70)
            }
        }
    }
}

fn run_file(path: &PathBuf) {
    let mut vm = VirtualMachine::default();
    let mut chunk = Chunk::default();
    let mut contents = String::new();
    let mut compiler = Compiler::default();
    if let Ok(mut file) = File::open(path) {
        let _ = file.read_to_string(&mut contents);
    } else {
        panic!("Couldn't open file or file doesn't not exist")
    }
    match interpret(contents, &mut vm, &mut chunk, &mut compiler) {
        InterpretResult::InterpretOk => (),
        InterpretResult::InterpretCompileError => exit(65),
        InterpretResult::InterpretRunTimeError => exit(70),
    }
}

fn main() {
    let args = Args::parse();

    if args.path.is_empty() {
        run_prompt();
    } else if args.path.len() == 1 {
        let path = &args.path[0];
        run_file(path);
    } else {
        panic!("Multiple file parsing not supported yet");
    }
}
