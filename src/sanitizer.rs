use std::net::IpAddr;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Local, NaiveDateTime, TimeZone, Datelike};
use std::io::BufRead;
use std::collections::BTreeMap;

use crate::file_mgmt;

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

pub fn bchart_format(logs: Vec<String>) -> Vec<(String, u64)>{
    let mut hashmap_bchart = HashMap::new();
    let mut existing_lines: HashSet<String> = HashSet::new();
    let mut i: u64 = 0;

    for elem in logs{
        if &elem != "\n" && existing_lines.insert(elem.clone()) == true {
            hashmap_bchart.insert(elem, 1);
            i = 1;
        }
        else if existing_lines.insert(elem.clone()) == false{
            i +=1;
            if let Some(value) =  hashmap_bchart.get_mut(&elem){
                *value = i;
            }

        }
        
    }

    let vec_bchart: Vec<(String, u64)> = hashmap_bchart.into_iter().collect();
    vec_bchart
}
#[allow(deprecated)]
pub fn schart_format(logs: Vec<String>) -> Vec<u64> {
    let cmn_date_log = Regex::new(r#"\d{2}/[A-Za-z]{3}/\d{4}:\d{2}:\d{2}:\d{2} [+\-]\d{4}"#).unwrap();
    let iso8601_date = Regex::new(r#"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+\-]\d{2}:\d{2})"#).unwrap();
    let syslog_date = Regex::new(r#"[A-Z][a-z]{2} \d{1,2} \d{2}:\d{2}:\d{2}"#).unwrap();
    let sql_date = Regex::new(r#"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}"#).unwrap();
    let timestamp_date = Regex::new(r#"\b\d{10}\b"#).unwrap();

    let mut dates_utc = Vec::new();

    for line in logs {
        if let Some(m) = cmn_date_log.find(&line) {
            if let Ok(dt) = DateTime::parse_from_str(m.as_str(), "%d/%b/%Y:%H:%M:%S %z") {
                dates_utc.push(dt.with_timezone(&Utc));
            }
        } else if let Some(m) = iso8601_date.find(&line) {
            if let Ok(dt) = m.as_str().parse::<DateTime<Utc>>() {
                dates_utc.push(dt);
            }
        } else if let Some(m) = syslog_date.find(&line) {
            let date_str = m.as_str();
            let current_year = Local::now().year();
            let full_input = format!("{} {}", current_year, date_str);
            if let Ok(naive) = NaiveDateTime::parse_from_str(&full_input, "%Y %b %e %H:%M:%S") {
                if let Some(local) = Local.from_local_datetime(&naive).single() {
                    dates_utc.push(local.with_timezone(&Utc));
                }
            }
        } else if let Some(m) = sql_date.find(&line) {
            if let Ok(naive) = NaiveDateTime::parse_from_str(m.as_str(), "%Y-%m-%d %H:%M:%S") {
                let dt = Local.from_local_datetime(&naive).unwrap().with_timezone(&Utc);
                dates_utc.push(dt);
            }
        } else if let Some(m) = timestamp_date.find(&line) {
            if let Ok(ts) = m.as_str().parse::<i64>() {
                if let Some(naive) = NaiveDateTime::from_timestamp_opt(ts, 0) {
                    let dt = DateTime::<Utc>::from_utc(naive, Utc);
                    dates_utc.push(dt);
                }
            }
        }
    }

    // Regroupement par périodes régulières (ex: 60 secondes)
    let period_secs = 60;
    let mut counts = BTreeMap::new();

    for dt in dates_utc {
        let ts = dt.timestamp();
        let bucket = ts - (ts % period_secs);
        *counts.entry(bucket).or_insert(0) += 1;
    }

    // Extraction finale
    counts.values().copied().collect()
}

pub fn gchart_format(logs: Vec<String>, filename: &str) -> u16{
    let file_log_content = file_mgmt::get_file(filename, true);
    let mut i: u64 = 0;

    for _elem in file_log_content.lines(){
        i += 1;
    }

    let j = logs.len();

    let percent: u64;

    if j as u64 >= i {
        percent = 100 * i / j as u64;
    }
    else{
        percent = 100 * j as u64 / i;
    }
    

    percent as u16

}