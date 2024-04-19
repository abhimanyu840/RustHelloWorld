# References and Borrowing

-   We use `&` to borrow a reference to a value.
-   We can have multiple references to the same value.
-   References must always be valid.
-   References are immutable by default.

```
fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s);
    println!("The size of `{}` is {}", s, len);
}

//Function to calculate length

fn calculate_length(s: &String) -> usize {
    let len = s.len();
    len
}
```

-   We can make a reference mutable by using `&mut`.

```
fn main() {
    //Mutable References
    let mut s = String::from("Hello");
    change(&mut s);

    println!("{}", s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

-   We can have only one mutable reference to a value at a time.

```
fn main(){
    //Multiple mutable references
    let mut s = String::from("Hello");
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

> We cannot do this. Rather we can do this.

```
fn main() {
   //Multiple mutable references
   let mut s = String::from("Hello");
   {
       let r1 = &mut s;
       r1.push_str(" World");

   }
   let r2 = &mut s;
   println!("{}", r2);
}
```

-   We can have multiple immutable references to a value at the same time.
-   We can have immutable and mutable references in the same scope.

```
fn main() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}
```

```

fn main() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;

    let r3 = &mut s;
    println!("{} and {}", r1, r2);
    println!("{}", r3);
}
```

> Here above code will throw error because it is getting confused about which reference is mutable and which is immutable.

### Dangling References

> A reference to a value that is no longer valid.

```
// Dangling References
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

> This code will not compile because the return value of dangle() is a reference to the String s that dangle() no longer owns. The reference will never be valid.

> Instead of returning a reference to the String, we could return the String itself.

```
fn main() {
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```
