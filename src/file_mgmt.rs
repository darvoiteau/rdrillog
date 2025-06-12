use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::collections::HashSet;

//To get the content of file
pub fn get_file(filename: &str, required: bool) -> io::BufReader<File>{
        //We try to open a file, and if its an required option (ex: logfile) the program wil be stopped. Otherwise the programm will create the file.
        let _file = File::open(&filename).unwrap_or_else(|_| {
            if required == true {
                eprintln!("Given filename dosen't exist !\n");
                std::process::exit(1);
            }
            File::create(filename).unwrap_or_else(|e|{
            eprintln!("Failed to open/create file: {}", e);
            std::process::exit(1);

            })
            
       });
       //Open the file
       let file = File::open(&filename).unwrap();
    
    //Take the content of the file
    let reader = io::BufReader::new(file);

    reader

}

pub fn save_file(filename: &str, mut text: Vec<String>, erase_file: bool, duplicate: bool) -> std::io::Result<()>{
    
    //If we don't want to erase the existing output file, we take the content of it and add filtered logs of this current rdrillog run
    if erase_file == false {
        let mut text_already: Vec<String> = Vec::new();
        let file_content = get_file(filename, false);

        //Use HashSet to find duplication more quickly if the user don't want duplication
        let mut existing_lines: HashSet<String> = HashSet::new();
        for elem in file_content.lines(){
            if duplicate == false {

            }
            else if duplicate == true {
                text_already.push(elem?);
            }
        }
        if duplicate == false {
            for elem in text {
                if elem != "\n" && existing_lines.insert(elem.clone()) == true{
                    text_already.push(elem);
                    text_already.push(String::from("\n"));   
                }
            }

        }
        text = text_already;
    }
    
    let mut file_save: File = File::create(filename)?;
    //Write all logs to the output file
    for elem in text{
        file_save.write_all(&elem.as_bytes())?;
    }

Ok(())

}
