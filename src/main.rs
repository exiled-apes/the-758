use gumdrop::Options;
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding};

#[derive(Clone, Debug, Options)]
struct AppOptions {
    #[options(help = "Solana rpc server url", default_expr = "default_rpc_url()")]
    rpc_url: String,

    #[options(command)]
    command: Option<Command>,
}

#[derive(Clone, Debug, Options)]
struct ListMetadataOptions {
    #[options(free)]
    args: Vec<String>,
}

#[derive(Clone, Debug, Options)]
struct ListExilesOptions {
    #[options(
        help = "Root account (where the 6◎ was sent)",
        default_expr = "default_root_account()"
    )]
    root_pubkey: String,
}

#[derive(Clone, Debug, Options)]
enum Command {
    ListMetadata(ListMetadataOptions),
    ListExiles(ListExilesOptions),
}

fn main() {
    eprintln!("Hello, apes!");

    let app_options = AppOptions::parse_args_default_or_exit();
    match app_options.clone().command {
        Some(command) => {
            match command {
                Command::ListMetadata(list_metadata_options) => {
                    list_metadata(app_options, list_metadata_options)
                }
                Command::ListExiles(list_exiles_options) => {
                    list_exiles(app_options, list_exiles_options)
                }
            };
        }
        None => todo!("implement a help command that prints usage, etc... (also eliminate the need for multiple matches :)"),
    }
}

fn list_metadata(app_options: AppOptions, list_metadata_options: ListMetadataOptions) {
    let _ = app_options;

    for arg in list_metadata_options.args {
        // todo assume each arg is an transaction signature
        // traverse into the last transaction -- CreateMasterEdition,
        //  locate the metadata account #6
        //  fetch the account
        //  then: let metadata = Metadata::from_account_info(metadata_account_info)?;
        let _ = arg;
    }
}

fn list_exiles(app_options: AppOptions, list_exiles_options: ListExilesOptions) {
    let rpc_client = RpcClient::new(app_options.rpc_url);

    let root_pubkey = list_exiles_options
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
                // - 66 log messages in their meta.
                // - 1 post token balance in their meta.

                let message = transaction.message();
                if message.account_keys.len() > 1 {
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
