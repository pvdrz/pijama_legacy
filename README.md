# Pijama

Pijama is a general purpose programming language with the following features:

- Completely functional: no mutation, no variables, no side effects, no state
  at all.
- Statically typed: Every term must have a defined type before execution.
- Type inference: Types are inferred when possible. But you can type your terms
  to be sure your code does what you want.

## Why?

I wrote this language as an small playground to test some of the constructs
explained in the "Types and Programming Languages" book. I also think that
there is a gap in the current PL ecosystem: I've been looking for a functional,
statically typed language that's easy to write and even though Haskell, OCaml
and Racket are fun, I didn't feel comfortable with their syntax. I don't intend
for others to take this language seriously, but I'm happy to discuss its design
and implementation.

## How to use

Clone the repository and use cargo to run your programs

```bash
cargo run path_to_your_code.pj
```

## Syntax

Pijama's syntax is heavily inspired by Elixir, Python, Ruby, and Rust. Blocks
of code are written using `do/end` and the types of terms are written using
`:`. As an example, here you have the most important function in all the CS
field:

```elixir
fn rec fact(n: Int): Int do
    if n <= 0 do
        1
    else
        n * fact(n - 1)
    end
end

fact(20)
```

Functions are first-class citizens in Pijama. You can write higher order
functions or define new functions by partially evaluating other functions

```elixir
fn min(cmp: Int -> Int -> Bool, x: Int, y: Int) do
    if cmp(x, y) do x else y end
end

less_than = fn(x: Int, y: Int) do x < y end

is_negative = less_than(0)

x = min(less_than, 5, -2)

is_negative(x)
```

## Compiling and evaluation

Pijama is an interpreted language, i.e., your program is evaluated instead of
generating an executable. However, it suffers several compilations before being
evaluated. Source code is compiled to an untyped lambda calculus with some
extensions (fix-point operator, conditionals, bit-based integer arithmetic,
etc) and this representation is executed in an stateless manner.  Right now
there is no garbage collection and evaluation is far from efficient, there is a
lot of duplicated values as we are doing call-by-name evaluation. If you happen
to find a memory leak, let me know.

## Next steps

These are some of the features I'd like to implement in the future:

- Algebraic data types: Having something like records and enumerations would
  allow to write a lot of data structures and make the language easier to use.

- Parametric polymorphism: Generics are a must to avoid repeating yourself.

- Ad-hoc polymorphism:  Something like traits or typeclasses to extend
  behavior.

- Moar type inference: The types of a function's arguments must be specified
  always. This should be optional and be there for documentation purposes only.

- Call-by-need evaluation: Pijama is evaluated by cloning a lot of values in
  the process, I'd be better if some computations could be recycled like
  Haskell does. This might require having a GC in the future.

- Tail-call optimization: Pijama doesn't have its own stack right now. Which
  mean that recursion can blow up without much effort. After implementing a
  stack it would be great to have tail-call optimization.

- List comprehensions: Because recursion might be too verbose for some
  operations. Here I'm more inclined to take the syntax from Python or Elixir
  rather than from Haskell.

- Standard library: If all the other steps are done. It's time to let this
  language interact with the world. Some IO and collections could be a nice to
  have.

## Non-next steps

- OOP: Classes and inheritance aren't in the goals of this language. Scala
  already manages this nicely.

- References: This adds a lot of complexity for the sake of performance. Rust
  and Go already take care of that.

## Contributing

If you want to get a ticket for this roller-coaster check
[CONTRIBUTING.md](https://github.com/christianpoveda/pijama/blob/master/CONTRIBUTING.md)
for ways to contribute to Pijama and feel free to open an issue, do a pull
request or drop an email. We'll be happy to hear from you.
