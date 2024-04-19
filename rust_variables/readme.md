# Variables in Rust

### We can use `let` keyword to define a variable in rust

```
    let x = 5;
    println!("The value of x is: {}", x);
```

### Variables are immutable in rust.

> We cannot do this

```
    x = 6;
    println!("The new value of x is: {}", x);
```

> But we can make it mutable by adding a `mut` keyword

```
    let mut x = 5;
    println!("The value of x is: {}", x);
```

## Shadowing of variables in rust

> We can redeclare a variable in rust i.e.

```
    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6;
    println!("The new value of x is: {}", x);
```

> We can also use previous variable at the time of shadowing

```
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The new value of x is: {}", x);
```

> Also we can change data types of it while we redeclare it

```
    let x = 5;
    println!("The value of x is: {}", x);

    let x = "Hello World";
    println!("Now the x is: {}", x);
```

> But we cannot mutate it by changing its data types i.e. This is not allowed

```
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = "Hello World";
```

## Constants

> When defining a constant we have to give its type like `i32` or `u32`

```
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum point is: {}", MAX_POINTS);
```

-   Constants needs to be assigned at the time of declaration
-   We can't use shadowing in const
