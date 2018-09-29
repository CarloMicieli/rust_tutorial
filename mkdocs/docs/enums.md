# Enum

```rust
enum Exp {
    Val { n: i32 },
    Variable { name: String, exp: Box<Exp> },
    Sum { lhs: Box<Exp>, rhs: Box<Exp> },
}
```

```rust
impl Exp {
    pub fn eval(exp: &Exp) -> i32 {
        match exp {
            Exp::Val { n } => *n,
            Exp::Variable { name: _, exp: e } => Exp::eval(&e),
            Exp::Sum { lhs, rhs } => Exp::eval(&lhs) + Exp::eval(&rhs),
        }
    }
}
```
