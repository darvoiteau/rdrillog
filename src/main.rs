use argh::FromArgs;

mod parser;
mod file_mgmt;
mod web_server;
mod sanitizer;
mod graph;

#[derive(FromArgs)]
///reach new args
struct Args {
    #[argh(option, short = 'f')]
    ///pattern to find in logs. For regex please use ""
    elem_to_find: String,

    #[argh(option, short = 'i', default = r#"String::from("$#$$##$$#$#")"#)]
    ///pattern to find in addition to the pattern defined by -f, for regex use ""
    include_regex: String,

    #[argh(option, short = 'e', default = r#"String::from("$#$$##$$#$#")"#)]
    ///logs found in -i and -f to exclude by a pattern, for regex use ""
    exclude_regex: String,

    #[argh(option, short = 'o', default = r#"String::from("output.txt")"#)]
    ///output filename, by default 'output.txt'
    output: String,

    #[argh(option, short = 'm', default = r#"String::from("$#$$##$$#$#")"#)]
    ///will take only things that match with this pattern in matched -f -i logs instead of the take the entire line of log, for regex use ""
    match_only: String,

    #[argh(option, short = 'l')]
    ///logfile to analyze
    logfile: String,

    #[argh(option, short = 's', default = "false")]
    ///take only things that match with -f -l pattern instead of the take the entire line of log. To enable it, use "true" value
    strict: bool,

    #[argh(option, short = 'd', default = "true")]
    ///if this option is disabled, no duplicate entry will be saved in the output file, to disable it use "false" value
    duplicate: bool,

    #[argh(option, short = 'w', default = "false")]
    ///enable a webserver to display logs filtered in the output file
    webserver: bool,

    #[argh(option, default = r#"String::from("254.254.254.254")"#)]
    ///set the ip listening for the webserver
    ip: String,

    #[argh(option, default = "0")]
    ///set the port listening for the webserver
    port: u16,

    #[argh(option, default = "3600")]
    ///define the interval between each new alayze of the given logfile in seconds, by default 3600S -> 1h
    parsing_time: u64,

    #[argh(option, default = "true")]
    ///if disabled, this option will modify the output file without erase the existing content of this output file, to disable it use "false" value
    erase: bool,

    #[argh(option, short = 'g', default = "false")]
    ///if enabled, rdrillog will show some graphs about found logs. To enable it, use "true"
    graph: bool,

    #[argh(option, default= "60")]
    ///period in sec used for sampling in graphs. The smaller is, the most it will take a time but it will be more precise. By default 60s
    sampling: i64




}
fn main() {
    let mut ip_str: &str = "";
    let options: Args = argh::from_env();
    //Check if the parameter -f is not empty
    sanitizer::regex_verif(&options.elem_to_find.as_str(), true);
    //Check if this options must be provided by the user, if it the case and it is empty the program will be stopped
    //Str will be returned
    let output = sanitizer::empty_str_check(&options.output.as_str(), false);
    let logfile = sanitizer::empty_str_check(&options.logfile.as_str(), true);
    //Check if filename contain unwanted char
    sanitizer::filename_check(&options.output.as_str());
    sanitizer::filename_check(&options.logfile.as_str());
    //Check if option --strict and --match_only are enabled in same time
    sanitizer::strict_matchonly(options.strict, &options.match_only.as_str());
    //If we want the webserver, we verify the port, ip, and output file given by the user
    if options.webserver == true {
        ip_str = sanitizer::empty_str_check(&options.ip.as_str(), true);
        sanitizer::check_ip(&options.ip.as_str());
        sanitizer::port_check(options.port);
        sanitizer::wb_file_path_check(&options.output, options.webserver);

    }

    //Start parsing log and get logs parsed and filtered in a String Vector
    let match_log_vec = parser::parser(logfile, &options.elem_to_find.as_str(), &options.exclude_regex.as_str(), &options.include_regex.as_str(), options.strict, options.duplicate, &options.match_only.as_str());
    
    //Save parsed and filtered log in the output file
    file_mgmt::save_file(output, match_log_vec.clone(), options.erase, options.duplicate).unwrap_or_else(|e|{
        eprintln!("Failed to save content to file: {}", e);
        std::process::exit(1);
    });

    //Call multithreading function that will run webserver and reparse each x seconds logs
    if options.webserver == true {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(web_server::launch_srv(options.parsing_time, ip_str, options.port, &options.output.as_str(), logfile, &options.elem_to_find.as_str(), &options.exclude_regex.as_str(), &options.include_regex.as_str(), options.strict, options.duplicate, &options.output.as_str(), options.erase, &options.match_only.as_str()));

    }
    else if options.webserver == false && options.graph == true{
        println!("Logs are ready ! Building graphs... \n");
        let vec_bchart = sanitizer::bchart_format(match_log_vec.clone());
        let values_vec = sanitizer::schart_format(match_log_vec.clone(), options.sampling);
        let percent = sanitizer::gchart_format(match_log_vec, &options.logfile);
        graph::graph_display(vec_bchart, values_vec, percent).unwrap_or_else(|e|{
            eprintln!("Failed to display and build charts: {}",e);
            std::process::exit(1);
        });

    }

    
}
