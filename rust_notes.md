
### Chapter 1 and 2
* println!("adasd") . Methods with '!' means we are calling a macro in Rust.
* In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.
* The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
* The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that, like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable. (Chapter 4 will explain references more thoroughly.)
* Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own.
*  Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers. The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.
* When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.
* Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.6 of the rand crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the Cargo.lock file the first time you run cargo build, so we now have this in the guessing_game directory.When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.
* When you do want to update a crate, Cargo provides the command update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file. In this case, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0.
* Rust has a strong, static type system. However, it also has type inference. When we wrote let mut guess = String::new(), Rust was able to infer that guess should be a String and didn’t make us write the type. The secret_number, on the other hand, is a number type. A few of Rust’s number types can have a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others. Unless otherwise specified, Rust defaults to an i32, which is the type of secret_number unless you add type information elsewhere that would cause Rust to infer a different numerical type. The reason for the error is that Rust cannot compare a string and a number type
* We create a variable named guess. But wait, doesn’t the program already have a variable named guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example. We’ll cover this in more detail in Chapter 3, but for now, know that this feature is often used when you want to convert a value from one type to another type. See `guess` in the example below:
 ```

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```
* Additionally, the u32 annotation in this example program and the comparison with secret_number means Rust will infer that secret_number should be a u32 as well. So now the comparison will be between two values of the same type!
* The underscore, `_`, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them. 
```aiignore
 let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```

### Chapter 3 - Common Programming Concepts
#### Variables and Mutability
*  By default, variables are immutable. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers.
* The following prog, gives you compilation error since `x` is an immutable variable. You received the error message cannot assign twice to immutable variable `x` because you tried to assign a second value to the immutable x variable. _The Rust compiler guarantees that when you state that a value won’t change, it really won’t change, so you don’t have to keep track of it yourself._
* in order to make `x` mutable, you can make it `let mut x = 5;`
```aiignore
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

```
* Rust has constants as well which are always immutable and need to be set to a constant expression only, not something that can change value at runtime. Also, const needs to be annotated with the type always.
```aiignore
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
* _Shadowing_ : This is used to declare a new variable with the same name.(This is similar to conceptually to RDDs that we see in Spark). Example - 
```aiignore
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```
* Shadowing is different than `mut` - with shadowing we can change the type of the variable. Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name.
```aiignore
let spaces = "   ";
let spaces = spaces.len();
```
#### Data types
* Rust is a statically typed language, which means that it must know the types of all variables at compile time.
* Rus has two subsets of data types - scalar and compound data types.
* When compiling Rust code in debug mode, it checks for integer overflows , but in release it wont and just wrap the value which can result in unexpected behaviour.
* Rust has two floating point types : `f32` and `f64` . Default is f64 which is capable of more precision.
* Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. 
* Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
* A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
  ```aiignore
    fn main() {
       let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
  ```
* we can use pattern matching/destructuring or index based  retrieval to access the contents of a tuple. Example :
```aiignore
   let tup = (1, 2.0, 500);

    println!("Value of first index {}", tup.0);

    let (x,y,z) = tup;

    println!("Value of 2nds index {y}");
```
* Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length. The data is allocated in stack instead of heap. if you need  the collection to grow or shrink, you need vector type which is part of the standard library. Indexes start with `0`. Some examples of array:
```aiignore
let a = [1,3,4,5];
let x: [i32; 5] = [1,2,3,4,5]; // with explicity type annotation
```


#### Functions
* Functions in Rust start with the keyword `fn`.
* In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean.
* There is a distinction between statement and expression in Rust language unlike other languages.
```aiignore
statements - they are instructions that perform an action and do not return any value. Eg - declaring a variable, function etc.
expressoing - evaluate to a resultant value. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
 For example:
     fn main() {
        let y = {
            let x = 3;
            x + 1
        };
    
        println!("The value of y is: {y}");
    }
```
* You must indicate the return type of the function with an `->` if the func returns a value.Note there is no `;` in the following function. Adding a `;` would throw an error since it would become a statement instead of an expression. Eg :
```aiignore
fn plus_one(x: int32) -> i32 {
   x+1
}
```
#### Control Flow
* `if` and `loops` are the common constructs to control the flow of you code execution in Rust.
* Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions. Example : 
```aiignore
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```
* Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
```aiignore
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
```
* There are 3 kinds of loops in Rust - loop, while and for.
* loop lets you execute a block of code forever, until you explicitly tell it to stop. You can place break in the block if you want to break out depending on some condition.
```aiignore
fn main() {
    loop {
        println!("again!");
    }
}
```
* You can return values from a loop. Ex : 
```aiignore
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```
* Loop labels allow to disambiguate within nested loops. The label starts with a single quote. It can be used with break or continue to specific which loop this applies to. By default, it would apply to the innermost loop at that point. See Example : 
```aiignore
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```
* Example to iterate over a collection:
```aiignore
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
For Range iteration. rev() reverses the range
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

### Ownership
* Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works.
* Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.
* You must know whether the value is on the heap or the stack to understand the ownership rules further. The stack is well organized and used to store values of known, fixed size - always follows FIFO. Heap is less organized and you can ask the memory allocator to find a space in heap big enough for your needs and it returns a pointer to that location. This pointer is a known & fixed size and pointer itself could be stored in a stack. but the actual data would be on the heap.
* Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation. Read full details [here](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)
* Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
* Ownership rules
```aiignore
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
```