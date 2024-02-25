# Definition of Error Propagation for Rust Beginners

## Intro

I was delving into Rust to learn its basics and get accustomed to its code. My aim was to enhance the guessing game from the tutorial by incorporating a file for reading words or sentences, thus making it configurable and introducing modularity for adding new guessing sentences. However, I quickly realized that, at this stage in Rust, this task isn't straightforward and led me into more advanced areas of Rust code.

In this discussion, the focus is on error propagation, error handling, issues with documentation, and problems with explanations.

Understanding error handling in Rust is vital. A program should provide clues to programmers when something goes wrong. Despite its apparent simplicity, error handling in Rust can feel like navigating a minefield. It's concerning that beginners encounter advanced Rust features, like file reading and error handling, without adequate guidance or support.

The main challenge lies in the complexity surrounding seemingly simple tasks, such as reading a file and handling potential errors effectively. Whether errors are created on the heap or the stack adds unnecessary complexity, making it difficult for beginners to grasp. Resolving basic file reading errors can take an entire day or longer, which seems disproportionate.

A beginner-friendly solution involves extending the error struct with baseline implementations for common error scenarios. This would simplify error identification and handling. The '?' operator hides error resolution complexities, benefiting beginners but potentially limiting control for advanced users.

This issue isn't isolated to file operations; it extends to scenarios requiring Rust's generics or interfaces, complicating error propagation and handling further.


## Basics or error handling

The current error propagation has turned out to be an exercise in finding one's way in a labyrinth rather than simply reading a file if okay, parsing the file. The main issue turned out to be the Result return. Since I'm a C/C++ engineer, or something like that, I was baffled by the idea that there is a type that has 2 Enums, and they are returned. In C++ there are templates for that kind of behavior. Now already, the problems begins. In C++, an Enum is a number basically, either assigned by the compiler or given by the user, but in Rust, that is a Struct, a class that is almighty and confusing.

Let's dissect the basic problems. The first problem: what happens with the error if there is none? So everything went well (for example, read file). A normal result is returned, but to be complete or exhaustive, if I check a Result return, it has Ok and Err, I need to check both. It took me literally half an hour to find what it returns in the case. It returns a unit struct, or in translation, a struct with no data, that is resolved and understood by the compiler. Just to let you know, the Rust compiler is a weird magic thing that resolves all sorts of issues.

But why does one make a struct out of an Enum? Enum should be simple, easy, a number connected to a label in the code, done. So to sort out the enum that is a struct, and so-called unit struct in cases that one needs all the time, one needs pretty good knowledge of what the compiler will do. Otherwise, it is difficult to figure out what the compiler does automagically and what logic has to be done by the user.

Wait, so there is an Ok and an Err, both Enums, but they accept structs and can return them. The following code, from the Rust book, shows how the Result type is envisioned: a Struct with two possible Enums. However, the Enums are generics, these are templates in C++, that will accept various types.
```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Okay, I understood the **Result** type as a return type. But I want to read and parse a file. Let's take a look at reading and parsing, so 2 different types of errors are possible. I think I need something like generics, akin to templates in C++? Ah, that makes sense; the Enums in the **Result** type are generics, and through them, I just send the types back. Okay, they don't have a simple way to propagate them out, but it still works. This seems simple enough (boy, am I wrong), more on that later. The main example is in this repository, and the situation is that we have a single YAML file that we want to read. I am going to formalize and summarize my research or learning on these subjects, and yes, subjects. The previous discussion was just a taste of what is happening in the code in-depth and what needs to be done.

```rust
//The error is that the 
fn read_yaml_file_fail_2(filename: &str) -> Result<YamlData, serde_yaml::Error> {
    let file_res = match File::open(filename) {
        Ok(file) => Ok(file),
        Err(err) => Err(err),
    };

    let reader = std::io::BufReader::new(file_res);
    let yaml_data = match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => yaml_data,
        //Err(err) => Err(err),
        Err(err) =>
        {
            //return Err("Error reading yaml file please check the file.".into());
            return Err(err);// Convert the error to the appropriate type, 
        }
    };

    Ok(yaml_data)
}


