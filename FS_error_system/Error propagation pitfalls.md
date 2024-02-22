# Definition of Error Propagation for Rust Beginners

The definition is simple: a program does something, it fails, or something goes wrong, and it should give you as a programmer a hint about what happened. This looks simple, but in my journey, it turned out to be an obstacle course littered with mines.

The current error propagation has turned out to be an exercise in finding one's way in a labyrinth rather than simply reading a file if okay, parsing the file. The main issue turned out to be the Result return. Since I'm a C/C++ engineer, or something like that, I was baffled by the idea that there is a type that has 2 Enums, and they are returned. Now already, the problem begins. In C++, an Enum is a number basically, either assigned by the compiler or given by the user, but in Rust, that is a Struct, a class that is almighty and confusing.

The first problem: what happens with the error if there is none? So everything went well (for example, read file). A normal result is returned, but to be complete or exhaustive, if I check a Result return, it has Ok and Err, I need to check both. It took me literally half an hour to find what it returns in the case. It returns a unit struct, or in translation, a struct with no data, that is resolved and understood by the compiler. Just to let you know, the Rust compiler is a weird magic thing that resolves all sorts of issues.

But why does one make a struct out of an Enum? Enum should be simple, easy, a number connected to a label in the code, done. So to sort out the enum that is a struct, and so-called unit struct in cases that one needs all the time, one needs pretty good knowledge of what the compiler will do. Otherwise, it is difficult to figure out what the compiler does automagically and what logic has to be done by the user.

Wait, so there is an Ok and an Err, both Enums, but they accept structs and can return them. The following code, from the Rust book, shows how the Result type is envisioned: a Struct with two possible Enums. However, the Enums are generics, these are templates in C++, that will accept various types.
```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Okay, I understood the **Result** type as a return type. But I want to read and parse a file. Let's take a look at reading and parsing, so 2 different types of errors are possible. I think I need something like generics, akin to templates in C++? Ah, that makes sense; the Enums in the **Result** type are generics, and through them, I just send the types back. Okay, they don't have a simple way to propagate them out, but it still works. This seems simple enough (boy, am I wrong), more on that later. The main example is in this repository, and the situation is that we have a single YAML file that we want to read. I am going to formalize and summarize my research or learning on these subjects, and yes, subjects. The previous discussion was just a taste of what is happening in the code in-depth and what needs to be done.

Just to dissect types of errors that the program will encounter. The first is the file I/O. For example, if the file does not exist, the drive is not accessible, or the operating system doesn't let us access the directory, or some other problem arises. The other issue can be that the YAML file is parsed by the serde crate (Serde -> short for serialization deserialization) while reading/parsing the YAML file. For example, the format doesn't fit, or there are values that don't comply, or any other problems. The issue is that we have to then propagate the error for the user to see or the system to automatically break off and notify the user.

How to handle them. This code represents the example of which types can be returned. As we can see, there are 2 types of errors.
```Rust
Result<YamlData, serde_yaml::Error>
Result<File, io::Error>
```

#### Option one
To avoid returning 2 types of error, through the result, we can split the function into 2 functions.
The following code show how this iis built in the code example that is in submited.

```Rust
fn open_file_1(filename: &str) -> Result<File, io::Error> {
    match File::open(filename) {
        Ok(file) => Ok(file),
        Err(err) => Err(err),
    }
}
```

```rust
fn read_yaml_file_fail_2(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    let yaml_data = match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => yaml_data,
        //Err(err) => Err(err),
        Err(err) =>
        {
            return Err(err);// Convert the error to the appropriate type, 
        }
    }
    
    Ok(yaml_data) // TODO learn OK return explanation 
}
``` 
Reviewingthis code I can personally say that I do not like this this should be solved in one function without any problems.


#### Option two

In the typical scenario in C++, handling exceptions on the spot or returning through templated functions would suffice. We open the file and process the YAML file or whichever file we have. Here we face a problem, the Oopen file function can fail, and the parse Yaml file can also fail. The challenge lies in returning the error out of this functions. We need to handle both types through a generic type. This generic type corresponds to the **Err** part of the **```Result<Ok, Err>```** type. But how do we return it? 
In Rust, there are some fundamental rules. The type's size should be known at compile time. Also there is the criteria of monomorphoisation, from the compiler book [link](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html) : 

> "Rust takes a different approach: it monomorphizes all generic types. This means that compiler stamps out a different copy of the code of a generic function for each concrete type needed."

However, that's just half of the story. Actually, the size should also be known. After some research, one way to return an unknown size type back to the caller function is through **```Box<dyn Error>```** in Rust. But wait, what is **```Box<dyn Error>```**? Weren't we just aiming to return a **Result** type? How did we get from **```Err<T>```** to **```Box<dyn Error>```**?

Well, welcome to reality. The **```Result<Ok, Err>```** returns either one or the other; they are generics, by the way, and the **Err** needs a type specification and size specification known at compile time. Since this is not the case here until runtime, we need to work around all of these problems. All of this introduces something called the trait object. We have to work with a **```Box<dyn Error>```** that is a trait object. Trait objects, which are references to types that implement a certain trait, do not have a size known at compile time. Therefore, they are unsized. To work with trait objects, Rust requires a reference (**&**) or a smart pointer like **Box** or **Arc** (I got this somewhere).

Here's how it works: The error type is accessed by the **std::Error**, and the **std::Error** is a trait. Meaning no data, no fields, which makes it an object without data, since the standard identifications from the **std::Error** can be used to access the data from the custom Error type. The **```Box<dyn Error>```** is a very tricky thing. The **Box** is a pointer that is pointing to the parts of the trait object. Trait objects are represented by a fat pointer. The fat pointer has two parts: the data pointer and the vtable pointer. The vtable is the table that has all the trait methods that a trait is implementing or inheriting/deriving. It is roughly equivalent to the pure abstract class full of virtual functions in C++. As it is visible, the data part is not known at compile time, and the vtable part is known. To solve this, the compiler recognizes the **dyn** keyword as dynamic dispatch and leaves the data part resolution to the runtime.

Here is the code that allows to handle different types of data through the Result return. 
```Rust
Result<YamlData, Box<dyn std::error::Error>>
```

The official documentation on this part is a bit deficient and unclear. So you might exactly as me strugle through the information gathering process and understanding things. In the next parts I will try to define the problems and the ideas behind the construction of syntax.

## Fat pointer diagram

![alt text](https://github.com/mbencik/Rust-learning/blob/main/FS_error_system/Images/Rust_pointers_explanation.jpg)

### Explanation of the basic pointer 
In Rust, both **```Box<dyn Error>```** and **```&mut dyn Error```** are used to handle errors in a similar manner, but they represent different ownership and borrowing semantics.

**```Box<dyn Error>```** is a boxed trait object that owns its underlying data.
It represents a heap-allocated object that implements the Error trait.
Since it's a box, it has a fixed size and lives on the heap.
**```Box<dyn Error>```** is typically used when you need to move an error across ownership boundaries or when you want to store an error with a dynamic type in a data structure.

```rust
// Example of creating a Box<dyn Error>
use std::error::Error;

