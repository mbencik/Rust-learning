This is a analysis of my journes in learning Rust and how to handle the finer but critical points in resolving Rust design issues

In this document the goal is to adres the error propagation, error handling, nonsense with the documentation and problems with the explanations.

```Box<dyn Error>``` is hit early, the reason is in my case that just wanted to open a file and read it, sounds logical. Right? Welll not so fasst in rust those two thigs produce different types of error which in turn if you want ot propagate the need special care. This is when ```Box<dyn Error>```, but in the official documentation there is no definitive guidance what should happen if we need to handle multiple diffenrent errors and return the ```Result<>``` type. And this happens pritty much right away. Now I have some experience with programming in C/C++ but there the idea is different there one gives or handles a type here one handles a Trait. Now this is no type, but a type/struct inherits the trait but the trait does not have variables butr it has methods which on it self is very confusing. 

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

let yaml_data: YamlData = serde_yaml::from_reader(reader)?;
```
In Rust, the ? operator is used for error propagation. It's often placed at the end of an expression that returns a Result type. If the result is Ok(value), the value is unwrapped and returned. If the result is Err(error), the error is returned early from the function, and it is expected that the caller will handle the error.

In the context of the comment, it's emphasized that using ? with from_reader(reader) is the preferred way to handle errors during deserialization because it succinctly handles both successful and failed cases.

The phrase "the ultimate solution" implies that using ? operator helps simplify error handling and is considered idiomatic Rust code. However, it's crucial to be aware that the ? operator can only be used in functions that return a Result type.

//to avoid the Ok in the end do not shadow the yaml_data in the match expression
//in this case the Ok(yaml_data) => yaml_data will be transformed into Ok(yaml_data) => Ok(yaml_data)
// there is no statement Ok in the end since the last curly brace does not have a semi colon and this means inplictly that the function ends
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
    //Ok(()) //you cannot return unit since the yaml_data is expected, again error[E0308]: mismatched types
}



//It returns a Result<User, Box<dyn Error>>, indicating that it can either 
//successfully return a User struct or an error that implements the Error trait, 
//wrapped in a Box
//The question mark operator (?) unwraps valid values or returns erroneous values, 
//propagating them to the calling function.
//When applied to values of the Result<T, E> type, it propagates errors. If the 
//value is Err(e), then it will return Err(From::from(e))
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
