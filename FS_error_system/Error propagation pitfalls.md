# Definition of Error Propagation for Rust Beginners

I was trying to learn the basics and get accustomed to Rust code. My idea was to take the guessing game from the tutorial and simply add a file from which I can read words or sentences to make it configurable. I wanted to introduce some modularity to allow for the addition of new guessing sentences. However, I found out that at this point in Rust, this task is anything but easy, and I pretty much landed myself in the more advanced areas of Rust code. 

The definition was simple: a program does something, it fails, or something goes wrong, and it should give you as a programmer a hint about what happened. This looks simple, but in my journey, it turned out to be an obstacle course littered with mines. 

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

    Ok(yaml_data) // TODO learn OK return explanation 
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
    
    Ok(yaml_data) // TODO learn OK return explanation 
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
https://huonw.github.io/blog/2015/01/the-sized-trait/

Error handling and propagation
https://fettblog.eu/rust-enums-wrapping-errors/
