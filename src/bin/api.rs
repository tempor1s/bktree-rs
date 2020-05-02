#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use bktree_rs::*;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug, Deserialize, Serialize)]
struct ExactCloseResponse {
    exact: Vec<String>,
    close: Vec<String>,
}

impl<'a> ExactCloseResponse {
    fn new(exact: Vec<String>, close: Vec<String>) -> Self {
        ExactCloseResponse { exact, close }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the 'did you mean' api."
}

#[get("/<word>/<tolerence>")]
fn get_word(
    word: String,
    tolerence: usize,
    bktree: State<SpellTree<String>>,
) -> Json<ExactCloseResponse> {
    // find the item in the tree
    let (exact, close) = bktree.find(&word, tolerence);

    // convert &Strings to a normal String
    let exact: Vec<String> = exact.iter().map(|val| val.to_string()).collect();
    let close: Vec<String> = close.iter().map(|val| val.to_string()).collect();

    // return the json response
    Json(ExactCloseResponse::new(exact, close))
}

fn generate_tree(filename: &str) -> SpellTree<String> {
    let words = read_words_from_file(filename);
    let gen_tree: SpellTree<String> = SpellTree::new_from_vec(words.unwrap());
    gen_tree
}

fn read_words_from_file(filename: &str) -> Result<Vec<(String, String)>, Error> {
    let f = File::open(filename)?;
    let buffered = BufReader::new(f);
    let mut output: Vec<(String, String)> = vec![];

    for line in buffered.lines() {
        let line = line?;
        output.push((line.clone(), line));
    }

    Ok(output)
}

fn main() {
    let tree: SpellTree<String> = generate_tree("words.txt");
    rocket::ignite()
        .manage(tree)
        .mount("/", routes![index, get_word])
        .launch();
}
