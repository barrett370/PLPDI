# PLPDI Compilers: Assignment 2

## Question A

```ocaml
func f x = x + x
func twice f x = f(f(x))
twice f 1
```

1. Draw the AST

**Note: Variable (named) root nodes represent parameters**

![`f` and `twice`](diag1.png)
\pagebreak

![`twice f 1`](diag2.png)
\pagebreak

2. Draw the ASG

I created the ASG of this program using the SPARTAN online ASG-based abstraction machine.

The following _SPARTAN_ statement:

>```SPARTAN
>bind f = LAMBDA(;x. PLUS(x,x)) in
>bind twice = LAMBDA(; f. LAMBDA(; x. APP(f,APP(f,x))))
>in
>APP(APP(twice,f),1)
>```

Created the following ASG:

![Initial ASG](ASG1.png){ height=40% }

3. Perform type inference on this program

![Typed ASG](typedasg.jpg)
\pagebreak
4. Draw the intermediate ASGs in the evaluation of this program

**Note Diagrams read from left to right on the page**

![Initial Stage](ASG1.png)
\pagebreak
![Evaluation Stage 1](ASGE1.png)
\pagebreak
![Evaluation Stage 2](ASGE2.png)
\pagebreak
![Evaluation Stage 3](ASGE3.png)
\pagebreak
![Evaluation Stage 4](ASGE4.png)
\pagebreak
![Evaluation Stage 5](ASGE5.png)
\pagebreak
![Evaluation Stage 6](ASGE6.png)
\pagebreak
![Evaluation Stage 7](ASGE7.png)
\pagebreak
![Evaluation Stage 8](ASGE8.png)
\pagebreak
![Evaluation Stage 9](ASGE9.png)
\pagebreak

## Question B

```OCaml
var z = ref(func x y -> x + y)
z := (!z)(1)
(!z)(2)
```

1. Draw the AST of this program

![AST](./b1.jpg)
\pagebreak

2. Draw the ASG of this program

This ASG was created using the SPARTAN visualiser with the code:

>```SPARTAN
>new z = LAMBDA(;x. LAMBDA(;y.  PLUS(x,y))) in
>bind a = ASSIGN(z,APP(DEREF(z),1)) in
>bind b = APP(DEREF(z),2) in
>SEC(a;b)
>```

![Initial ASG](BASG1.png)
\pagebreak

3. Draw the intermediate ASGs in the evaluation of this program

**Note Diagrams read from left to right on the page**

![Initial Stage](./BASG1.png)
\pagebreak
![Evaluation Stage 2](./BASGE2.png)
\pagebreak
![Evaluation Stage 3](./BASGE3.png)
\pagebreak
![Evaluation Stage 4](./BASGE4.png)
\pagebreak
![Evaluation Stage 5](./BASGE5.png)
\pagebreak
![Evaluation Stage 6](./BASGE6.png)
\pagebreak
![Evaluation Stage 7](./BASGE7.png)
\pagebreak
![Evaluation Stage 8](./BASGE8.png)
\pagebreak

4. Perform type inference on this program

![Type inference](./b4.jpg)
\pagebreak

### Static Vs Dynamic Typing

In statically typed languages, such as C or Rust, one is required to specify the type of any variable, function parameter or return argument explicitly.
In dynamically typed languages, such as Python, a variable or method can be said to have `Any` type. This allows for less verbose, more flexible code. Dynamic typing in and of itself does not lead to runtime errors, instead it makes it much easier for a programmer to accidentally cause them.

For example, in python you could define a function:

```python
f = lambda x,y: x**y
```

And call it with:
```python
f(2,"foo")
```
leading to a runtime error

Whereas, in Rust, a dynamically typed language (that does not attempt to cast explicitly defined variables), The compilation fails as it expects an `i32` not a `str`

```rust
fn main() {

    let f = |x:i32,y:i32| { x^y };

    f(1,2); // Ok
    f(2,"foo"); // Compilation Error

}
```

In the context of this example, we are assuming the type of `+`. If instead the operator had the type `str -> str -> str` the program would fail at runtime as we attempted to pass integers. If the compiler knew the type of `+`, i.e. in our graph `+` was already labelled `str -> str -> str` then our type inference for the rest of the program would fail, leading to a compilation error rather than a runtime error.
