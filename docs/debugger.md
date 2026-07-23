# Depuración en Rust

La forma más práctica de depurar Rust es con 3 niveles: 
1. impresión, 
2. depurador y 
3. herramientas de `cargo`.

## Depuración paso a paso con breakpoints en VS Code

- Instala `rust-analyzer` y una extensión LLDB (por ejemplo CodeLLDB).
- Abre main.rs, haz clic en el margen izquierdo para poner breakpoint.
- Ejecuta “Run and Debug” y elige configuración LLDB.
- Puedes:
  - Step over / into
  - inspeccionar variables
  - ver stack frames
  - evaluar expresiones

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

## Mini Guía de Depuración


### **Objetivo**
Ver el estado de guess antes y después de read_line: contenido, longitud/capacidad impresas por el programa, y cambios de dirección.

### **1) Breakpoints recomendados**
1. Coloca un breakpoint en main.rs para el estado inicial.
2. Coloca otro en main.rs para el estado después de leer.
3. Inicia Rust: Debug guessing_game con F5.

### **2) Qué escribir en la Debug Console (LLDB)**

Usa estos comandos, que sí funcionan bien con Rust:

~~~text
frame variable guess
frame variable -R guess
v guess
~~~

Para ver todo el frame actual:

~~~text
frame variable
~~~

Para navegar:

~~~text
n
s
c
bt
~~~

Significado rápido:
1. n: siguiente línea (step over).
2. s: entrar en llamada (step into).
3. c: continuar hasta próximo breakpoint.
4. bt: stack trace.

### **3) Por qué falla p guess.len()**
LLDB no soporta bien evaluar expresiones Rust en muchos casos, y cae a Objective-C++.
Por eso métodos como len() no se resuelven en consola aunque el programa compile perfecto.

### **4) Forma fiable de ver len/capacity y punteros**
Ya la tienes en tu código con prints:
1. Estado inicial en main.rs.
2. Estado final en main.rs.

Esa es la vía más robusta hoy para String en Rust durante debug.

### **5) Flujo sugerido de 30 segundos**
1. F5.
2. Al parar en main.rs, ejecuta frame variable guess.
3. n varias veces hasta pasar read_line.
4. Introduce texto cuando lo pida.
5. Al parar en main.rs, vuelve a ejecutar frame variable guess.
6. Compara con la salida impresa de len/cap y direcciones.


## Depuración rápida con `println!` o `dbg!`
- `println!` para mensajes claros.
- `dbg!(expr)` imprime valor y además archivo/línea.
- Ejemplo:
```rust
dbg!(secret_number);
dbg!(&guess, guess.len(), guess.capacity());
```

## Compilar en modo debug y ejecutar

- `cargo run` ya usa perfil `dev` (sin optimizaciones agresivas, con símbolos).
- Para errores más estrictos:
```bash
cargo check
cargo clippy
```

## Alternativa por terminal con LLDB/GDB

- Compila:
```bash
cargo build
```
- LLDB:
```bash
lldb target/debug/guessing_game
```
- Dentro de lldb:
```text
breakpoint set --name main
run
next
step
frame variable
```

## Cuando hay panics

- Para ver backtrace:
```bash
RUST_BACKTRACE=1 cargo run
```
- Backtrace completo:
```bash
RUST_BACKTRACE=full cargo run
```

## Para bugs de ownership/borrowing

- `cargo check` suele dar el diagnóstico principal.
- `rust-analyzer` te marca errores al vuelo.
- `dbg!` ayuda a seguir flujo y estados intermedios sin cambiar mucho código.

En tu programa actual, un primer paso útil sería poner breakpoint justo antes y después de `read_line` en main.rs, y observar `guess.len()`, `guess.capacity()` y `guess.as_ptr()` para ver cuándo cambia el buffer.

Si quieres, te preparo una configuración mínima de VS Code (`launch.json`) para depurar este proyecto con un clic.