fn produce_error() -> Box<dyn Error> {
    // Create an error and return it boxed
    Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
}
```

**```&mut dyn Error```** is a mutable reference to an object that implements the Error trait.
It does not own the underlying data; it borrows it mutably.
This reference is typically used when you want to pass around an error without transferring ownership, allowing the calling function to inspect or modify it.


```rust
// Example of processing a mutable reference to dyn Error
use std::error::Error;

fn process_error(error: &mut dyn Error) {
    // Modify the error if necessary
    error.source(); // Example usage of the Error trait method
}
```

Choosing Between **```Box<dyn Error>```** and **```&mut dyn Error```**:

Use **```Box<dyn Error>```** when you need to transfer ownership of the error, store it in a data structure, or return it from a function.
Use **```&mut dyn Error```** when you want to borrow the error mutably for inspection or modification without transferring ownership.

In summary, **```Box<dyn Error>```** is used for owning errors and transferring them across boundaries, while **```&mut dyn Error```** is used for borrowing errors for temporary access or modification. The choice depends on the ownership and borrowing semantics required by your code.

![alt text](https://github.com/mbencik/Rust-learning/blob/main/FS_error_system/Images/Rust_pointers_explanation.jpg)

### Explanation of the dyn keyword, dynamic dispatch, sized trait and static dispatch

In the Rust documentation on compile time known sizes I encountered that this is a pritty important concept in Rust. In the Rust documentation [link](https://doc.rust-lang.org/nightly/std/marker/trait.Sized.html) the sized is a trait marker. In one fo the sources it is said "sized is a compiler built-in trait", I dont know about that but it is a trait that does not have to be expllicitly derived and the compiler will still know the size of the object type at compile time. A type is considered sized if the precise size of a value of type is known and fixed at compile time once the real types of the type parameters are known (i.e. after completing monomorphisation).


|||||||<  We would need to identify which error type it is, cast the inner trait preferably to a struct (no idea if that is possible, probably trait should not be upgraded), which is probably impossible. That means either from outside reimplement the needed functions (to figure out parse the errors) to actually read the internal fields of the custom error. This makes the whole thing a bit top-heavy and full of overhead, and that is a bit of a problem.
The main issue of it all is that a simple procedure that should actually just give you the option to read something (file), propagate the error, and handle the error properly. One more thing is the error in the end created on the heap or is created on the stack (is so overcomplicated that it's almost unbelievable)? Go find out this so you know. This is, for example, for a beginner, very difficult to comprehend. Why does he need to resolve this many problems with just one tiny problem? Why does it take a full day or longer to resolve the problem of propagating and visualizing an error? He just wanted to open a file and propagate an error if it fails. A much better process would be if the error struct that is a trait extension extends its implementation so there is an option that we use the error rate and you have a bare-bone error trait like now and that stays as is, but there is an error class that has baseline implementations of the error trait. This can be used to identify what kind of error it is. That is why the '?' was introduced to hide the complexities of the implementation and match-making to resolve the errors. Which is great for beginners but bad for people that want more control and clear practices how to handle error propagation.
The bad part is that this problem would extend to any type in the Rust realm where we need generics, basically or interfaces, abstract features to send such common data out.


In this document the goal is to adress the error propagation, error handling, nonsense with the documentation and problems with the explanations.



First the 

https://joshleeb.com/posts/rust-traits-and-trait-objects/
https://www.reddit.com/r/rust/comments/vzq6pd/is_boxstderrerror_acceptable/
https://doc.rust-lang.org/book/ch10-02-traits.html
https://betterprogramming.pub/rust-basics-structs-methods-and-traits-bb4839cd57bd

```rust
use std::error::Error as StdError;


