pub mod programs;

#[cfg(test)]
mod tests {
    use solana_sdk::{message::Message, signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
    use solana_client::rpc_client::RpcClient; 
    use solana_program::{pubkey::Pubkey, system_instruction::transfer, system_program};
    use std::str::FromStr;
    use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs}; 


    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        // Create a new keypair 
        let kp = Keypair::new(); 
        println!("You've generated a new Solana wallet: ",);
        println!("");

        // Print the public key as a Base58 string
        println!("Public Key (Base58): {}", kp.pubkey().to_string());
        println!("");

        // Print the secret key as a byte array
        println!("Secret Key: {:?}", kp.secret().to_bytes());
        println!("");

        println!("To save your wallet, copy and paste the following into a JSON file:"); println!("{:?}", kp.to_bytes());
    }

    // #[test]
    // fn airdrop() {
    //     // Import our keypair 
    //     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    //     // Connected to Solana Devnet RPC Client 
    //     let client = RpcClient::new(RPC_URL);

    //     // We're going to claim 2 devnet SOL tokens (2 billion lamports) 
    //     match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) { 
    //         Ok(s) => { 
    //             println!("Success! Check out your TX here:"); 
    //             println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string()); 
    //         }, 
    //         Err(e) => println!("Oops, something went wrong: {}", e.to_string()) 
    //     }; 
    // }

    // #[test]
    // fn transfer_sol() {
    //     // Import our keypair 
    //     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    //     // Define our WBA public key 
    //     let to_pubkey = Pubkey::from_str("ENR1GVHkLh7gUA6GxFnmT6afSJyQXqntVkQWJscqdc7J").unwrap();

    //     // Create a Solana devnet connection 
    //     let rpc_client = RpcClient::new(RPC_URL);

    //     // Get recent blockhash
    //     let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    //     let transaction = Transaction::new_signed_with_payer(
    //         &[transfer( 
    //             &keypair.pubkey(), 
    //             &to_pubkey, 
    //             1_000_000 
    //         )], 
    //         Some(&keypair.pubkey()), 
    //         &vec![&keypair], 
    //         recent_blockhash 
    //     );

    //     // Send the transaction 
    //     let signature = rpc_client 
    //         .send_and_confirm_transaction(&transaction) 
    //         .expect("Failed to send transaction");

    //     // Print our transaction out 
    //     println!( 
    //         "Success! Check out your TX here: 
    //         https://explorer.solana.com/tx/{}/?cluster=devnet", 
    //         signature 
    //     ); 
    // }

    // #[test]
    // fn empty_wallet() {
    //     // Import our keypair 
    //     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    //     // Define our WBA public key 
    //     let to_pubkey = Pubkey::from_str("ENR1GVHkLh7gUA6GxFnmT6afSJyQXqntVkQWJscqdc7J").unwrap();

    //     // Create a Solana devnet connection 
    //     let rpc_client = RpcClient::new(RPC_URL);

    //     // Get recent blockhash
    //     let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    //     // Get balance of dev wallet 
    //     let balance = rpc_client
    //         .get_balance(&keypair.pubkey()) 
    //         .expect("Failed to get balance"); 

    //     // Create a test transaction to calculate fees 
    //     let message = Message::new_with_blockhash( 
    //         &[transfer( 
    //             &keypair.pubkey(), 
    //             &to_pubkey, 
    //             balance, 
    //         )], 
    //         Some(&keypair.pubkey()), 
    //         &recent_blockhash 
    //     );

    //     // Calculate exact fee rate to transfer entire SOL amount out of account minus fees 
    //     let fee = rpc_client 
    //         .get_fee_for_message(&message) 
    //         .expect("Failed to get fee calculator");

    //     // Deduct fee from lamports amount and create a TX with correct balance 
    //     let transaction = Transaction::new_signed_with_payer( 
    //         &[transfer( 
    //             &keypair.pubkey(), 
    //             &to_pubkey, 
    //             balance - fee, 
    //         )], 
    //         Some(&keypair.pubkey()), 
    //         &vec![&keypair], 
    //         recent_blockhash
    //     ); 

    //     // Send the transaction 
    //     let signature = rpc_client 
    //         .send_and_confirm_transaction(&transaction) 
    //         .expect("Failed to send transaction");

    //     // Print our transaction out 
    //     println!( 
    //         "Success! Check out your TX here: 
    //         https://explorer.solana.com/tx/{}/?cluster=devnet", 
    //         signature 
    //     );
    // }

    #[test]
    pub fn complete_prereq() {
        // Create a Solana devnet connection 
        let rpc_client = RpcClient::new(RPC_URL);

        // Import our signer wallet
        let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");

        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Define our instruction data 
        let args = CompleteArgs { 
            github: b"obinnafranklinduru".to_vec() 
        };

        // Get recent blockhash 
        let blockhash = rpc_client 
            .get_latest_blockhash() 
            .expect("Failed to get recent blockhash"); 

        // Now we can invoke the "complete" function 
        let transaction = WbaPrereqProgram::complete( 
            &[&signer.pubkey(), &prereq, &system_program::id()], 
            &args, 
            Some(&signer.pubkey()), 
            &[&signer], 
            blockhash 
        ); 

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature)
    }
}