error[E0277]: the trait bound `Result<File, std::io::Error>: std::io::Read` is not satisfied
    --> src\main.rs:63:51
     |
     |     let yaml_data = match serde_yaml::from_reader(reader) {
     |                           ----------------------- ^^^^^^ the trait `std::io::Read` is not implemented for `Result<File, std::io::Error>`
     |                           |
     |                           required by a bound introduced by this call
     |
     = help: the trait `std::io::Read` is implemented for `BufReader<R>`
     = note: required for `BufReader<Result<File, std::io::Error>>` to implement `std::io::Read`
```

The code attempts to return a **```Result<File, std::io::Error>```** from the match expression, which can produce either an error or success. However, the error returned by **serde_yaml::from_reader** is of type **serde_yaml::Error**, not **std::io::Error**. The error message accurately identifies the location and nature of the type mismatch, but its suggested solution is misleading.

```rust
fn read_yaml_file_optimal(filename: &str) -> Result<YamlData, serde_yaml::Error> {
    // Read file content
    let yaml_content = fs::read_to_string(filename)?;

    // Parse YAML content
    let yaml_guess_list = serde_yaml::from_str(&yaml_content)?;

    Ok(yaml_guess_list)
}

error[E0277]: `?` couldn't convert the error to `serde_yaml::Error`
  --> src\main.rs:38:52
   |
   | fn read_yaml_file_optimal(filename: &str) -> Result<YamlData, serde_yaml::Error> {
   |                                              ----------------------------------- expected `serde_yaml::Error` because of this
   |     // Read file content
   |     let yaml_content = fs::read_to_string(filename)?;
   |                                                    ^ the trait `From<std::io::Error>` is not implemented for `serde_yaml::Error`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             <serde_yaml::Error as From<serde_yaml::libyaml::error::Error>>
             <serde_yaml::Error as From<serde_yaml::libyaml::emitter::Error>>
```
The error message **error[E0277]: ? couldn't convert the error to serde_yaml::Error** occurs because the question mark operator (**?**) in the code is attempting to convert the error type to **serde_yaml::Error**, but the conversion is not possible. Specifically, in the given code, the **fs::read_to_string** function is expected to return a **```Result<YamlData, serde_yaml::Error>```**, but it returns a **```Result<YamlData, std::io::Error>```** instead. The compiler notes that the **```From<std::io::Error>```** trait is not implemented for **serde_yaml::Error**, which prevents the automatic conversion. The help message suggests alternative types that implement the **From<T>** trait, but they do not resolve the immediate issue with the conversion from **std::io::Error** to **serde_yaml::Error**. In this case the compiler is not misleading, and it is easy to deduce that we need asolution for the return value.

Just to dissect types of errors that the program will encounter. The first is the file I/O. For example, if the file does not exist, the drive is not accessible, or the operating system doesn't let us access the directory, or some other problem arises. The other issue can be that the YAML file is parsed by the serde crate (Serde -> short for serialization deserialization) while reading/parsing the YAML file. For example, the format doesn't fit, or there are values that don't comply, or any other problems. The issue is that we have to then propagate the error for the user to see or the system to automatically break off and notify the user.

How to handle them. This code represents the example of which types can be returned. As we can see, there are 2 types of errors.
```Rust
Result<YamlData, serde_yaml::Error>
Result<File, io::Error>
```

#### Option one
To avoid returning 2 types of errors through the result, we can split the function into 2 functions.
The following code shows how this is implemented in the example code that is submitted.

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
    
    Ok(yaml_data) // 
}
``` 
Reviewing this code, I personally believe that it could be improved to handle the errors more efficiently. It seems like the current implementation could be simplified to handle all cases within a single function, without any issues.


#### Option two

In the typical scenario in C++, handling exceptions on the spot or returning through templated functions would suffice. We open the file and process the YAML file or whichever file we have. Here, we face a problem: the open file function can fail, and the parse YAML file can also fail. The challenge lies in returning the error out of these functions. We need to handle both types through a generic type. This generic type corresponds to the **Err** part of the **```Result<Ok, Err>```** type. But how do we return it?

In Rust, there are some fundamental rules. The type's size should be known at compile time. Also, there is the criteria of monomorphization, from the compiler book [link](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html) :

> "Rust takes a different approach: it monomorphizes all generic types. This means that compiler stamps out a different copy of the code of a generic function for each concrete type needed."

##### Explanation of the dyn keyword, dynamic dispatch, sized and trait objects

Returning an unknown-size type in Rust presents a challenge. One approach is to use **```Box<dyn Error>```**. But what exactly is **```Box<dyn Error>```** and its relation to Result?

In Rust, **```Result<Ok, Err>```** represents either success or failure. The Err variant requires type and size specifications at compile time. To handle this, Rust introduces trait objects like **```Box<dyn Error>```**. These are references to types implementing a trait, but their size isn't known at compile time.

In Rust, a trait object is created by combining the dyn keyword with a trait name, like **dyn MyTrait**. It represents any type that implements the specified trait. Trait objects allow you to treat different types that implement the same trait as interchangeable.
To work with trait objects, Rust uses references (**&**) or smart pointers like **Box** or **Arc**. **```Box<dyn Error>```** acts as a pointer to parts of the trait object, represented by a fat pointer containing data and vtable pointers. While the vtable contains trait methods, the data part's size is unknown until runtime. Rust's compiler employs dynamic dispatch with the dyn keyword to resolve this at runtime. Sized types offer flexibility, as the compiler can manipulate them directly. Placing an unsized type behind a pointer makes it sized. Box<Trait> allows handling a trait object like a normal value, ensuring sizedness without changing ownership semantics.

For example, suppose you have a trait called Drawable with a method **draw(&self)**. You can create a trait object dyn Drawable that can hold any type implementing the Drawable trait. This allows you to call the draw method on any object that implements Drawable, regardless of its concrete type.

Trait objects are useful in scenarios where you need to work with different types in a polymorphic way, such as creating collections of objects with different concrete types but similar behavior.

Trait objects in Rust present a way to work with different types through a common interface provided by a trait. The **dyn** keyword is used to denote that a type is a trait object, allowing for dynamic dispatch and polymorphic behavior at runtime.

In our example **```Box<dyn Error>```** is a tool for handling errors of unknown size, facilitating effective error management and propagation in Rust applications.

```Rust
Result<YamlData, Box<dyn std::error::Error>>
```
The official documentation on this topic can be unclear, leading to confusion. In the following sections, we'll delve into the problems and ideas behind the syntax construction. So you might exactly as me strugle through the information gathering process and understanding things. In the next parts I will try to define the problems and the ideas behind the construction of syntax.

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
The code **```Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))```** creates a new instance of the **```std::io::Error```** struct with the specified error kind (in this case, **```std::io::ErrorKind::NotFound```**) and error message ("File not found"). The **Box::new** function is used to box the error instance, allocating it on the heap and returning a pointer to the boxed error. This boxed error can be returned from functions or passed around as needed, allowing for consistent error handling in Rust. This is basically how the heap allocations happen.


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

## Some standard mishaps with the code 

### Problems with syntax and compiler 
 
#### Compiler misleading

OK and no semicollo or semi colon

explanation how the rust compiler is capable only to check for a spacific block of errors and stops at a error

#### Mesage incomprehensible



#### Misleading syntax and compiler nonsense

```rust
use std::fs::File;
use std::io::{BufReader, BufRead};
use serde_yaml;
use std::error::Error as StdError;

#[derive(Debug, Serialize, Deserialize)]
struct YamlData {
    sentences: Vec<String>,
}

fn read_yaml_file(filename: &str) -> Result<YamlData, Box<dyn StdError>> {
    // Open the file and create a reader
    let file = std::fs::File::open(filename)?;
    let reader = BufReader::new(file);

    // Deserialize the YAML content using serde_yaml::from_reader
    let yaml_data: YamlData = serde_yaml::from_reader(reader)?;

    Ok(yaml_data)
}
```

Here we can see the minimal implementation to read a YAML file. The nonsense here is the naming of the std::io members. BufRead is a trait, and BufReader is a struct that implements BufRead. Once you get used to it, it's not bad, but if you make a syntax mistake, you will not be happy. 

```rust
error[E0599]: no function or associated item named `new` found for trait `BufRead`
  --> src\main.rs:51:27
   |
51 |     let reader = BufRead::new(file);
   |                           ^^^ function or associated item not found in `BufRead`
```

Here is one possible error.


// shadowed variable nonsense and no return

https://joshleeb.com/posts/rust-traits-and-trait-objects/
https://www.reddit.com/r/rust/comments/vzq6pd/is_boxstderrerror_acceptable/
https://betterprogramming.pub/rust-basics-structs-methods-and-traits-bb4839cd57bd




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
``` 

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
https://doc.rust-lang.org/book/ch10-02-traits.html
https://huonw.github.io/blog/2015/01/the-sized-trait/

Error handling and propagation
https://fettblog.eu/rust-enums-wrapping-errors/
