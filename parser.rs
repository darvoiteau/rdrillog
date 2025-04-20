use regex::Regex;
use std::io::BufRead;
use std::collections::HashSet;

use crate::file_mgmt;


pub fn parser(filename: &str, elem_to_find: &str, exclude_regex: &str, include_regex: &str, strict: bool, duplicate: bool, matchonly: &str) -> Vec<String>{
    //To have CR contained in String
    let string = String::from("\n");

    //Convert given pattern search by the user to regex
    let elem_re = Regex::new(&elem_to_find).unwrap();
    let exclude_re = Regex::new(&exclude_regex).unwrap();
    let include_re = Regex::new(&include_regex).unwrap();
    let matchonly_re = Regex::new(&matchonly).unwrap();

    //Get log contained by the given filename by the user
    let file_log_content = file_mgmt::get_file(filename, true);

    //Vector that will contain logs filtered depending or pattern given by the user (-f -i -e -m)
    let mut log_match_vec: Vec<String> = Vec::new();

    //Create HashSet to verify duplicate entry with more speed
    let mut existing_lines: HashSet<String> = HashSet::new();

    
    for elem in file_log_content.lines() {
        match elem{
            Ok(elem) =>{
                if elem_re.is_match(&elem) && !exclude_re.is_match(&elem) && strict == false || include_re.is_match(&elem) && !exclude_re.is_match(&elem) && strict == false{
                       if matchonly == "$#$$##$$#$#"{
                            log_match_vec.push(elem.clone());
                            log_match_vec.push(string.clone());
                       }
                       //If match_only is set we take only the element matchec by math_only in -i and -f that not match with -e
                       else if matchonly != "$#$$##$$#$#"{
                        //For loop to add elem that match with -f
                        for match_item in elem_re.find_iter(&elem){
                            for match_in_matchonly in matchonly_re.find_iter(&match_item.as_str().to_string()){
                                log_match_vec.push(match_in_matchonly.as_str().to_string());
                                log_match_vec.push(string.clone());
                            }
                        }
                        //For loop to add elem that match with -i
                        for match_item in include_re.find_iter(&elem){
                            for match_in_matchonly in matchonly_re.find_iter(&match_item.as_str().to_string()){
                                log_match_vec.push(match_in_matchonly.as_str().to_string());
                                log_match_vec.push(string.clone());
                            }
                        
                        }
                            
                       }              
                }
                else if elem_re.is_match(&elem) && !exclude_re.is_match(&elem) && strict == true || include_re.is_match(&elem) && !exclude_re.is_match(&elem) && strict == true {
                    //For loop to add elem that match with -f
                    for match_item in elem_re.find_iter(&elem){
                            log_match_vec.push(match_item.as_str().to_string());
                            log_match_vec.push(string.clone());
                            //existing_lines.insert(match_item.as_str().to_string()); 
                    }
                    //For loop to add elem that match with -i
                    for match_item in include_re.find_iter(&elem){
                        log_match_vec.push(match_item.as_str().to_string());
                        log_match_vec.push(string.clone());
                    }
                    
                }

            }
            Err(e) => {
                // If error when we read the line, we display an error
                eprintln!("Error of line reading : {}", e);
                continue;
            }
        
        

        }
    }
    //Here the goal is to find duplicated elem and remove it if the -d is set to false by the user
    if duplicate == false {
        let mut log_match_vec2: Vec<String> = Vec::new();
        //Check with our HashSet if an elem is a duplication or not.
        for elem in log_match_vec {
            if &elem != "\n" && existing_lines.insert(elem.clone()) == true {
                log_match_vec2.push(elem);
                log_match_vec2.push(String::from("\n"));

                
            }

        }
    //log_match_vec2 contains logs after duplication checking
    log_match_vec = log_match_vec2;
    }
    


log_match_vec

}