# lox-lang

A tree-walking interpreter for the [Lox language](https://craftinginterpreters.com/), implemented in Rust. Built as a complete study of interpreter design — from scanning and parsing through semantic analysis and runtime evaluation.

---

## Architecture

The interpreter is structured as a classic multi-phase pipeline:

```
Source Code
    │
    ▼
┌─────────┐
│  Lexer  │  &str → Vec<Token>
└─────────┘
    │
    ▼
┌─────────┐
│ Parser  │  Vec<Token> → AST (Expr / Stmt)
└─────────┘
    │
    ▼
┌──────────────┐
│   Resolver   │  AST → resolve variable depths (side-effects on Interpreter)
└──────────────┘
    │
    ▼
┌─────────────┐
│ Interpreter │  AST → LoxValue (tree-walking eval)
└─────────────┘
```

Each phase is intentionally decoupled. The `Resolver` performs a static pass over the AST before any execution begins, binding each variable reference to a fixed scope depth. This depth is stored in the `Interpreter` via a `HashMap<usize, usize>` (keyed on expression node identity), allowing the runtime to skip straight to the correct environment frame without dynamic name lookup.

---

## Scope Resolution

Lexical scoping is the most architecturally interesting part of this implementation.

### Environment Chain

Environments are represented as a linked list of `Rc<RefCell<Environment>>` nodes:

```rust
pub struct Environment {
    pub enclosing: Option<Env>,
    pub values: HashMap<String, LiteralValue>,
}
```

`Rc` provides shared ownership across closures and the interpreter's execution stack. `RefCell` enables interior mutability for variable assignment without requiring a mutable borrow on the entire chain — necessary because closures can mutate captured variables long after their defining scope has returned.

### Scope Traversal

The `Resolver` walks the AST in a separate pass, computing the number of hops from a variable's use site to its declaration:

```rust
fn resolve_local(&mut self, expr: &Expr, name: &Token) {
    for (i, scope) in self.scopes.iter().enumerate().rev() {
        if scope.contains_key(&name.lexeme.clone()) {
            self.interpreter.resolve(expr.id, self.scopes.len() - 1 - i);
            return;
        }
    }
    // Not found in any local scope → treat as global
}
```

At runtime, `get_at` and `assign_at` on the `Interpreter` traverse the environment chain by exactly that depth:

```rust
fn ancestor(&self, env: Rc<RefCell<Environment>>, depth: usize) -> Rc<RefCell<Environment>> {
    let mut current = env;
    for _ in 0..depth {
        let next = current.borrow().enclosing.clone().unwrap();
        current = next;
    }
    current
}

pub fn get_at(&self, distance: usize, name: &str) -> Option<LiteralValue> {
    ancestor(Rc::clone(&self.environment), distance)
        .and_then(|env| env.borrow().values.get(name).cloned())
}

pub fn assign_at(&self, distance: usize, name: &Token, value: LiteralValue) {
    ancestor(Rc::clone(&self.environment), distance)
        .and_then(|env| env.borrow_mut().values.insert(name.lexeme.clone(), value));
}
```

This eliminates any dynamic name search at runtime — variable resolution is O(depth) in scope hops, which in practice is near-constant for well-structured code.

---

## Functions & Closures

`LoxFunction` captures the environment at the point of definition, not the point of call:

```rust
pub struct LoxFunction {
    pub declaration: Rc<FunctionStmt>,
    pub closure: Env,
}
```

When a function is called, a new `Environment` is created with `closure` as its enclosing scope, not the caller's environment. This correctly implements lexical (static) scoping semantics and allows closures to outlive their defining scope:

```lox
fun makeCounter() {
  var i = 0;
  fun increment() {
    i = i + 1;
    return i;
  }
  return increment;
}

var counter = makeCounter();
counter(); // 1
counter(); // 2
```

The `i` binding in `increment`'s closure environment persists even after `makeCounter` has returned, because `Rc<RefCell<Environment>>` keeps the allocation alive as long as any closure holds a reference to it.

---

## Getting Started

**Prerequisites:** Rust 1.70+

```bash
git clone https://github.com/your-username/lox-lang
cd lox-lang
cargo build --release
```

**Run a Lox file:**

```bash
./target/release/lox-lang script.lox
```

**Start the REPL:**

```bash
./target/release/lox-lang
```

---

## What's Not Implemented

This is a tree-walking interpreter — it makes no attempt at bytecode compilation or optimization. The following Lox features are also not included in this implementation:

- Classes and inheritance
- Standard library (beyond `print` and `clock`)

---

## References

- [Crafting Interpreters — Robert Nystrom](https://craftinginterpreters.com/)
- [The Rust Reference — Interior Mutability](https://doc.rust-lang.org/reference/interior-mutability.html)
