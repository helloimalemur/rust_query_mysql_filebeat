// ./threadlog_mysql mysql://foxx:password@kale:3306/ ./output.txt
// show status where variable_name = 'threads_connected';
use mysql::*;
use mysql::prelude::Queryable;
use std::{env, process};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
struct Status {
    threads_connected: String,
    threads: i32
}

fn main() {

    // arguments
    let args: Vec<String> = env::args().collect();

    // argument 1 - url/uri
    let url = match args.get(1) {
        Some(url) => url,
        None => ""
    };

    // argument 2 - output file path
    let path = match args.get(2) {
        Some(p) => p,
        None => "./"
    };

    // no argument message
    if url.len() == 0 {
        println!("{}", "No Arguments..");
        println!("{}", "Query Database for Thread Count!");
        println!("{}", "Format;");
        println!("{}", "./threadlog_mysql mysql://user:password@kale:3306/ ./output.txt");
        process::exit(2);
    }

    // create mysql database connection
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // make query and map to 'Status' struct
    let query = conn.query_map("show status where variable_name = 'threads_connected';",
    |(threads_connected, threads)| {
        Status { threads_connected, threads }
    });

    // print result
    println!("{:?}", query.as_ref().unwrap());

    // format data to be written to log file
    let data = format!("{} {} \n", query.as_ref().unwrap().get(0).unwrap().threads_connected, query.as_ref().unwrap().get(0).unwrap().threads);

    // open file handle
    let mut file = OpenOptions::new().write(true).create(true).append(true).open(path).unwrap();

    // write data to file
    match file.write(data.as_ref()) {
        Ok(ok) => ok,
        Err(e) => panic!("{}", e.to_string())
    };

    // flush buffer on file
    match file.flush() {
        Ok(ok) => ok,
        Err(e) => panic!("{}", e.to_string())
    };

    }