#[derive(Debug, Serialize, Deserialize)]
struct YamlData {
    sentences: Vec<String>,
}

//TODO Separate the file open and deserialization of yaml, othervise I have to solve the problem with 2 different errors
fn read_yaml_file(filename: &str) -> Result<YamlData, Box<dyn StdError>> {

    let yaml_content = fs::read_to_string(filename).expect("Failed to read file");
    let yaml_guess_list: serde_yaml::Value = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");

    let f = std::fs::File::open(filename).expect("Could not open file.");
    let reader = BufReader::new(f);
    let yaml_guess_list_strings:YamlData = serde_yaml::from_reader(reader).expect("Could not read values.");
    println!("deserialized = {:?}", yaml_guess_list);
    println!("deserialized 2 = {:?}", yaml_guess_list_strings);


    // Open the YAML file
    let file = std::fs::File::open(filename)?;
    let reader = BufReader::new(file);

    // Deserialize the YAML content using serde_yaml::from_reader
    let yaml_data: YamlData = serde_yaml::from_reader(reader)?;

    Ok(yaml_data)
}
```


When File::open fails, we use Err(Box::new(err)) to wrap the concrete error 
into a Box<dyn StdError>. This allows us to return a trait object that 
implements the Error trait, enabling us to handle different error types uniformly.

```rust
fn open_file(filename: &str) -> Result<File, Box<dyn StdError>> {
    match File::open(filename) {
        Ok(file) => Ok(file),
        Err(err) => Err(Box::new(err)),
    }
}

fn open_file_1(filename: &str) -> Result<File, io::Error> {
    match File::open(filename) {
        Ok(file) => Ok(file),
        Err(err) => Err(err),
    }
}
```


Because the Ok(yaml_data) is missing, the compiler assumes that the return procedure
is returning the wrong type or unit, causing a mismatched types error.

```rust
fn read_yaml_file_fail_1(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => Ok(yaml_data), // Return yaml_data if deserialization succeeds
        Err(err) => {
            return Err(err); // Convert the error to the appropriate type and return
        }
    };
}
```
The compiler has no return in this case since there is a semicolon at the end of the match.
This causes a mismatched types error.

```rust
error[E0308]: mismatched types
--> src\main.rs:69:41
|
69 | fn read_yaml_file_fail_1(file: File) -> Result<YamlData, serde_yaml::Error> {
|    ---------------------                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<YamlData, Error>`, found `()`
|    |
|    implicitly returns `()` as its body has no tail or `return` expression
...
79 |     };
|      - help: remove this semicolon to return this value
|
= note:   expected enum `Result<YamlData, serde_yaml::Error>`
        found unit type `()`
```   



Missing Ok statement in the end, creates a problem 
Either the Ok(yaml_data) has to be set at the end as a return or the let yaml_data shadow hats to be removed
If one moves into the game Ok(yaml_data), tthe march operator gets a ; at the end 

```rust
fn read_yaml_file_fail_2(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    let yaml_data = match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => yaml_data,
        //Err(err) => Err(err),
        Err(err) =>
        {
            return Err(err);// Convert the error to the appropriate type, 
        }
    }
    
    Ok(yaml_data) // TODO learn OK return explanation 
    //Ok(()) //you cannot return unit since the yaml_data is expected, again error[E0308]: mismatched types
}
``` 

fn read_yaml_file_fail_3(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    let yaml_data = match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => yaml_data,
        //Err(err) => Err(err),
        Err(err) =>
        {
            return Err(err);// Convert the error to the appropriate type, 
        }
    };
    
    Ok(yaml_data) // TODO learn OK return explanation 
    //Ok(()) //you cannot return unit since the yaml_data is expected
}

