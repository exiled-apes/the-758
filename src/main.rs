use gumdrop::Options;
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding};

#[derive(Debug, Options)]
struct AppOptions {
    #[options(
        help = "Root account (where the 6◎ was sent)",
        default_expr = "default_root_account()"
    )]
    root_pubkey: String,

    #[options(help = "Solana rpc server url", default_expr = "default_rpc_url()")]
    rpc_url: String,
    // TODO deleteme?
    // #[options(help = "Root tx", default_expr = "default_root_tx_signature()")]
    // root_tx_signature: String,
}

fn main() {
    eprintln!("Hello, apes!");

    let app_options = AppOptions::parse_args_default_or_exit();
    let rpc_client = RpcClient::new(app_options.rpc_url);

    let root_pubkey = app_options
        .root_pubkey
        .parse()
        .expect("Could not parse root account");
    println!("root_pubkey {:?}", root_pubkey);

    let root_account = rpc_client
        .get_account(&root_pubkey)
        .expect("Could not fetch root account");
    println!("root_account {:?}", root_account);

    let root_statuses = rpc_client
        .get_signatures_for_address(&root_pubkey)
        .expect("Could not fetch root signatures");

    for root_status in root_statuses.iter() {
        let tx_confirmation_status = root_status
            .to_owned()
            .confirmation_status
            .expect("Could not retrive confirmation status");

        if tx_confirmation_status != TransactionConfirmationStatus::Finalized {
            continue;
        }

        let block_time = root_status.block_time.expect("Could not fetch block_time");

        match tx_confirmation_status {
            solana_transaction_status::TransactionConfirmationStatus::Finalized => {
                let signature = root_status
                    .signature
                    .parse()
                    .expect("Could not parse signature");

                let encoded_confirmed_tx = rpc_client
                    .get_transaction(&signature, UiTransactionEncoding::Base58)
                    .expect("Could not fetch transaction");

                let encoded_tx_with_status_meta = encoded_confirmed_tx.transaction;

                let transaction = encoded_tx_with_status_meta
                    .transaction
                    .decode()
                    .expect("Could not decode transaction");

                // A note on current heuristics (subject to debugging & community validation).
                // True exiles have:
                // - 10 account_keys in their message.
                // - 66 log messages in their meta.
                // - 1 post token balance in their meta.

                let message = transaction.message();
                if message.account_keys.len() == 10usize {
                    let owner = message.account_keys[0];

                    let meta = encoded_tx_with_status_meta
                        .meta
                        .expect("Could not fetch meta");

                    if let Some(log_messages) = meta.clone().log_messages {
                        if let Some(post_token_balances) = meta.clone().post_token_balances {
                            if log_messages.len() == 66 && post_token_balances.len() == 1 {
                                for post_token_balance in post_token_balances.iter() {
                                    println!(
                                        "{} {} {}",
                                        block_time, owner, post_token_balance.mint
                                    );
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        };
    }
}

fn default_root_account() -> String {
    "5TBwDKpQhFjdcfNEyoBQeoUYstMf6MwYWeumTcLdW3Yp".to_owned()
}

fn default_rpc_url() -> String {
    "https://api.mainnet-beta.solana.com".to_owned()
}
