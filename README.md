# rust

## Getting Started

### Installation

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustc --version
```

### funcname() and funcname!()

-> Adding ! at the end of a function name means its a macro
-> _macros don’t always follow the same rules as functions_

### Cargo

-> Used to compile and build Rust projects along with dependencies
-> Create a new Cargo project by

```
cargo new PROJECTNAME
```

-> _[package]_ => All package info _[dependencies]_ => All libraries used by Rust project

```
cargo build // builds the cargo project
cargo run // builds and runs the project
cargo check // compiles but does not create an executable
cargo build --release // builds release version with optimizations to run faster
```

### Things to remember from Guessing Game Program

-> By default variables are immutable in Rust.
-> Use _mut_ to create mutable variables
-> Associator syntax _(::)_

```
Type::AssociatedFunction()
```

-> To receive Input & - Reference

```
io::stdin()
    .read_line(&mut VARNAME)
```

-> Error handing is done through _.expect()_. read*line method returns \_Result* type.
-> Result can of _Ok_ and _Err_ variant. An error in the method crashes the program. Not using _.expect()_ gives a warning during compilation.
-> To print variables, curly braces are used _{varName}_
-> Crates or libraries used in Rust program are added as

```
[dependencies]
crateName = "VERSION"
```

-> _cargo build_ can be used to downloaded dependency crates.
-> _cargo update_ can be run to update dependencies. Source - creates.io
-> cargo.lock contains the version of all creates, so that the same build can be reproduced.
-> _cargo doc --open_ opens the documentation for all crates used locally.
-> rand crate

```
rand::thread_rng() // function that returns a random number
    .gen_range(start..=end) // gives the range between which number has to be generated, start and end number are inclusive. Belongs to Rng trait.
```

-> _match_ is an arm, compares and executes code based on its result. In this case 3 patterns are present _Ordering::Greater_, _Ordering::Less_ and _Ordering::Equal_.
-> Variable _shadowing_, lets us reuse variables instead of creating a new one. Mostly useful in case of type conversions.
-> To annotate variable type _(:)_ can be used.

```
let number_unsigned: u32 = 48;
```

-> A loop can be created by _loop {}_ and is exited using _break;_.
-> Instead of using .expect to crash program during errors, we can handle the error like

```
let guess: u32 = guess.trim().parse() { // checking if user input is a number
    Ok(num) => num,
    Err(_) => continue,
};
```

-> In above snippet, _num_ is the resultant value returned by _parse()_ function. The (\_) is a catchall value which says match all possible error values.

### Constants and Shadowing in Rust

-> Constants in Rust, cant use mut keyword. Syntax,

```
const SOME_CONSTANT_NAME: u32 =200
```

-> The type of the constant must always be annotated.

-> Immutable variables can be shadowed and assigned different values using let keyword.

```
fn main() {
    let x = 5;
    let x = x + 1; // 6

    {
        let x = x * 2; // 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); //6
}
```

### Data Types in Rust

-> Scalar and Compound Data Types
-> Scalar Type - Represent single value
-> Examples

```
let integer_var: u32 = 100; //Integer
let floar_var: f32 = 5.0; //Float
let boolean_var: bool = true; //Boolean
let character_var: char = 'a'; //Character
```

-> For Integers, u - unsigned : Values are always positive and i - signed : Can have negative values
-> Each signed variant can store numbers from -(2^(n - 1)) to 2^(n - 1) - 1.

-> Compound Type - Represent Multiple values.
