![Current Version](https://img.shields.io/crates/v/phaktionz?label=Phaktionz)
![Release](https://img.shields.io/github/v/release/MKProj/Phaktionz-CLI?include_prereleases&label=Release)
  
![Stable](https://img.shields.io/github/workflow/status/MKProj/Phaktionz-CLI/Rust/stable?label=Stable)
![Beta](https://img.shields.io/github/workflow/status/MKProj/Phaktionz-CLI/Rust/beta?label=Beta)
![Edge](https://img.shields.io/github/workflow/status/MKProj/Phaktionz-CLI/Rust/edge?label=Edge)  

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
