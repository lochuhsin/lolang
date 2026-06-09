<div align="center">

# 🦀 lolang

**A small programming language, built from scratch in Rust.**

A single-pass bytecode compiler and a stack-based virtual machine for a dynamically-typed,
Lox-flavored language — written end to end, from raw source text to executed bytecode.

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Edition](https://img.shields.io/badge/edition-2021-blue)](https://doc.rust-lang.org/edition-guide/)
[![Architecture](https://img.shields.io/badge/architecture-bytecode%20VM-orange)](#-under-the-hood)
[![Status](https://img.shields.io/badge/status-work%20in%20progress-yellow)]()

</div>

---

## 📖 Table of Contents

- [About](#-about)
- [The Pipeline](#-the-pipeline)
- [Getting Started](#-getting-started)
- [A Tour of the Language](#-a-tour-of-the-language)
- [Feature Status](#-feature-status)
- [Under the Hood](#-under-the-hood)
- [Project Layout](#-project-layout)
- [Development](#-development)
- [Roadmap](#-roadmap)
- [Acknowledgements](#-acknowledgements)

---

## 🌱 About

**lolang** is a learning project: a from-scratch implementation of a programming language,
written in Rust. It follows the *bytecode virtual machine* architecture popularized by
[*Crafting Interpreters*](https://craftinginterpreters.com/) — instead of walking an AST,
the compiler lowers source code into a flat array of bytecode instructions, which a
purpose-built stack machine then executes.

The language itself is dynamically typed and C-flavored, with numbers, booleans, strings,
`nil`, variables, and the usual arithmetic and comparison operators. You can run it two ways:

- **Interactively**, via a REPL, or
- **From a file**, by passing a source path on the command line.

> The project also keeps a `release/0.0.1-interpreter` branch — an earlier tree-walking
> interpreter — so you can compare the two classic approaches to running a language:
> walking the syntax tree directly vs. compiling to bytecode and running it on a VM.

---

## 🔧 The Pipeline

Source text flows through a four-stage pipeline before it runs. Everything is **single-pass**:
the compiler scans, parses, and emits bytecode in one walk over the tokens.

```
   source string
        │
        ▼
┌───────────────┐   characters → tokens
│    Scanner    │   hand-written lexer; DFA-style keyword matching,
│  scanner.rs   │   string/number literals, // comments, line tracking
└───────┬───────┘
        │  Token stream
        ▼
┌───────────────┐   Pratt parser (precedence-climbing)
│ Parser + Rules│   parser.rs drives the token cursor;
│ rules.rs      │   rules.rs maps each token → (prefix, infix, precedence)
└───────┬───────┘
        │
        ▼
┌───────────────┐   emits opcodes as it parses
│   Compiler    │   declarations · statements · expressions
│  compiler.rs  │   global + local variables, block scopes
└───────┬───────┘
        │  writes into
        ▼
┌───────────────┐   bytecode · line info · constant pool
│     Chunk     │   chunk.rs — the compiled program
└───────┬───────┘
        │  fed to
        ▼
┌───────────────┐   stack-based dispatch loop
│ Virtual Machine│  vm.rs — pushes/pops GenericValues,
│  vm.rs        │   resolves globals via a hash Table
└───────────────┘
        │
        ▼
      output
```

---

## 🚀 Getting Started

### Prerequisites

You only need the Rust toolchain. If you don't have it yet:

```bash
curl https://sh.rustup.rs -sSf | sh     # or: make install-cargo
```

### Build

```bash
git clone https://github.com/lochuhsin/lolang.git
cd lolang
cargo build              # debug build
cargo build --release    # optimized build
```

### Run the REPL

With no arguments, lolang drops you into an interactive prompt:

```bash
cargo run
```

```
>> print 1 + 2 * 3;
7
>> var name = "lolang";
>> print "hello, " + name;
hello, lolang
>> exit
```

### Run a source file

Pass a path with `--path` (alias `-p`):

```bash
cargo run -- --path program.lo
```

> File extension is just convention — `.lo` is a friendly choice for **lo**lang.

> 💡 **Debug builds are verbose by design.** When compiled with `debug_assertions`, the VM
> prints a disassembly of every chunk and traces the stack as each instruction executes —
> a built-in window into how the bytecode runs. Use `--release` for quiet output.

---

## 🎨 A Tour of the Language

```js
// Comments start with two slashes.

// --- Values ---
print 42;              // numbers are 64-bit floats
print 3.14159;
print true;            // booleans
print nil;             // the absence of a value
print "hello world";   // strings

// --- Arithmetic ---
print 1 + 2 * 3;       // 7   — precedence is respected
print (1 + 2) * 3;     // 9   — grouping with parentheses
print 10 / 4;          // 2.5
print -7;              // unary negation

// --- String concatenation ---
print "foo" + "bar";   // foobar

// --- Comparison & equality ---
print 10 > 3;          // true
print 2 <= 2;          // true
print 1 == 1;          // true
print 1 != 2;          // true

// --- Logical negation ---
print !true;           // false

// --- Variables ---
var greeting = "hi";
print greeting;

// --- Block scope ---
{
    var local = "only visible in here";
    print local;
}
```

---

## ✅ Feature Status

| Area | Feature | Status |
|------|---------|:------:|
| **Values** | Numbers (`f64`), Booleans, `nil`, Strings | ✅ |
| **Arithmetic** | `+` `-` `*` `/`, unary `-` | ✅ |
| **Strings** | Concatenation with `+` | ✅ |
| **Comparison** | `==` `!=` `<` `<=` `>` `>=` | ✅ |
| **Logical** | `!` (not) | ✅ |
| **Logical** | `and` / `or` | 🚧 planned |
| **Statements** | `print`, expression statements, blocks `{ }` | ✅ |
| **Variables** | Global variables (`var`) | ✅ |
| **Variables** | Local variables & lexical scoping | 🚧 in progress |
| **Control flow** | `if` / `else`, `while`, `for` | 🚧 planned |
| **Functions** | `fun`, `return`, calls | 🚧 planned |
| **Classes** | `class`, `this`, `super` | 🚧 planned |
| **Operators** | Ternary `?:`, modulo `%` | 🚧 planned |
| **Runtime** | Garbage collection | 🚧 planned |
| **Tooling** | REPL, file runner, bytecode disassembler | ✅ |

> ✅ = working &nbsp; 🚧 = scaffolded / on the roadmap. Keywords like `if`, `for`, `fun`, and
> `class` are already recognized by the scanner — wiring them through the compiler and VM is
> the next chapter of work.

---

## 🔬 Under the Hood

### Values

Every runtime value is a `GenericValue` enum ([`values.rs`](src/values.rs)):

```rust
enum GenericValueType {
    Bool(bool),
    Number(f64),
    Object(DynamicSizeObject),   // heap-allocated, e.g. strings
    Nil,
}
```

Arithmetic and comparison are implemented through Rust's operator-overloading traits
(`Add`, `Sub`, `Mul`, `Div`, `Neg`), each returning a `Result` so the VM can raise a clean
runtime error on, say, adding a number to `nil` or dividing by zero. `DynamicSizeObject`
already carries `prev`/`next` links — groundwork for a future garbage collector.

### The Chunk

A [`Chunk`](src/chunk.rs) is a compiled program: a flat `Vec` of bytecode, a parallel array
of source line numbers (for error reporting), and a **constant pool** holding literals such
as numbers and string names.

### The Bytecode

The VM understands a compact instruction set ([`vm.rs`](src/vm.rs)):

| Category | Opcodes |
|----------|---------|
| Constants & literals | `OpConstant`, `OpNil`, `OpTrue`, `OpFalse` |
| Arithmetic | `OpAdd`, `OpSubtract`, `OpMultiply`, `OpDivide`, `OpNegate` |
| Logic & comparison | `OpNot`, `OpEqual`, `OpGreater`, `OpLess`, `OpGreaterEqual`, `OpLessEqual` |
| Variables | `OpDefineGlobal`, `OpGetGlobal`, `OpSetGlobal`, `OpGetLocal`, `OpSetLocal` |
| Statements & stack | `OpPrint`, `OpPop`, `OpReturn` |

### The Virtual Machine

The VM is a classic **stack machine**. It keeps a fixed-size value stack, an instruction
pointer, and a hash [`Table`](src/table.rs) for global variables. The core is a single
dispatch loop that reads one opcode at a time, manipulates the stack, and advances the IP:

```text
OpConstant 1.0   →  push 1.0
OpConstant 2.0   →  push 2.0
OpAdd            →  pop 2.0, pop 1.0, push 3.0
OpPrint          →  pop 3.0, print it
OpReturn         →  done
```

A built-in **disassembler** (`disassemble_chunk` / `disassemble_instruction`) prints
human-readable bytecode, which the debug build emits automatically so you can watch the
compiler's output and the VM's execution side by side.

---

## 📂 Project Layout

```
lolang/
├── Cargo.toml              # crate manifest (clap, lazy_static)
├── Makefile                # build / run / coverage shortcuts
├── src/
│   ├── main.rs             # CLI entry point — REPL & file runner (clap)
│   ├── lib.rs              # module wiring
│   ├── scanner.rs          # lexer: source → tokens
│   ├── tokens.rs           # TokenType + Token definitions
│   ├── parser.rs           # Pratt parser state & token cursor
│   ├── rules.rs            # precedence table & parse rules
│   ├── compiler.rs         # single-pass compiler → bytecode
│   ├── chunk.rs            # bytecode container + constant pool
│   ├── vm.rs               # stack-based virtual machine + disassembler
│   ├── values.rs           # runtime value types & operators
│   ├── table.rs            # hash table for global variables
│   ├── constants.rs        # tunables (e.g. stack size)
│   └── errors.rs           # error reporting helpers
└── tests/
    └── integration_tests.rs
```

Unit tests live next to the code they cover (`scanner_test.rs`, `parser_test.rs`,
`compiler_test.rs`).

---

## 🛠 Development

```bash
cargo test                 # run the test suite
cargo build                # debug build (verbose VM tracing)
cargo build --release      # optimized, quiet build
make coverage              # HTML coverage report via cargo-tarpaulin
```

Useful Make targets:

| Target | Description |
|--------|-------------|
| `make release` | Build in release mode and launch the REPL |
| `make coverage` | Generate an HTML coverage report (`cargo-tarpaulin`) |
| `make install-cargo` | Install the Rust toolchain (first-time setup) |
| `make install-cargo-dep` | Install dev dependencies (`cargo-tarpaulin`) |

---

## 🗺 Roadmap

- [ ] `and` / `or` short-circuiting logical operators
- [ ] Finish local variables & lexical scoping
- [ ] Control flow: `if` / `else`, `while`, `for`
- [ ] Functions, call frames, and `return`
- [ ] Classes, methods, `this`, and `super`
- [ ] Ternary `?:` and modulo `%`
- [ ] Garbage collection for heap objects
- [ ] Richer runtime error messages with accurate line numbers

---

## 🙏 Acknowledgements

The design follows Robert Nystrom's wonderful book
[**Crafting Interpreters**](https://craftinginterpreters.com/), reimagined in Rust.
If you're curious about how programming languages work, start there.

---

<div align="center">

*Built with 🦀 and curiosity.*

</div>
