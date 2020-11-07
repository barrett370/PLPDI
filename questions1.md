# Questions

- How to automate conversion from AST to ASG.
- How do you deal with recursive type definitions in type inference?
  - In rust I found you had to use `Boxes` which are smart pointers to the heap. When the box goes out of scope the box is destructed. They allow rust to accept that a recursive type won't have infinite size
