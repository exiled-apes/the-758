use futures::executor::block_on;
use gumdrop::Options;
use linereader::LineReader;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{account::ReadableAccount, message::Message, program_pack::Pack};
use solana_transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding};
use spl_token::state::Mint;
use spl_token_metadata::{state::Metadata, utils::try_from_slice_checked};

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

#[tokio::main]
async fn main() {
    eprintln!("Hello, apes!");

    let app_options = AppOptions::parse_args_default_or_exit();
    match app_options.clone().command {
        Some(command) => {
            match command {
                Command::ListMetadata(list_metadata_options) => {
                    block_on(list_metadata(app_options, list_metadata_options))
                }
                Command::ListExiles(list_exiles_options) => {
                    list_exiles(app_options, list_exiles_options)
                }
            };
        }
        None => todo!("implement a help command that prints usage, etc... (also eliminate the need for multiple matches :)"),
    }
}

async fn list_metadata(app_options: AppOptions, _list_metadata_options: ListMetadataOptions) {
    let rpc_client = RpcClient::new(app_options.rpc_url);

    let mut r = LineReader::new(std::io::stdin());
    while let Some(Ok(line)) = r.next_line() {
        let line = std::str::from_utf8(line).expect("Couldn't decode line!");
        let line: Vec<&str> = line.trim().split(' ').collect();

        let mint_address = line.get(1).expect("Couldn't extract mint address");

        let mint_pubkey = mint_address.parse().expect("Could not parse mint pubkey");

        let mint_account = rpc_client
            .get_account(&mint_pubkey)
            .expect("Could not fetch mint account");

        let mint =
            Mint::unpack_unchecked(&mint_account.data()).expect("Couldn't unpack mint state");

        let mint_authority = mint.mint_authority.expect("Missing mint authority");

        let mint_authority_txs = rpc_client
            .get_signatures_for_address(&mint_authority)
            .expect("could not fetch signatures for mint authority");

        // we expect the mint_authority to have participated in exactly 1 txn
        assert_eq!(mint_authority_txs.len(), 1);

        let ape_genesis_tx = mint_authority_txs.get(0).expect("Could not get genesis tx");

        let ape_genesis_sig = ape_genesis_tx
            .signature
            .parse()
            .expect("Could not parse signature");

        let ape_genesis_tx = rpc_client
            .get_transaction(&ape_genesis_sig, UiTransactionEncoding::Base58)
            .expect("Could not fetch transaction");

        let ape_genesis_tx = ape_genesis_tx.transaction;

        let ape_genesis_tx = ape_genesis_tx
            .transaction
            .decode()
            .expect("Could not decode transaction");

        let ape_genesis_msg: &Message = ape_genesis_tx.message();

        let creat_master_ed_ix = ape_genesis_msg
            .instructions
            .get(6)
            .expect("Could not get create master edition instruction");

        let metadata_account_idx = *creat_master_ed_ix
            .accounts
            .get(5)
            .expect("Could not get metadata account index");

        let metadata_pubkey = ape_genesis_msg
            .account_keys
            .get(metadata_account_idx as usize)
            .expect("Could not get metadata account");

        let metadata_account = rpc_client
            .get_account(metadata_pubkey)
            .expect("Could not fetch metadata account");

        // let metadata = Metadata::try_from_slice_checked(&metadata_account.data())
        //     .expect("Could not deserialzie metadata");
        let md: Metadata = try_from_slice_checked(
            metadata_account.data(),
            spl_token_metadata::state::Key::MetadataV1,
            spl_token_metadata::state::MAX_METADATA_LEN,
        )
        .expect("Could not deserialze metadata");

        let url = format!("{}", md.data.uri);
        if url.len() == 0 {
            println!("{} {:?}: {} {}", "{", mint_address, "null", "}");
        } else {
            let res = reqwest::get(url).await;
            if res.is_err() {
                println!("{} {:?}: {} {}", "{", mint_address, "null", "}");
            } else {
                let res = res.expect("Could not get response");
                let txt = res.text().await;
                if txt.is_err() {
                    println!("{} {:?}: {} {}", "{", mint_address, "null", "}");
                } else {
                    let txt = txt.expect("Could not parse metadata response body");
                    println!("{} {:?}: {} {}", "{", mint_address, txt, "}");
                }
            }
        }
    }
}

fn list_exiles(app_options: AppOptions, list_exiles_options: ListExilesOptions) {
    let rpc_client = RpcClient::new(app_options.rpc_url);

    let root_pubkey = list_exiles_options
        .root_pubkey
        .parse()
        .expect("Could not parse root account");
    eprintln!("root_pubkey {:?}", root_pubkey);

    let root_account = rpc_client
        .get_account(&root_pubkey)
        .expect("Could not fetch root account");
    eprintln!("root_account {:?}", root_account);

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
                    // let owner = message.account_keys[0];

                    let meta = encoded_tx_with_status_meta
                        .meta
                        .expect("Could not fetch meta");

                    if let Some(log_messages) = meta.clone().log_messages {
                        if let Some(post_token_balances) = meta.clone().post_token_balances {
                            if log_messages.len() == 66 && post_token_balances.len() == 1 {
                                for post_token_balance in post_token_balances.iter() {
                                    println!("{} {}", block_time, post_token_balance.mint);
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
