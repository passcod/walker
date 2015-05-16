# Walker

_[Unstable]? Psshhh, imma give you that thing._

## Install

With Cargo:

```toml
[dependencies]
walker = "^1.0.0"
```

## Usage

```rust
use walker::walk;

extern crate walker;

match walk(path) {
  Err(e) => panic!(),
  Ok(iter) => {
    for entry in iter {
      println!("{}", entry);
    }
  }
}
```

## Stability

This library started out immediately at 1.0, so any breaking change will bump
the major and you can thus be sure that it won't break your code.

This library will track Rust stable. I don't have the free time to track
nightly, but you're welcome to contribute!

## Etc

Licensed under [Apache 2.0] or [MIT], at your option.

Code was heavily inspired from [the version in the Rust source code][orig]
(there's not too many ways to write a directory recursor, you know).

The `#[test]` was copied directly from the afore-mentioned source, and is
therefore [Copyright © The Rust Project Developers][©rust].

Written by [Félix Saparelli](https://passcod.name) and [contributors].

[Apache 2.0]: http://www.apache.org/licenses/LICENSE-2.0
[MIT]: http://passcod.mit-license.org
[Unstable]: http://doc.rust-lang.org/stable/std/fs/fn.walk_dir.html
[contributors]: https://github.com/passcod/walker/graphs/contributors
[orig]: http://doc.rust-lang.org/stable/src/std/fs.rs.html#1010-1054
[©rust]: http://rust-lang.org/COPYRIGHT
