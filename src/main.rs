use std::{io, vec};
use std::fs;
use std::fs::File;
use std::io::{BufReader};
use std::error::Error as StdError;
use serde::{Serialize, Deserialize};
use serde_yaml::*;
use std::result::Result;


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
/*
When File::open fails, we use Err(Box::new(err)) to wrap the concrete error 
into a Box<dyn StdError>. This allows us to return a trait object that 
implements the Error trait, enabling us to handle different error types uniformly.
*/
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

/* 
//Missing Ok statement in the end, creates a problem 
//Either the Ok(yaml_data) has to be set at the end as a return or the let yaml_data shadow hats to be removed
// If one moves into the game Ok(yaml_data), tthe march operator gets a ; at the end 
fn read_yaml_file_fail_2(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    let yaml_data = match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => yaml_data,
        //Err(err) => Err(err),
        Err(err) =>
        {
            //return Err("Error reading yaml file please check the file.".into());
            return Err(err);// Convert the error to the appropriate type, 
        }
    }
    //let yaml_data: YamlData = serde_yaml::from_reader(reader)?;
    //Ok(yaml_data) // TODO learn OK return explanation 
    //Ok(()) //you cannot return unit since the yaml_data is expected, again error[E0308]: mismatched types
}
*/

fn read_yaml_file_fail_3(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    let yaml_data = match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => yaml_data,
        //Err(err) => Err(err),
        Err(err) =>
        {
            //return Err("Error reading yaml file please check the file.".into());
            return Err(err);// Convert the error to the appropriate type, 
        }
    };
    //let yaml_data: YamlData = serde_yaml::from_reader(reader)?; // the ultimate solution, but be carefull the ? is a special operator for handling the entire correct incorect result malarky
    Ok(yaml_data) // TODO learn OK return explanation 
    //Ok(()) //you cannot return unit since the yaml_data is expected
}


//to avoid the Ok in the end do not shadow the yaml_data in the match expression
//in this case the Ok(yaml_data) => yaml_data will be transformed into Ok(yaml_data) => Ok(yaml_data)
// there is no statement Ok in the end since the last curly brace does not have a semi colon and this means inplictly that the function ends
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

fn fail_to_openfile_1()
{
    //This code structure demonstrates how you can handle file opening errors, print 
    //messages, format strings using the format! macro, and handle return values. Make 
    //sure to adjust the code according to the logic and requirements of your program.
    let open_file_result = open_file("Hello, world!");
    match open_file_result {
        Ok(yaml_data) => yaml_data,
        Err(err) => {
            println!("Failed to open file: {}", err);
            return; // Ensure proper termination or handle the error gracefully
        },
    };
}

fn fail_to_openfile_2()
{
    let open_file_false = open_file_1("Hello, world!");
    let ret = match open_file_false {
        Ok(yaml_data) => yaml_data, // If file opened successfully, return the data
        Err(err) => {
            // Print the error message
            println!("Failed to open file: {}", err);
            return; // Exit the function if there's an error
        }
    };
}

fn main() {

    // first filing test to oupen a non existing file 
    //fail_to_openfile_1();

    // second filing test to oupen a non existing file 
    //fail_to_openfile_2();

    // shows the build failure in case the correct return from the function is missing
    //read_yaml_file_fail_2();

    let open_file_ret = open_file( "Guess_data.yml");
    let open_file_ret_1 = open_file_1( "Guess_data.yml");
}
