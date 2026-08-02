# Wisp

`wisp` is a small Lisp-family language, strongly inspired by Scheme. It was created solely to learn two things: Rust and Lisp. It is pre-alpha software with a long way to go, so don't use it for anything serious, or in fact, non-serious.

## Usage

Just `cargo run` in the root to start the REPL, or `cargo run [file1, file2, ...]` to run prewritten wisp.

## Features

- Builtins are first-class values
- Lexical closures
- Prelude, written in Lisp
- Recoverable errors
- No dependencies
- Scheme-style truthy values (only `#f` is false)

## Examples

- `examples/fib.scm`: Prints the first 20 fibonacci numbers.

## Limitations (the wishlist)

Some of these may be out of scope, but who knows?

- No TCO
- Stack overflow aborts the process
- Memory grows with calls and is not freed (keeps the design simpler)
- Lots of missing builtins and special forms
- No strings, characters, vectors or maps
- No comments
- Every number is float, no 'numerical tower'
- REPL only, no file reading

## Sample

```scheme
> (+ 1 2 3)
6

> (< 1 2)
#t

> (equal? '(1 2) '(1 2))
#t

> (car '(a b c))
'a

> (car (cdr '(1 2 3)))
2

> (define double (lambda (x) + x x)))
> (double 21)
42

> (define adder (lambda (n)(lambda (x) (+ x n))))
> (define add10 (adder 10))
> (add10 5)
15

> (define sum (lambda (xs)(if (null? xs) 0 (+ (car xs)(sum (cdr xs))))))
> (sum '(10 20 30))
60

> (map (lambda (n) (+ n 1)) (1 2 3))
(2 . (3 . (4 . ())))

> (filter (lambda (n) (< n 3) '(1 2 3 4))
(1 . (2 . ()))

> (fold-left + 0 '(1 2 3 4 5))
15

> (fold-left (lambda (a b)(cons b a)) '() '(1 2 3))
(3 . (2 . (1 . ())))
```
