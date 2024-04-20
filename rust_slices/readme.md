# Slice

-   Slices are references to contiguous sequence of elements in a collection rather than the collection itself.
-   Slices are immutable.
-   Slices are not references to the original collection.

> Syntax of slice is `&[T]` where `&` is reference to the collection and `T` is type of element in the collection

-   Slicing of array

```
fn main() {
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    let slice: &[char] = &arr[1..3];

    println!("sliced value is {:?}", slice);
}
```

-   Slicing of vector of char

```
    let vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let slicedvec: &[char] = &vec[1..5];

    println!("sliced value is {:?}", slicedvec);
```

-   Slicing of String

```
    let str = String::from("Abhimanyu");
    let slicedstr: &str = &str[..];

    println!("Sliced string is {:?}", slicedstr)
```

-   Slicing of array of String

```
    let str_array: [&str; 5] = ["Abhimanyu", "Kumar", "Singh", "Another", "String"];

    let slice_str_array: &[&str] = &str_array[1..=3];

    println!("Sliced Array of str is {:?}", slice_str_array);
```

## Getting first word of sentence(Without using Slice)

```
fn main() {
    let sentence = String::from("This is a sentence ");

    let returned = first_word(&sentence);

    println!("{:?}",returned)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
```

## Getting first word of sentence(Using Slice)

```
fn main() {
    let sentence = String::from("This is a sentence ");

    let returned = first_word(&sentence);

    println!("{:?}", returned)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```
