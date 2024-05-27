Minigrep

Minified version of the command line search tool `grep` (globally search a regular expression and print). In the case of this project, the minified grep will search and return all the lines that contain the given search string.

In the current version, the project only accepts `.txt` files and you have the ability to do case-insensitive search. In the CLI, `query` refers to the search word and `file_path` is the path to the `.txt` file you want to search.

Install the project:
- Install Rust via official documentation (the default installation should include build and package manager).
- Clone the repo.
- Change directory to the project.
```bash
$ cd minigrep
```
- Run the project, where the query is the search word and the filepath is the path to the file.
```bash
$ cargo run -- query file_path
```
  - For case-insensitive search:
```bash
$ IGNORE_CASE=1 cargo run -- to sample.txt
```
  - Print the output to a `.txt` file.
```bash
cargo run -- query file_path > output_file_path.txt
```
