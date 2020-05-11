#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use bktree_rs::*;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    query: String,
    distance: usize,
    limit: usize,
    count: usize,
    //TODO: order: String, order: "lexigraphical"
    results: Vec<HashMap<usize, Vec<String>>>,
}

impl Response {
    fn new(
        query: String,
        distance: usize,
        limit: usize,
        count: usize,
        results: Vec<HashMap<usize, Vec<String>>>,
    ) -> Self {
        Response {
            query,
            distance,
            limit,
            count,
            results,
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hi there!\nMake a get request to /words?query=<word (string)>&distance=<int>&limit=<int (0 for all matches)> to get started."
}

#[get("/words?<query>&<distance>&<limit>")]
fn get_word(
    query: String,
    distance: usize,
    limit: Option<usize>,
    bktree: State<SpellTree<String>>,
) -> Json<Response> {
    let mut count = 0;
    let mut words: Vec<HashMap<usize, Vec<String>>> = vec![];

    let limit = match limit {
        Some(num) => num,
        None => 0,
    };

    for i in 0..distance + 1 {
        if i == 0 {
            if count == limit && limit != 0 {
                break;
            }

            let (exact, _) = bktree.find(&query, 0);

            let mut hm: HashMap<usize, Vec<String>> = HashMap::new();
            let mut exact_words: Vec<String> = vec![];

            for word in exact {
                if count < limit || limit == 0 {
                    let word = word.to_string();
                    exact_words.push(word);

                    count += 1;
                }
            }

            hm.insert(0, exact_words);
            words.push(hm);
        } else {
            if count == limit && limit != 0 {
                break;
            }

            let (_, close) = bktree.find(&query, i);

            let mut hm: HashMap<usize, Vec<String>> = HashMap::new();
            let mut close_words: Vec<String> = vec![];

            for word in close {
                if count < limit || limit == 0 {
                    let word = word.to_string();
                    close_words.push(word);

                    count += 1;
                }
            }

            hm.insert(i, close_words);
            words.push(hm);
        }
    }

    // return the json response
    // Json(ExactCloseResponse::new(exact, close))
    Json(Response::new(query, distance, limit, count, words))
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
