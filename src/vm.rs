use crate::chunk::Chunk;
use crate::constants;
use crate::errors::runtime_error;
use crate::table;
use crate::table::Table;
use crate::values::GenericValue;
use crate::values::GenericValueType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpCode {
    OpReturn = 1,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpLessEqual,
    OpGreaterEqual,
    OpPrint,
    OpPop,
    OpDefineGlobal,
    OpGetGlobal,
    OpSetGlobal,
    OpGetLocal,
    OpSetLocal,
    // TODO: implement bang equal, mod %
}

impl OpCode {
    pub fn from_usize(value: usize) -> OpCode {
        match value {
            1 => OpCode::OpReturn,
            2 => OpCode::OpConstant,
            3 => OpCode::OpNegate,
            4 => OpCode::OpAdd,
            5 => OpCode::OpSubtract,
            6 => OpCode::OpMultiply,
            7 => OpCode::OpDivide,
            8 => OpCode::OpNil,
            9 => OpCode::OpTrue,
            10 => OpCode::OpFalse,
            11 => OpCode::OpNot,
            12 => OpCode::OpEqual,
            13 => OpCode::OpGreater,
            14 => OpCode::OpLess,
            15 => OpCode::OpLessEqual,
            16 => OpCode::OpGreaterEqual,
            17 => OpCode::OpPrint,
            18 => OpCode::OpPop,
            19 => OpCode::OpDefineGlobal,
            20 => OpCode::OpGetGlobal,
            21 => OpCode::OpSetGlobal,
            22 => OpCode::OpGetLocal,
            23 => OpCode::OpSetLocal,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::OpReturn => "OpReturn",
            Self::OpConstant => "OpConst",
            Self::OpNegate => "OpNegate",
            Self::OpAdd => "OpAdd",
            Self::OpSubtract => "OpSubtract",
            Self::OpMultiply => "OpMultiply",
            Self::OpDivide => "OpDivide",
            Self::OpNil => "OpNil",
            Self::OpTrue => "OpTrue",
            Self::OpFalse => "OpFalse",
            Self::OpNot => "OpNot",
            Self::OpEqual => "OpEqualEqual",
            Self::OpGreater => "OpGreater",
            Self::OpLess => "OpLess",
            Self::OpGreaterEqual => "OpGreaterEqual",
            Self::OpLessEqual => "OpLessEqual",
            Self::OpPrint => "OpPrint",
            Self::OpPop => "OpPop",
            Self::OpDefineGlobal => "OpDefineGlobal",
            Self::OpGetGlobal => "OpGetGlobal",
            Self::OpSetGlobal => "OpSetGlobal",
            Self::OpGetLocal => "OpGetLocal",
            Self::OpSetLocal => "OpSetLocal",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    UnsupportedOperation(String, String),
    InvalidOperation(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::UnsupportedOperation(type1, type2) => {
                write!(f, "Operation not supported for {} and {}", type1, type2)
            }
            RuntimeError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRunTimeError,
}

#[derive(Default)]

pub struct VirtualMachine {
    pub ip: usize, // instruction pointer, the index currently pointing to the instruction in chunk
    pub vm_stack: VirtualMachineStack,
    pub table: Table,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            vm_stack: VirtualMachineStack::default(),
            table: Table::default(),
        }
    }
    pub fn run(&mut self, chunk: &mut Chunk) -> InterpretResult {
        loop {
            #[cfg(debug_assertions)]
            {
                for i in 0..self.vm_stack.ptr {
                    print!("[ {} ]", self.vm_stack.values[i])
                }
                println!();
                disassemble_instruction(chunk, self.ip);
            }
            let op_code = self.read_op(chunk);
            match op_code {
                OpCode::OpReturn => {
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpConstant => {
                    let val = self.read_constant(chunk);
                    self.vm_stack.push(val);
                }
                OpCode::OpNegate => {
                    self.vm_stack.negate_peek();
                }
                OpCode::OpAdd => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop(); // Handle empty value stack

                    let v = v1 + v2;

                    match v {
                        // TODO: put the actual line, not 0
                        Ok(v) => {
                            self.vm_stack.push(v);
                        }
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                    }
                }
                OpCode::OpSubtract => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop(); // Handle empty value stack
                    let v = v1 - v2;
                    match v {
                        Ok(v) => self.vm_stack.push(v),
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                    }
                }
                OpCode::OpMultiply => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop(); // Handle empty value stack
                    let v = v1 * v2;
                    match v {
                        // TODO: put the actual line, not 0
                        Ok(v) => self.vm_stack.push(v),
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                    }
                }
                OpCode::OpDivide => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop(); // Handle empty value stack
                    let v = v1 / v2;
                    match v {
                        // TODO: put the actual line, not 0
                        Ok(v) => self.vm_stack.push(v),
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                    }
                }
                OpCode::OpNil => self.vm_stack.push(GenericValue::from_none()),
                OpCode::OpFalse => self.vm_stack.push(GenericValue::from_bool(false)),
                OpCode::OpTrue => self.vm_stack.push(GenericValue::from_bool(true)),
                OpCode::OpNot => {
                    let val = self.vm_stack.pop();

                    // TODO: move this to value, operator overloading (trait ~~~)
                    fn is_false(v: &GenericValue) -> Result<bool, RuntimeError> {
                        match v {
                            GenericValueType::Nil => Ok(true),
                            GenericValueType::Bool(b) => Ok(!b),
                            _ => Err(RuntimeError::InvalidOperation("unary only support boolean and None, should the error be implemented in this phase ?".to_string())),
                        }
                    }
                    match is_false(&val) {
                        Ok(v) => self.vm_stack.push(GenericValue::from_bool(v)),
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                    }
                }
                OpCode::OpEqual => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop();
                    self.vm_stack.push(GenericValue::from_bool(v1 == v2))
                }
                OpCode::OpGreater => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop();

