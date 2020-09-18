# Password Helper
Basic Rust CLI to enable you to extract the characters from a string that make up your password.
The basic idea is if you have a randomly generated password such as `FRVEV44r4!3#` and was prompted
(perhaps by your bank) for character 1,3,4 you can run:
``passwordcli FRVEV44r4!3# 1,3,4 `` and you will receive `FVE` as a result.

# Installation
Right now I am not distributing the binaries directly. To get it running, make sure you have rust installed.
Then close this repo,cd into the directory and run
````
cargo install --path .
````

This builds and installs the binaries to `~/.cargo/bin` (atleast on mac). Add this folder to your path and you can run 
it form anywhere.

# Running tests
Simply Run `cargo test` from within the folder.

