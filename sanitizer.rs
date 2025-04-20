use std::net::IpAddr;
use regex::Regex;


//Check if given regex is not empty
pub fn regex_verif(regex: &str, required: bool){
    let nb_char = regex.len();

    if nb_char < 1 && required == true {
        eprintln!("Error ! You need to define the option 'f' or 'elem_to_find'");
        std::process::exit(1);
    }
}

//Check if the given str is not empty and if it is required parameter the programm will be stopped
pub fn empty_str_check(str: &str, required: bool) -> &str{
    let nb_char = str.len();
    if nb_char < 1 && required == true {
        eprintln!("Error ! Verify the option 'l' or 'logfile', or ip and port option if you want to use web server");
        std::process::exit(1);
    }
str
}

//Verify in the case of the usage of webserver if the output file contain '/'char. If it is the case the program will be stopped
pub fn wb_file_path_check(filename: &str, webserver: bool){
    let rgx_slash_windows = Regex::new("[/\\/]").unwrap();
    let rgx_slash_linux = Regex::new("/").unwrap();
    if rgx_slash_linux.is_match(filename) || rgx_slash_windows.is_match(filename) && webserver == true {
        eprintln!("Error ! You cannot use '/' or '\' when you want to use webserver");
        std::process::exit(1);
    }

}

//Check if the given ip by the user is a correct ip
pub fn check_ip(ip: &str){
    let ip_re = Regex::new("[a-zA-Z]").unwrap();
    match ip.parse::<IpAddr>(){
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error ! Bad Ip given for the webserver IP: {}", e);
            std::process::exit(1);
        }
    };

    if ip_re.is_match(ip){
        eprintln!("Error ! Please define a valid IP for the webserver.\nNotice: Ipv6 is not currently suported by Rdrillog\n");
        std::process::exit(1);
    }


}

//Check if the given port by the user is correct
pub fn port_check(port: u16){
    if port > 65534 {
        eprintln!("Error ! The given port is out of supported ports by standards of network");
        std::process::exit(1);
    }
    else if port == 0{
        eprintln!("Error ! Please specify a port.");
        std::process::exit(1);
    }
}

//Check if the filename given by the user not contain unwanted char
pub fn filename_check(filename: &str){
    let unwaned_special_chars_re = Regex::new(r#"[\\\/=:*?"',;!{}\[\]()'<>|]+"#).unwrap();
    if unwaned_special_chars_re.is_match(filename) {
        eprintln!("Error ! The given filename '{}' contain invalid special character", filename);
        std::process::exit(1);
    }
}

//Verify if the user not use --strict and --match_only option in the same time
pub fn strict_matchonly(strict: bool, match_only: &str){
    if strict == true && match_only != "$#$$##$$#$#" {
        eprintln!("Error! Strict option and match_only option cannot be set in the same time ! \n");
        std::process::exit(1);
    }
}