                    // TODO: move this to value, operator overloading (trait ~~~)
                    fn is_greater(
                        v1: GenericValue,
                        v2: GenericValue,
                    ) -> Result<bool, RuntimeError> {
                        match (v1, v2) {
                            (GenericValueType::Number(n1), GenericValueType::Number(n2)) => {
                                Ok(n1 > n2)
                            }
                            _ => Err(RuntimeError::InvalidOperation(
                                " > not supported ".to_string(),
                            )),
                        }
                    }
                    match is_greater(v1, v2) {
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                        Ok(v) => self.vm_stack.push(GenericValueType::from_bool(v)),
                    }
                }
                OpCode::OpLess => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop();

                    // TODO: move this to value, operator overloading (trait ~~~)
                    fn is_less(v1: GenericValue, v2: GenericValue) -> Result<bool, RuntimeError> {
                        match (v1, v2) {
                            (GenericValueType::Number(n1), GenericValueType::Number(n2)) => {
                                Ok(n1 < n2)
                            }
                            _ => Err(RuntimeError::InvalidOperation(
                                " < not supported ".to_string(),
                            )),
                        }
                    }
                    match is_less(v1, v2) {
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                        Ok(v) => self.vm_stack.push(GenericValueType::from_bool(v)),
                    }
                }
                OpCode::OpGreaterEqual => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop();

                    // TODO: move this to value, operator overloading (trait ~~~)
                    fn is_greater_equal(
                        v1: GenericValue,
                        v2: GenericValue,
                    ) -> Result<bool, RuntimeError> {
                        match (v1, v2) {
                            (GenericValueType::Number(n1), GenericValueType::Number(n2)) => {
                                Ok(n1 >= n2)
                            }
                            _ => Err(RuntimeError::InvalidOperation(
                                " >= not supported ".to_string(),
                            )),
                        }
                    }
                    match is_greater_equal(v1, v2) {
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                        Ok(v) => self.vm_stack.push(GenericValueType::from_bool(v)),
                    }
                }
                OpCode::OpLessEqual => {
                    let v1 = self.vm_stack.pop();
                    let v2 = self.vm_stack.pop();

                    // TODO: move this to value, operator overloading (trait ~~~)
                    fn is_less_equal(
                        v1: GenericValue,
                        v2: GenericValue,
                    ) -> Result<bool, RuntimeError> {
                        match (v1, v2) {
                            (GenericValueType::Number(n1), GenericValueType::Number(n2)) => {
                                Ok(n1 <= n2)
                            }
                            _ => Err(RuntimeError::InvalidOperation(
                                " <= not supported ".to_string(),
                            )),
                        }
                    }
                    match is_less_equal(v1, v2) {
                        Err(e) => runtime_error(0, e.to_string().as_str()),
                        Ok(v) => self.vm_stack.push(GenericValueType::from_bool(v)),
                    }
                }
                OpCode::OpPrint => {
                    println!("{}", self.vm_stack.pop()) // 我手改成 peek , 書裡面寫 pop, 我在思考....
                }
                OpCode::OpPop => {
                    self.vm_stack.pop();
                }
                OpCode::OpDefineGlobal => {
                    let name = self.read_string(chunk);
                    self.table.set(name.clone(), self.vm_stack.peek(0));
                    self.vm_stack.pop();
                }
                OpCode::OpGetGlobal => {
                    let name = self.read_string(chunk);
                    if let Some(v) = self.table.get(&name) {
                        self.vm_stack.push(v.clone());
                    } else {
                        runtime_error(0, &format!("undefined global variable :{}", name));
                        return InterpretResult::InterpretRunTimeError;
                    }
                }
                OpCode::OpSetGlobal => {
                    let name = self.read_string(chunk);
                    if self.table.set(name.clone(), self.vm_stack.peek(0)) {
                        self.table.delete(&name);
                        runtime_error(0, format!("Undefined global variable :{}", name).as_str());
                    }
                }
                OpCode::OpGetLocal => {
                    let slot = self.read_op_raw(chunk);
                    self.vm_stack.push(self.vm_stack.get_by_index(slot));
                }
                OpCode::OpSetLocal => {
                    let slot = self.read_op_raw(chunk);
                    self.vm_stack
                        .assign_by_index(slot, self.vm_stack.peek(0).clone());
                }
            };
        }
    }

    fn read_string(&mut self, chunk: &mut Chunk) -> String {
        self.read_constant(chunk)
            .as_string()
            .expect("read_string operation should always be valid")
    }

    // read value as original
    fn read_op_raw(&mut self, chunk: &mut Chunk) -> usize {
        let code = chunk.bytecode[self.ip];
        self.ip += 1;
        code
    }

    // read operation code (READ_BYTE )
    fn read_op(&mut self, chunk: &mut Chunk) -> OpCode {
        let code = chunk.bytecode[self.ip];
        self.ip += 1;
        OpCode::from_usize(code)
    }

    fn read_constant(&mut self, chunk: &mut Chunk) -> GenericValue {
        let code = self.read_op_raw(chunk);
        chunk.const_pool.values[code].clone()
    }
}

