use std::{io, fs, fs::File};
use std::io::{BufReader, BufRead};
use std::error::Error as StdError;
use serde::{Serialize, Deserialize};
use serde_yaml::*;
use std::result::Result;

#[derive(Debug, Serialize, Deserialize)]
struct YamlData {
    sentences: Vec<String>,
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

fn read_yaml_file_optimal(filename: &str) -> Result<serde_yaml::Value, Box<dyn StdError>> {
    // Read file content
    let yaml_content = fs::read_to_string(filename)?;

    // Parse YAML content
    let yaml_guess_list = serde_yaml::from_str(&yaml_content)?;

    Ok(yaml_guess_list)
}

fn read_yaml_file(filename: &str) -> Result<YamlData, Box<dyn StdError>> {
    // Open the file and create a reader
    let file = std::fs::File::open(filename)?;
    let reader = BufReader::new(file);

    // Deserialize the YAML content using serde_yaml::from_reader
    let yaml_data: YamlData = serde_yaml::from_reader(reader)?;

    Ok(yaml_data)
}


/*
//Missing Ok statement in the end, creates a problem 
//Either the Ok(yaml_data) has to be set at the end as a return or the let yaml_data shadow hats to be removed
//If one moves into the game Ok(yaml_data), the march operator gets a ; at the end 
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

    //Ok(yaml_data) // TODO learn OK return explanation 
}
*/


//3 different ways to write the same thing
fn read_yaml_file_fail_3(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    let yaml_data = match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => yaml_data,
        Err(err) => return Err(err),
    };
    Ok(yaml_data)
}

fn read_yaml_file_fail_3_1(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    match serde_yaml::from_reader(reader) {
        Ok(yaml_data) => Ok(yaml_data),
        Err(err) => return Err(err),
    }
}

fn read_yaml_file_fail_3_2(file: File) -> Result<YamlData, serde_yaml::Error> {
    let reader = std::io::BufReader::new(file);
    let yaml_data: YamlData = serde_yaml::from_reader(reader)?;
    Ok(yaml_data)
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
    //Ok(yaml_data) // TODO learn OK return explanation 
    //Ok(()) //the inner bracket is called unit and called when there no meaningful return 
}



//https://users.rust-lang.org/t/help-understanding-return-for-box-dyn-error/33748/2
fn read_yaml_file_2(filename: &str) -> Result<YamlData, Box<dyn StdError>> {
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
    //read_yaml_file_fail_2("Hello0");

    // shows the build failure in case the correct return from the function is missing
    //read_yaml_file_fail_2();

    let open_file_Res_Box = open_file( "Guess_data.yml");
    let open_file_Res_Err = open_file_1( "Guess_data.yml");



    let yaml_file_res = read_yaml_file( "Guess_data.yml" );

    println!("YAML deserialized to a vector of strings = {:?}", yaml_file_res);
}
