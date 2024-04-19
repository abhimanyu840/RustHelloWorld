# Functions in rust

> A function is a set of statements that perform a specific task.

> In Rust we use `fn` keyword for defining a function. After that we write function name in **Snake Case**.

```
fn main() {
    println!("Functions in Rust");
    another_function();
}

fn another_function() {
    println!("This is another function!")
}
```

## Parameters and Arguments

> Parameters are specified after the function name, in parentheses.

```
fn main() {
    println!("Functions in Rust");
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x)
}
```

> An argument is the concrete value of the parameter.

```
fn main() {
    println!("Functions in Rust");
    another_function(5);
    another_function(10);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

### Multiple Parameters

```
fn another_function(x: i32, y: char) {
println!("The value of x is: {}", x);
println!("The value of y is: {}", y);
}

fn main() {
println!("Functions in Rust");
another_function(5, 'a');
}

```

## Statements and Expressions

> Statements are instructions that perform some action and do not return a value.

```
    let x = 5; //Statement
```

> Expressions evaluate to a resulting value.

```
    let y = {
        let x = 3;
        x + 1 //Expression
    };
```

> In rust functions are expressions because they evaluate to a value.

## Return Values

> We can return a value in function by using `-> <return type>`. For e.g.

```
fn add(a: i32, b: i32) -> i32 {
    a + b;
}

fn main() {
    println!("{}", add(22, 44));
}
```

> We can also return a tuple from a function

```
fn sum_diff(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

fn main() {
    let x = sum_diff(22, 44);
    println!("The sum is {}, and the diff is {}", x.0, x.1);
    println!("The sum and diff is {:?}", x)
}
```

## Early Return

> We use `return` keyword to return early from a function.

```
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```
