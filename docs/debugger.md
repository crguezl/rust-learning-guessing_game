# Debugging in Rust

The most practical way to debug Rust is with 3 levels:

1. Printing,

2. Debugger, and

3. Loading tools.

## Step-by-step debugging with breakpoints in VS Code

- Install `rust-analyzer` and an LLDB extension (for example, CodeLLDB).

- Open main.rs, click in the left margin to set a breakpoint.

- Run “Run and Debug” and choose LLDB settings.

- Can: 
- Step over/into 
- inspect variables 
- view stack frames 
- evaluate expressions

## .vscode/launch.json

```json
{ 
"version": "0.2.0", 
"configurations": [ 
{ 
"name": "Rust: Debug guessing_game", 
"type": "lldb", 
"request": "launch", 
"post": { 
"args": [ 
"build", 
"--bin", 
"guessing_game" 
], 
"filter": { 
"name": "guessing_game", 
"kind": "bin" 
} 
}, 
"args": [], 
"cwd": "${workspaceFolder}", 
"sourceLanguages": [ 
"rust" 
] 
}, 
{ 
"name": "Rust: Debug unit tests (guessing_game)", 
"type": "lldb",

"request": "launch",

"cargo": {

"args": [

"test",
"--no-run",
"--bin",
"guessing_game"
],

"filter": {

"name": "guessing_game",
"kind": "bin"
}
},

"cwd": "${workspaceFolder}",

"sourceLanguages": [
"rust"
]
}
]
}
```

## Mini Debugging Guide

### **Objective**
View the state of `guess` before and after `read_line`: content, length/capacity printed by the program, and address changes.

### **1) Recommended Breakpoints**
1. Place a breakpoint in `main.rs` for the initial state.

2. Place another breakpoint in `main.rs` for the state after reading.

` ...
`````````````
`````
3. Start Rust: Debug guessing_game with F5.

### **2) What to type in the Debug Console (LLDB)**

Use these commands, which work correctly with Rust:

~~~text
frame variable guess
frame variable -R guess
v guess
~~~

To see the entire current frame:

~~~text
frame variable
~~~

To navigate:

~~~text
n
s
c
bt
~~~

Quick meanings:
1. n: step over.

2. s: step into.

3. c: continue to the next breakpoint.

4. bt: stack trace.

### **3) Why `p guess.len()` fails**
LLDB doesn't handle evaluating Rust expressions well in many cases, and falls back to Objective-C++. Therefore, methods like `len()` don't resolve in the console even if the program compiles perfectly.

### **4) Reliable way to view len/capacity and pointers**
You already have this in your code with `prints`:
1. Initial state in `main.rs`.

2. Final state in `main.rs`.

This is the most robust way to view Strings in Rust during debugging.

### **5) Suggested 30-second flow**
1. Press F5.

2. When stopped at `main.rs`, execute `frame variable guess`.

3. Press `n` several times until `read_line` is reached.

4. Enter text when prompted.

5. When stopped at `main.rs`, execute `frame variable guess` again.

6. Compare with the printed output of `len/capacity` and addresses.

## Quick debugging with `println!` or `dbg!`
- `println!` for clear messages.

- `dbg!(expr)` prints the value and also the file/line.

- Example:
```rust
dbg!(secret_number);

dbg!(&guess, guess.len(), guess.capacity());

``

## Compile in debug mode and run

- `cargo run` already uses the `dev` profile (without aggressive optimizations, with symbols).

- For more stringent errors:
```bash
cargo check
cargo clippy
```

## Terminal alternative with LLDB/GDB

- Compile:
```bash
cargo build
```
- LLDB:
```bash
lldb target/debug/guessing_game
```
- Inside lldb:
```text
breakpoint set --name main
run
next
step
frame variable
```

## When panics occur

- To view backtrace:
```bash
RUST_BACKTRACE=1 cargo run
```
- Full backtrace:
```bash
RUST_BACKTRACE=full cargo run
```

## For ownership/borrowing bugs

- `cargo check` usually provides the main diagnosis.

- `rust-analyzer` highlights errors on the fly.

- `dbg!` helps track flow and intermediate states without changing much code.

In your current program, a useful first step would be to set breakpoints just before and after `read_line` in `main.rs`, and observe `guess.len()`, `guess.capacity()`, and `guess.as_ptr()` to see when the buffer changes.

If you'd like, I can prepare a minimal VS Code configuration (`launch.json`) for you to debug this project with a single click.