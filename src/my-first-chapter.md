# My First Chapter

Fill out your content here.

```rust
fn main() {}
```


```rust,editable
fn main() {
    let number = 5;
    print!("{}", number);
}
```


\\( \int x dx = \frac{x^2}{2} + C \\)

\\[ \mu = \frac{1}{N} \sum_{i=0} x_i \\]


```rust,noplayground
let mut name = String::new();
std::io::stdin().read_line(&mut name).expect("failed to read line");
println!("Hello {}!", name);
```

```rust,ignore
# This example won't be tested.
panic!("oops!");
```

Here is a component:
```rust,no_run,noplayground
{{#include test.rs:component}}
```

Here is a system:
```rust,no_run,noplayground
{{#include test.rs:system}}
```

This is the full file.
```rust,no_run,noplayground
{{#include test.rs:all}}
```
### A heading

Some text.

#### A smaller heading

More text.

* milk
* eggs
* butter

1. carrots
1. celery
1. radishes


Use [mdBook](https://github.com/rust-lang/mdBook).
Read about [mdBook](README.md).
A bare url: <https://www.rust-lang.org>.


This is an example of a footnote[^note].


![The Rust Logo](images/smart1.png)

| Header1 | Header2 |
|---------|---------|
| abc     | def     |

- [x] Complete task
- [ ] Incomplete task


[^note]: This text is the contents of the footnote, which will be rendered
towards the bottom.






