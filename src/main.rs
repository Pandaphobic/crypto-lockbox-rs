use include_crypt::{include_crypt, EncryptedFile};

static PASSWORD: EncryptedFile = include_crypt!("encrypted_data/password");
static SEED: EncryptedFile = include_crypt!("encrypted_data/seedphrase");
static WALLET: EncryptedFile = include_crypt!("encrypted_data/wallet");
static PRIVATE_KEY: EncryptedFile = include_crypt!("encrypted_data/private_key");

fn main() {
    // Load Password File
    let decrypted_passfile = PASSWORD.decrypt_str();
    let decrypted_pass = match decrypted_passfile {
        Ok(string) => string,
        Err(error) => panic!("Problem opening password file: {:?}", error),
    };
    // Remove \n from the end of password
    // println!("{}", decrypted_pass);
    let password = decrypted_pass.trim(); 

    println!("This files is encrypted! Enter your password to continue. \n");
    // Prompt for a password on TTY    
    let input_pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
       
    // Check Password
    if input_pass == password {        
        
        let decrypted_secret = Secret::decrypt();
        println!("{}", decrypted_secret.seed);
        println!("{}", decrypted_secret.wallet);
        println!("{}", decrypted_secret.private_key);

    } else {
        println!("Password Not Accepted");
    }
}

struct Secret {
    seed: String,
    wallet: String,
    private_key: String,
}

impl Secret {
    fn decrypt() -> Secret { 
        // Dectrypt Items
        let decrypted_seed = SEED.decrypt_str().unwrap(); 
        let decrypted_wallet = WALLET.decrypt_str().unwrap();
        let decrypted_private_key = PRIVATE_KEY.decrypt_str().unwrap();
        
        // Clone value into new Secret
        let seed = decrypted_seed.clone();
        let wallet = decrypted_wallet.clone();
        let private_key = decrypted_private_key.clone();
        // Return a decrypted Secret
        return Secret { seed, wallet, private_key};
    }
}
