# PLPDI Compilers: Assignment 2

## Question A

```ocaml
func f x = x + x
func twice f x = f(f(x))
twice f 1
```

1. Draw the AST

![`f` and `twice`](diag1.png)
\pagebreak

![`twice f 1`](diag2.png)
\pagebreak

2. Draw the ASG

I created the ASG of this program using the online ASG-based abstraction machine. 

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
