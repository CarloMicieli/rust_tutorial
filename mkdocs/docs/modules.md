# Moduli e Crates

_Moduli_ sono da sempre stati usati per organizzare le varie componenti di una programma. L'approccio di Rust sull'argomento è a dir poco singolare.

Dopo aver creato un nuovo progetto con **Cargo** siamo pronti a partire:

```
$ cargo new modules
  Created binary (application) `modules` project
```

```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
```

in questo momento tutti gli elementi del programma si trovano nel file `main.rs`:

```rust
use std::fmt;

fn main() {
    let one_half = Rational::new(1, 2);
    println!("One half, or {}", one_half);
}

struct Rational {
    n: i32,
    d: i32,
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}

impl Rational {
    fn new(n: i32, d: i32) -> Rational {
        Rational { n, d }
    }
}
```

```
$ cargo run
   Compiling modules v0.1.0 (file:///home/carlo/Projects/rust/modules)                                 
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/modules`
One half, or 1/2
```

Moduli in Rust possono contenere: _function_, _tipi_, _valori_. Di default gli elementi in Rust sono *privati*: possono essere resi pubblici (e visibili da altri moduli) usando la parola chiave `pub`.

Una unità di compilazione in Rust si chiama **Crate** e include un modulo _root_, per le librerie è contenuto nel file `lib.rs` per gli eseguibili `main.rs`.

```rust
fn main() {
    let one_half = math::Rational::new(1, 2);
    println!("One half, or {}", one_half);
}

mod math {
    use std::fmt;

    pub struct Rational {
        n: i32,
        d: i32,
    }

    impl fmt::Display for Rational {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}/{}", self.n, self.d)
        }
    }

    impl Rational {
        pub fn new(n: i32, d: i32) -> Rational {
            Rational { n, d }
        }
    }
}
```

> Name lookup in expressions is relative to the module in which the expression appears unless the name is fully qualified.


Nome _qualified_:

- `::` root
- `super::`: parent module
- `self::` current module


Ci sono tre modi per dichiarare un modulo `math`:
- con un blocco `pub mod math {}` definito nel modulo _root_ (`main.rs` o `lib.rs` a seconda del tipo di progetto);
- in un file `./math.rs`
- in un file `./math/mod.rs`


- Opzione 1

```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
    ├── main.rs
    └── math.rs
```

 - Opzione 2

```
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
    ├── main.rs
    └── math
        └── mod.rs
```

Nel modulo _root_ è obbligatorio definire i moduli usati:

```rust
pub mod math;

fn main() {
    let one_half = math::Rational::new(1, 2);
    let one_third = math::Rational::new(1, 3);
    println!("One half plus one third is {}", one_half + one_third);
    println!("One half plus one third is {}", math::to_rational(42));
}
```

il contenuto di `math/mod.rs` e `math.rs` è identico:

```rust
use std::fmt;
use std::ops;

/// Una semplice funzione
pub fn to_rational(n: i32) -> Rational {
    Rational::from_int(n)
}

/// Rappresenta una frazione
#[derive(Clone, Copy, Debug)]
pub struct Rational {
    n: i32,
    d: i32,
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rational { n, d: 1 } => write!(f, "{}", n),
            Rational { n, d } => write!(f, "{}/{}", n, d),
        }
    }
}

impl Rational {
    pub fn new(n: i32, d: i32) -> Rational {
        Rational { n, d }
    }

    fn from_int(n: i32) -> Rational {
        Rational::new(n, 1)
    }
}

impl ops::Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        let Rational { n: n1, d: d1 } = self;
        let Rational { n: n2, d: d2 } = other;
        Rational {
            n: (n1 * d2) + (d1 * n2),
            d: d1 * d2,
        }
    }
}
```

== `Crate`s

Un `Crate` è il nome che una libreria prende in Rust, il loro scopo è permette la condisione di codice tra diversi programmi.

Per include un `Crate` nel proprio progetto dobbiamo aggiungerlo alle dipendenze nel file `Cargo.toml`: iniziamo aggiungendo `crono`, una libreria rust per la gestione delle date:

```toml
[package]
name = "modules"
version = "0.1.0"
authors = ["CarloMicieli <carlo.micieli@gmail.com>"]

[dependencies]
chrono = "0.4"
```

```
$ cargo build
    Blocking waiting for file lock on the registry index
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling num-traits v0.2.6                                                                         
   Compiling num-integer v0.1.39
   Compiling libc v0.2.43
   Compiling time v0.1.40
   Compiling chrono v0.4.6
   Compiling modules v0.1.0 (file:///home/carlo/Projects/rust/modules)
    Finished dev [unoptimized + debuginfo] target(s) in 8.40s
```

Non siamo ancora pronti per usare le funzionalità offerte da questo **crate**: dobbiamo importarlo nel modulo _root_, nel nostro caso `main.rs`:

```rust
extern crate chrono;

pub mod math;

fn main() {
    let one_half = math::Rational::new(1, 2);
    let one_third = math::Rational::new(1, 3);
    println!("One half plus one third is {}", one_half + one_third);
    println!("One half plus one third is {}", math::to_rational(42));
}
```

Con `extern crate chrono` abbiamo ottenuto due risultati allo stesso tempo: incluso il *crate* `chrono` nel programma e importato il modulo nel modulo corrente.

```rust
extern crate chrono;
use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("It's {}", local);
}
```






































## Bibliografia
- https://blog.infinitenegativeutility.com/2017/8/the-basic-principles-of-rust-modules
