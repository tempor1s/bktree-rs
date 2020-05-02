#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use bktree_rs::*;
use rocket::response::status;
use rocket::State;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[get("/<word>/<tolerence>")]
fn index(
    word: String,
    tolerence: usize,
    bktree: State<SpellTree<String>>,
) -> status::Accepted<String> {
    let (_exact, close) = bktree.find(&word, tolerence);
    status::Accepted(Some(format!("{:?}", close)))
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
        .mount("/", routes![index])
        .launch();
}