////////////////////////////////////////////////////////////////
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("==     {}     ==", name);

    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset)
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04}   ", offset);

    let instruction = OpCode::from_usize(chunk.bytecode[offset]);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!(" |     ")
    } else {
        print!("{:04}   ", chunk.lines[offset])
    }

    match instruction {
        OpCode::OpReturn => simple_instruction(instruction, offset),
        OpCode::OpConstant => constant_instruction(instruction, offset, chunk),
        OpCode::OpNegate => simple_instruction(instruction, offset),
        OpCode::OpAdd => simple_instruction(instruction, offset),
        OpCode::OpSubtract => simple_instruction(instruction, offset),
        OpCode::OpMultiply => simple_instruction(instruction, offset),
        OpCode::OpDivide => simple_instruction(instruction, offset),
        OpCode::OpNil => simple_instruction(instruction, offset),
        OpCode::OpFalse => simple_instruction(instruction, offset),
        OpCode::OpTrue => simple_instruction(instruction, offset),
        OpCode::OpNot => simple_instruction(instruction, offset),
        OpCode::OpEqual => simple_instruction(instruction, offset),
        OpCode::OpGreater => simple_instruction(instruction, offset),
        OpCode::OpLess => simple_instruction(instruction, offset),
        OpCode::OpGreaterEqual => simple_instruction(instruction, offset),
        OpCode::OpLessEqual => simple_instruction(instruction, offset),
        OpCode::OpPrint => simple_instruction(instruction, offset),
        OpCode::OpPop => simple_instruction(instruction, offset),
        OpCode::OpDefineGlobal => constant_instruction(instruction, offset, chunk),
        OpCode::OpGetGlobal => constant_instruction(instruction, offset, chunk),
        OpCode::OpSetGlobal => constant_instruction(instruction, offset, chunk),
        OpCode::OpGetLocal => byte_instruction(instruction, offset, chunk),
        OpCode::OpSetLocal => byte_instruction(instruction, offset, chunk),
    }
}

