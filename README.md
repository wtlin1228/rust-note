# Rust Note

- **Command-Line Rust** by Ken Youens-Clark: https://www.oreilly.com/library/view/command-line-rust/9781098109424/ 
- **Crust of Rust** by Jon Gjengset: https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa

## `str` vs `&str` vs `String`

- `str -> [char]`, `str` is a sequence of chars, it doesn't know the size.
- `&str -> &[char]`, `&str` is a reference to a sequence of chars, so it knows both the start and the size of that char sequence.
- `String -> Vec<char>`, `String` is heap allocated where `&str` can be pointed to something on the stack or the heap.

If I have a `String`, going to a `&str` is trivial, it's cheap(AsRef).

Going the other way is harder, `&str -> String`, it's expensive(memcpy).

ref: https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2