# The ultimate solution for error propagation
```rust
let yaml_data: YamlData = serde_yaml::from_reader(reader)?;
```
In Rust, the ? operator is used for error propagation. It's often placed at the end of an expression that returns a Result type. If the result is Ok(value), the value is unwrapped and returned. If the result is Err(error), the error is returned early from the function, and it is expected that the caller will handle the error.

In the context of the comment, it's emphasized that using ? with from_reader(reader) is the preferred way to handle errors during deserialization because it succinctly handles both successful and failed cases.

The phrase "the ultimate solution" implies that using ? operator helps simplify error handling and is considered idiomatic Rust code. However, it's crucial to be aware that the ? operator can only be used in functions that return a Result type.

to avoid the Ok in the end do not shadow the yaml_data in the match expression
in this case the Ok(yaml_data) => yaml_data will be transformed into Ok(yaml_data) => Ok(yaml_data)
there is no statement Ok in the end since the last curly brace does not have a semi colon and this means inplictly that the function ends
```rust
fn read_yaml_file_fail_3_1(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => Ok(yaml_data),
        Err(err) =>
        {
            //return Err("Error reading yaml file please check the file.".into());
            return Err(err);// Convert the error to the appropriate type, 
        }
    }
    //let yaml_data: YamlData = serde_yaml::from_reader(reader)?; // the ultimate solution, but be carefull the ? is a special operator for handling the entire correct incorect result malarky
    //Ok(yaml_data) // TODO learn OK return explanation 
    //Ok(()) //you cannot return unit since the yaml_data is expected
}

fn read_yaml_file_fail_4(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    let yaml_data: YamlData = serde_yaml::from_reader(reader)?;
    Ok(yaml_data) // TODO learn OK return explanation 
    //Ok(()) //you cannot return unit since the yaml_data is expected, error[E0308]: mismatched types
}
```


//It returns a Result<User, Box<dyn Error>>, indicating that it can either 
//successfully return a User struct or an error that implements the Error trait, 
//wrapped in a Box
//The question mark operator (?) unwraps valid values or returns erroneous values, 
//propagating them to the calling function.
//When applied to values of the Result<T, E> type, it propagates errors. If the 
//value is Err(e), then it will return Err(From::from(e))
```rust
//fn read_yaml_file_1(file: File) -> Result<YamlData, serde_yaml::Error> {
fn read_yaml_file_1(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    //let yaml_data = match serde_yaml::from_reader(reader) {
    match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => Ok(yaml_data),
        //Err(err) => Err(err),
        Err(err) =>
        {
            //return Err("Error reading yaml file please check the file.".into());
            return Err(err);// Convert the error to the appropriate type, 
        }
    }
    //let yaml_data: YamlData = serde_yaml::from_reader(reader)?;
    //Ok(yaml_data) // TODO learn OK return explanation 
    //Ok(()) //the inner bracket is called unit and called when there no meaningful return 
}



//https://users.rust-lang.org/t/help-understanding-return-for-box-dyn-error/33748/2
fn read_yaml_file_2(filename: &str) -> Result<YamlData, Box<dyn StdError>> {
    //let file = std::fs::File::open(filename)?;
    let file = if let Ok(file) = File::open(filename) {
        file
    } else {
        return Err("Failed to open file".into()); // Convert the error to the appropriate type
    };
    let reader = std::io::BufReader::new(file);
    let yaml_data: YamlData = serde_yaml::from_reader(reader)?;
    Ok(yaml_data)
}

fn read_yaml_file_3(filename: &str) -> Result<YamlData, Box<dyn StdError>> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let yaml_data: YamlData = serde_yaml::from_reader(reader)?;
    Ok(yaml_data)
}

fn read_yaml_file_4(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    // Remove the semicolon at the end of the match statement
    match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => Ok(yaml_data),
        Err(err) => Err(err),
    }
}
```


Source 
https://fettblog.eu/rust-enums-wrapping-errors/

Sized trait and concepts
https://doc.rust-lang.org/nightly/std/marker/trait.Sized.html
https://huonw.github.io/blog/2015/01/the-sized-trait/

Error handling and propagation
https://fettblog.eu/rust-enums-wrapping-errors/
