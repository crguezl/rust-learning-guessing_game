# Debugging in Rust

The most practical way to debug Rust is with 3 levels:
1. printing,
2. debugger, and
3. `cargo` tools.

## Step-by-step debugging with breakpoints in VS Code

- Install `rust-analyzer` and an LLDB extension (for example CodeLLDB).
- Open main.rs, click in the left margin to set a breakpoint.
- Run "Run and Debug" and choose an LLDB configuration.
- You can:
    - Step over / into
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
            "cargo": {
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


### **Goal**
See the state of guess before and after read_line: content, length/capacity printed by the program, and pointer/address changes.

### **1) Recommended breakpoints**
1. Place a breakpoint in main.rs for the initial state.
2. Place another in main.rs for the state after reading.
3. Start Rust: Debug guessing_game with F5.

### **2) What to type in the Debug Console (LLDB)**

Use these commands, which work well with Rust:

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

Quick meaning:
1. n: next line (step over).
2. s: enter call (step into).
3. c: continue to the next breakpoint.
4. bt: stack trace.

### **3) Why p guess.len() fails**
LLDB does not reliably support evaluating Rust expressions in many cases, and falls back to Objective-C++.
That is why methods like len() do not resolve in the console even though the program compiles perfectly.

### **4) Reliable way to view len/capacity and pointers**
You already have it in your code with prints:
1. Initial state in main.rs.
2. Final state in main.rs.

This is the most robust approach today for String in Rust during debugging.

### **5) Suggested 30-second flow**
1. F5.
2. When it stops in main.rs, run frame variable guess.
3. Press n several times until you pass read_line.
4. Enter text when prompted.
5. When it stops in main.rs, run frame variable guess again.
6. Compare it with the printed output of len/cap and addresses.


## Quick debugging with `println!` or `dbg!`
- `println!` for clear messages.
- `dbg!(expr)` prints the value and also file/line.
- Example:
```rust
dbg!(secret_number);
dbg!(&guess, guess.len(), guess.capacity());
```

## Compile in debug mode and run

- `cargo run` already uses the `dev` profile (without aggressive optimizations, with symbols).
- For stricter errors:
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

## When there are panics

- To see the backtrace:
```bash
RUST_BACKTRACE=1 cargo run
```
- Full backtrace:
```bash
RUST_BACKTRACE=full cargo run
```

## For ownership/borrowing bugs

- `cargo check` usually gives the main diagnosis.
- `rust-analyzer` highlights errors as you type.
- `dbg!` helps track flow and intermediate states without changing much code.

In your current program, a useful first step would be to place a breakpoint right before and after `read_line` in main.rs, and observe `guess.len()`, `guess.capacity()` and `guess.as_ptr()` to see when the buffer changes.

If you want, I can prepare a minimal VS Code configuration (`launch.json`) to debug this project with one click.