pub fn byte_instruction(op: OpCode, offset: usize, chunk: &Chunk) -> usize {
    let slot = chunk.bytecode[offset + 1];
    println!("{} {}", op, slot);
    offset + 2
}

pub fn simple_instruction(op: OpCode, offset: usize) -> usize {
    println!("{}", op);
    offset + 1
}

pub fn constant_instruction(op: OpCode, offset: usize, chunk: &Chunk) -> usize {
    println!(
        "{:?}, offset: {}, const_pool: {:?}",
        &chunk.bytecode, offset, chunk.const_pool.values
    );
    let constant = chunk.bytecode[offset + 1];
    let val = chunk.const_pool.values[constant].clone();

    println!("{}{}'{}'", op, " ".repeat(15), val);
    offset + 2
}

pub struct VirtualMachineStack {
    pub values: [GenericValue; constants::STACK_MAX as usize],
    pub ptr: usize,
    pub max_size: usize,
}

impl VirtualMachineStack {
    pub fn push(&mut self, value: GenericValue) {
        if self.ptr == self.max_size {
            panic!("[Push] Invalid operation, exceeds stack limit")
        }
        self.values[self.ptr] = value;
        self.ptr += 1;
    }

    pub fn pop(&mut self) -> GenericValue {
        if self.ptr == 0 {
            panic!("[Pop] Invalid operation, empty stack")
        }
        self.ptr -= 1;
        self.values[self.ptr].clone()
    }

    pub fn peek(&self, distance: usize) -> GenericValue {
        /*
           peek value, start from the top of the stack,
           zero means the top value
        */
        if self.ptr == 0 {
            panic!("[Peek] Invalid operation, empty stack ")
        }
        self.values[self.ptr - 1 - distance].clone()
    }
    // Special optimization for OP_NEGATE
    pub fn negate_peek(&mut self) {
        if self.ptr == 0 {
            panic!("[negate_peek] Invalid operation, empty stack ")
        }
        let v = -self.values[self.ptr - 1].clone();
        match v {
            // TODO: put the actual line, not 0
            Ok(v) => self.values[self.ptr - 1] = v,
            Err(e) => runtime_error(0, e.to_string().as_str()),
        }
    }

    pub fn get_by_index(&self, index: usize) -> GenericValue {
        self.values[index].clone()
    }

    pub fn assign_by_index(&mut self, index: usize, value: GenericValue) {
        self.values[index] = value;
    }
}

impl Default for VirtualMachineStack {
    fn default() -> Self {
        VirtualMachineStack {
            values: std::array::from_fn(|_| GenericValue::default()), // Initialize as nil
            ptr: 0,
            max_size: constants::STACK_MAX as usize,
        }
    }
}
