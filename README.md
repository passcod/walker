# Walker

__With the introduction of the fabulous @BurntSushi's [Walkdir],
this library will soon be deprecated. This is an early warning to
give you time to switch over with ease.__

Upon deprecation, Walker will implement a thin wrapper around Walkdir,
keeping the same interface as before as much as possible, and bumping
only the patch version number.

After a reasonable amount of time has passed, and downloads of this
library have reduced to nil or nearly there, it will be _yanked_ from
Crates.io as a way to enforce depreciation. At this point and thereafter,
if [someone wants the name I'm open to transfer ownership of the crate][1].

[walkdir]: https://crates.io/crates/walkdir
[1]: https://www.reddit.com/r/rust/comments/3roe70/advance_notice_of_deprecation_for_walker/cwq0ovn

---

_[Unstable]? Psshhh, imma give you that thing._

## Install

With Cargo:

```toml
[dependencies]
walker = "^1.0.0"
```

## Usage

```rust
use walker::Walker;

extern crate walker;

match Walker::new(path) {
  Err(e) => panic!(),
  Ok(iter) => {
    for entry in iter {
      println!("{}", entry);
    }
  }
}
```

You can also use `.rewind()` on a Walker instance to reset the iterator.

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
