# crypto-lockbox-rs
Hide your sensitive crypto info in an encrypted password protected binary!

## Getting Started

1. Clone this repo to your machine
2. Set your values in each of the corresponding files in `encrypted_data`
3. Build
4. Delete everything except the binary file `target/release/crypto-locker`
5. run with `./cyrpto-locker` - enter your password and voila!

## How It Works

Each of the files in `encrypted_data` is hashed and then stored in the binary. The information contained in these files is only accessible from functions insde the binary. Due to it being stored as a hash, it *should* be impossible to view it with a standard bin explorer.

None of the original files are required after building. Simply test it out and if you can get it, keep the binfile and password safe. If every you need your info, run the bin in *any* terminal, enter the correct password and boom - super secret information decrypted!

