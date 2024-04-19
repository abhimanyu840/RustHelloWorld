# Control Flow In Rust

## `if` Expression

```
    let number = 3;
    if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is smaller than 5");
    }
```

### Handling Multiple Conditions with `else if`

```
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
```

### Using `if` in a `let`statement

```
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}")
```

### Nested `if`

```
    let num = 15;
    if num % 2 == 0 {
        println!("{num} is even");
    } else {
        println!("{num} is odd");

        if num > 10 {
            println!("{num} is also grater than 10");
        } else {
            println!("{num} is not grater than 10");
        }
    }
```

## `&&` and `||`

```
    let a = 110;
    let b = 15;
    let c = 22;

    if a > b && a > c {
        println!("{a} is greater than {b} and {a} is also greater than {c}");
    } else {
        println!("Condition with && not met")
    }
```

```
    let a = 16;
    let b = 15;
    let c = 22;

    if a > b && a > c {
        println!("{a} is greater than {b} and {a} is also greater than {c}");
    } else if a > b || a > c {
        println!("Either {a} is greater than {b} or {a} is greater than {c}");
    } else {
        println!("Condition with && not met");
    }
```

## Loops

> In rust we can loop using `loop` keyword.

```
    loop {
        print!("Hello World!");
    }
```

-   Above example will go in infinite loop.

### Return values from `loop`

```
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the value of result is {result}");
```

### While loop

```
    let mut counter = 3;

    while counter != 0 {
        println!("{counter}!");
        counter -= 1;
        //Wait for 1 sec
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("cutoff!")
```

### For Constructs

```
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {element}");
    }
```

1. **For with characters**

```
    let s = "Hello World";
    for c in s.chars() {
        println!("{c}");
    }
```

2. **For with Range**

```
    for num in 1..4 {
        println!("The value of num is {num}")
    }
```

## The FizzBuzz Problem

-   #### Print integers 1-to-N, but print “Fizz” if an integer is divisible by three, “Buzz” if an integer is divisible by five, and “FizzBuzz” if an integer is divisible by both three and five.

```
    let n = 100;
    for num in 1..=n {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{num}");
        }
    }

```
