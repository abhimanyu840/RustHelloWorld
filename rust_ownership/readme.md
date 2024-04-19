# Ownership in Rust

> Ownership is a set of rules that govern how a Rust program manages memory.

-   Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory.

-   Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.
    > None of the features of ownership will slow down your program while it’s running.

## Ownerships Purpose in Rust

-   Ensuring memory safety at compile time
-   Ensuring thread safety at runtime

## Ownership Rules

-   Each value in Rust has a variable that’s called its owner.

    > There can only be one owner at a time.

-   When the owner goes out of scope, the value will be dropped.

-   The ownership of a value is passed from one variable to the next when the
-   > First, the variable that owns some value is assigned a new value.

-   > second, the variable that owns the value is assigned a new value.

-   The ownership of a value will always be valid.

## Variable Scope

```
fn main() {
    println!("Ownership in Rust");
    //s is not valid here, it's not yet declared
    let s = "hello"; //s is valid from this point forward
    //do stuff with s


} //this scope is now over, and s is no longer valid
```

## The String Type

```
fn main() {
    let mut s = String::from("hello");
    s.push_str(", World!");

    println!("{}", s)
}
```

## Memory and Allocation

-   String literals are stored in the read-only memory of the program.

-   String types are stored on the heap. The heap is less organized: when you put data on the heap, you request a certain amount of space.

    > The memory must be requested from the memory allocator at runtime.
    > When you're finished with the data, you return the memory to the allocator.
    > This process is called _allocating on the heap_ and is similar to _freeing_ memory on the heap.

-   In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that is no longer being used.
-   In the languages without a GC, you are responsible for freeing memory when it is no longer needed.

    > There are some problems with these.

    -   When you make a program that has a lot of memory allocations, it's going to take a long time to run.
    -   If we forgot to free the memory, we'll have a memory leak.
    -   If we do it too early we will have an invalid variable.
    -   If we do it twice we will have a double free. This is a bug in the program.

    > Rust takes a different approach: it won't automatically free the memory for us.

    > Instead, the memory is returned when the variable that owns it goes out of scope.
    > This function is called _drop_.

## The move trait

```
    let s1 = String::from("hello");
    let s2 = s1;
```

> The ownership of s1 is moved to s2.

```
    println!("s1: {}", s1);
```

> This will cause an error.

## The clone trait

```
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);
```

> This will not cause an error.

## The copy trait

```
    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);
```

> This will not cause an error.
> The reason is that the type of x is an integer, which is a _copy_ type.
> It only works on stack data because their size is known at compile time.

## Ownership and Functions

-   **The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.**

```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

## Return Value and Scopes

-   **Returning values can also transfer ownership.**

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

```
fn main() {
    let s = String::from("Hello");
    let (s2, len) = calculate_length(s);
    println!("The size of `{}` is {}", s2, len);
}

//Function to calculate length

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}
```
