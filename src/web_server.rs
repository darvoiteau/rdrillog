use warp::{http::StatusCode, reply::Reply, Filter};
use std::{convert::Infallible, sync::Arc};
use tokio::time::{sleep, Duration};

use crate::parser;
use crate:: file_mgmt;

pub async fn launch_srv(parsing_time: u64, addr: &str, port: u16, filename: &str, flogname: &str, elem_to_find: &str, exclude_regex: &str, include_regex: &str, strict: bool, duplicate: bool, output: &str, erase: bool, match_only: &str) {

     // Clone to String (or Arc<String> to share it)
     let flogname = flogname.to_string();
     let elem_to_find = elem_to_find.to_string();
     let exclude_regex = exclude_regex.to_string();
     let include_regex = include_regex.to_string();
     let matchonly = match_only.to_string();
     let output = output.to_string(); // ici le missing clone !
    let erase = erase;
    let duplicate = duplicate;
    let mut first_run = true;
     tokio::spawn(async move {
        loop {
            if first_run == true {
                first_run = false;
            }
            else {
                let match_log_vec = parser::parser(
                    &flogname,
                    &elem_to_find,
                    &exclude_regex,
                    &include_regex,
                    strict,
                    duplicate,
                    &matchonly
                );
                file_mgmt::save_file(&output, match_log_vec, erase, duplicate).unwrap_or_else(|e|{
                    eprintln!("Error ! Failed to save content in the file: {}", e);
                    std::process::exit(1);

                });
            }

            sleep(Duration::from_secs(parsing_time)).await;
        }
    });

    let fname_shared = Arc::new(filename.to_string());
    let filename_static = filename.to_string().into_boxed_str();
    // Route GET /filename
    let route ={
        let fname_shared_clone = fname_shared.clone();
        warp::path(filename_static.clone())
        .and(warp::get())
        .and_then(move || handle_serve_ips(fname_shared_clone.clone()))
    };

    //Split each octal of ip address and convert it to u8 because warp need erach ip octal in u8 and at this step we have a single &str
    let ip_srv_vec_str: Vec<&str> = addr.split('.').collect();
    let mut ip_srv_vec_u8: Vec<u8> = Vec::new();
    for elem in ip_srv_vec_str {
        ip_srv_vec_u8.push(elem.parse().unwrap());
        
    }
    println!("Web server is running: http://{}:{}/{}", addr,&port,filename);
    // Start the server
    warp::serve(route)
        .run(([ip_srv_vec_u8[0], ip_srv_vec_u8[1], ip_srv_vec_u8[2], ip_srv_vec_u8[3]], port))
        .await;
    
}

//Manage response of webserver when the program receive a request
async fn handle_serve_ips(filename: Arc<String>) -> Result<impl warp::Reply, Infallible> {
    match std::fs::read_to_string(&*filename) {
        Ok(content) => {
            let response = warp::reply::with_header(content, "Content-Type", "text/plain");
            Ok(response.into_response())
        }
        Err(_) => {
            let response = warp::reply::with_status("Fichier introuvable", StatusCode::NOT_FOUND);
            Ok(response.into_response())
        }
    }
}

