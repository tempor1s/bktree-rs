# Rust BKTree

A simple BKTree written in rust. It can be used as a library in your own code or as an API.

## 🚀 Getting Started

### API

If you would like to use the api, you can get started by making a get request to the following link.

https://bktree-api.dev.benlafferty.me/words?query=test&distance=2&limit=0

This will make a request to the API, and give words within a "edit" distance of 2 with no limit. It auto-completes from the standard unix dictonary.

What the paramaters do is as followed.

```bash
Query: The word that you want auto-complete or suggestions for.
Distance: The amount of potential 'edits' to a word that the API will return.
Limit: If you only want x amount of words returned form the API instead of all of them.
```

### Library

Currently, the library is not published on on crates.io, but you can import it as a Github repository if you would like to use it.

Add the following to your Cargo.toml file.

```toml
[dependencies]
bktree-rs = { git = https://github.com/tempor1s/bktree-rs }
```

## Contributors

Anyone is welcome to contribute!

<table>
  <tr>
    <td align="center"><a href="https://github.com/tempor1s"><img src="https://avatars0.githubusercontent.com/u/29741401?s=460&u=1ca03db5bbb7046bab14f72b7d6e801b9b0ac6f0&v=4" width="75px;" alt="Ben Lafferty"/><br /><sub><b>Ben Lafferty</b></sub></a><br /><a href="https://github.com/tempor1s/msconsole/commits?author=tempor1s" title="Code">💻</a></td>
