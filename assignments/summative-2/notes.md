# Misc Assignment Scratches

1.

    ```SPARTAN
    bind f = LAMBDA(;x. PLUS(x,x)) in
    bind twice = LAMBDA(; f. LAMBDA(; x. APP(f,APP(f,x))))
    in
    APP(APP(twice,f),1)
    ```

2.

    ```SPARTAN
    new z = LAMBDA(; x. LAMBDA(; y. PLUS(x,y))) in 
    SEC(ASSIGN(z, APP(DEREF(z),1)) ;
    APP(DEREF(z),2))
    ```
alternatively 

``` 
new z = LAMBDA(;x. LAMBDA(;y.  PLUS(x,y))) in 
bind a = ASSIGN(z,APP(DEREF(z),1)) in 
bind b = APP(DEREF(z),2) in
SEC(a;b)
```
