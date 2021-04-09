![Current Version](https://img.shields.io/crates/v/phaktionz?label=Phaktionz)
![Release](https://img.shields.io/github/v/release/MKProj/Phaktionz-CLI?include_prereleases&label=Release)
# Phaktionz Command Line Interface

If you would like to install do the following:  

[**Make sure to install rust first!!!**](https://rustup.rs/)
```
$ cargo install phaktionz
```

Currently due to some issues, there is no built in command 
to run the Phaktionz TCG book locally, however it can be done 
with the following: 

```bash
# Clone the git repository
$ git clone https://github.com/MKProj/Phaktionz-Book.git
# Install MDBook
$ cargo install mdbook
# Run serve on the Book's directory
$ mdbook serve Phaktionz-Book
```
