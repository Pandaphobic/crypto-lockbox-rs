use include_crypt::{include_crypt, EncryptedFile};

static PASSWORD: EncryptedFile = include_crypt!("password");
static SEED: EncryptedFile = include_crypt!("seedphrase");
static WALLET: EncryptedFile = include_crypt!("wallet");
static PRIVATE_KEY: EncryptedFile = include_crypt!("private_key");

fn main() {

    // Load Password File
    let decrypted_passfile = PASSWORD.decrypt_str();
    let decrypted_pass = match decrypted_passfile {
        Ok(string) => string,
        Err(error) => panic!("Problem opening password file: {:?}", error),
    };
    // Remove \n from the end of password
    let password = decrypted_pass.trim(); 

    println!("This files is encrypted! Enter your password to continue. \n");
    // Prompt for a password on TTY    
    let input_pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
       
    // Check Password
    if input_pass == password {        
        
        // Dectrypt Seed
        let decrypted_seed = SEED.decrypt_str();
        let decrypted_seed = match decrypted_seed {
            Ok(string) => string,
            Err(error) => panic!("Problem opening the seedphrase file: {:?}", error),
        };
        
        // Dectrypt Wallet
        let decrypted_wallet = WALLET.decrypt_str();
        let decrypted_wallet = match decrypted_wallet {
            Ok(string) => string,
            Err(error) => panic!("Problem opening the wallet file: {:?}", error),
        };

        // Dectrypt Private Key
        let decrypted_private_key = PRIVATE_KEY.decrypt_str();
        let decrypted_private_key = match decrypted_private_key {
            Ok(string) => string,
            Err(error) => panic!("Problem opening the wallet file: {:?}", error),
        };

        println!("Password Accepted! \n");
        println!("Wallet Public Address:");
        println!("{:?}\n", decrypted_wallet.trim());
        println!("Secret Phrase is:");
        println!("{:?}\n", decrypted_seed.trim());
        println!("Private Key is: ");
        println!("{:?}\n", decrypted_private_key.trim());         

    } else {
        println!("Password Not Accepted");
    }

}
