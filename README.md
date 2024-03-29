# Rust Note

- **Command-Line Rust** by Ken Youens-Clark: https://www.oreilly.com/library/view/command-line-rust/9781098109424/ 
- **Crust of Rust** by Jon Gjengset: https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa
- **The Little Book of Rust Macros**: https://github.com/veykril/tlborm

### `str` vs `&str` vs `String`

- `str -> [char]`, `str` is a sequence of chars, it doesn't know the size.
- `&str -> &[char]`, `&str` is a reference to a sequence of chars, so it knows both the start and the size of that char sequence.
- `String -> Vec<char>`, `String` is heap allocated where `&str` can be pointed to something on the stack or the heap.

If I have a `String`, going to a `&str` is trivial, it's cheap(AsRef).

Going the other way is harder, `&str -> String`, it's expensive(memcpy).

ref: https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2

### Indexing a String in ASCII

Since every ASCII byte is one `u8` long, we could convert the `String` into `Vec<u8>`.

```rs
let s = "hello world".to_string();
let v: Vec<u8> = s.into();
println!("{}", v[0]); // b'h'
println!("{}", v[4]); // b'o'
```

![indexing-string](./images/indexing-string.png)

### Use `as_bytes()` to iterate slice of ASCII bytes 

Performance Hint

All the inputs and outputs are in ASCII. Rust `String`s and `&str` are utf8, so while one might expect `"Hello".chars()` to be simple, it actually has to check each char to see if it's 1, 2, 3 or 4 `u8`s long. If we know a `&str` is ASCII then we can call .`as_bytes()` and refer to the underlying data as a `&[u8]` (byte slice). Iterating over a slice of ASCII bytes is much quicker as there are no codepoints involved - every ASCII byte is one `u8` long.

### cheat for compile fail test

```rs
/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;
```
ref: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes

### iterators

High level overview

```rs
fn main() {
    let mut iter = vec!["a", "b", "c"].into_iter();
    while let Some(e) = iter.next() {}
}
```

### why not generic?

Use a associated type make it easier to use a iterator because we don't have to specify the type for iterator.

```rs
// prefer this
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// not this
trait Iterator<Item> {
    fn next(&mut self) -> Option<Item>;
}
```

The advice is really just to "Use associated types whenever you can".

Associated types are easier to maintain but not allow multiple implementation for the trait.

```rs
trait Superpower {
    type Item;
    fn dig(&self) -> Option<Self::Item>;
}

trait SuperSoda<T> {
    fn dig(&self) -> Option<T>;
}

struct Leo;

impl Superpower for Leo {
    type Item = String;

    fn dig(&self) -> Option<Self::Item> {
        Some("Hello".to_string())
    }
}

impl SuperSoda<String> for Leo {
    fn dig(&self) -> Option<String> {
        Some("Bon!".to_string())
    }
}

impl SuperSoda<i32> for Leo {
    fn dig(&self) -> Option<i32> {
        Some(42)
    }
}

fn main() {
    let leo = Leo {};
    println!("{:?}", Superpower::dig(&leo));
    println!("{:?}", SuperSoda::<i32>::dig(&leo));
    println!("{:?}", SuperSoda::<String>::dig(&leo));
}

```

### iterator only do borrowing

```rs
fn main() {
    let vs = vec![1, 2, 3];
    for v in vs {
        // consumes vs, owned v
    }
    for v in vs.iter() {
        // borrows vs, & to v
    }
    for v in &vs {
        // equivalent to vs.iter()
    }
}
```

### flatten

![flatten](./images/flatten.png)

### scoped threads can borrow non-`'static` data

- [Function std::thread::scope](https://doc.rust-lang.org/std/thread/fn.scope.html)
- [parallel letter frequency](./exercism/parallel-letter-frequency/)

### Static Dispatch and Dynamic Dispatch

```rs
impl String {
    // static dispatch
    pub fn contains(&self, p: impl Pattern) -> bool {
        p.is_contained_in(self);
    }
}
```

A copy of the `String::contains` method is made for every distinct pattern type. So compiler knows which address to dispatch to. This process is called **Monomorphization**, and it's part of the reason generic Rust code usually performs as well as non-generic code.

![static-dispatch](./images/static-dispatch.png)

Note that **Monomorphization** also comes at a cost like slower compile time, larger program size and less effective for CPU's instruction cache. We can leaves only the type-dependent code for the compiler to copy for us while allowing the helper function to be shared.

```rs
impl String {
    // dynamic dispatch
    pub fn contains(&self, p: &dyn Pattern) -> bool {
        p.is_contained_in(&*self);
    }
}
```

Our program doesn't know which address to jump to in order to call the trait method `is_contained_in` on the given pattern. The caller must give callee both the `address of the pattern` and the `virtual method table`.

![dynamic-dispatch](./images/dynamic-dispatch.png)

A combination of a type that implements a trait and its vtable is known as a **trait object**.

ref: 

- [Rust for Rustaceans](https://www.amazon.com/Rust-Rustaceans-Programming-Experienced-Developers-ebook/dp/B0957SWKBS) - Chapter 2: Types, Dynamically Sized Types and Wide Pointers

```rs
trait Dig {
    fn dig_dig(&self);
}

struct A {
    normal: u32,
}

impl Dig for A {
    fn dig_dig(&self) {
        println!("A dig dig {}", self.normal);
    }
}

struct B {
    normal: u128,
}

impl Dig for B {
    fn dig_dig(&self) {
        println!("B dig dig {}", self.normal);
    }
}

// dynamic_dispatch don't need to be compiled to different instructions
// since d is not a generic type
fn dynamic_dispatch(d: &dyn Dig) {
    // 8 bytes pointer to the instance that implements the Dig Trait
    // 8 bytes pointer to the virtual method table
    println!("{}", mem::size_of_val(&d)); // always 16 bytes
}

// static_dispatch will be compiled to different instructions
// one for Struct A, and another one for Struct B
// ```
// // for Struct A
// fn static_dispatch(d: A) {
//     println!("{}", mem::size_of_val(&d));
// }
//
// // for Struct A
// fn static_dispatch(d: B) {
//     println!("{}", mem::size_of_val(&d));
// }
// ```
fn static_dispatch(d: impl Dig) {
    println!("{}", mem::size_of_val(&d));
}

fn main() {
    let a = A { normal: 0 };
    let b = B { normal: 1 };
    dynamic_dispatch(&a);
    dynamic_dispatch(&b);
    static_dispatch(a);
    static_dispatch(b);
}

```

### Ergonomic Trait Implementations

Rust does not automatically implement traits for references to types that implement traits. So, providing the blanket implementations are appropriate when defining a new trait.

```rust
trait Trait {}

fn foo<T: Trait>(t: T) {}

struct Bar;

impl Trait for Bar {}
impl Trait for &Bar {}
impl Trait for &mut Bar {}
impl Trait for Box<Bar> {}

fn main() {
    let bar1 = Bar {};
    foo(bar1);

    let bar2 = Bar {};
    foo(&bar2);

    let mut bar3 = Bar {};
    foo(&mut bar3);

    let bar4 = Box::new(Bar {});
    foo(bar4);
}
```