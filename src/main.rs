use gumdrop::Options;
use solana_client::rpc_client::RpcClient;

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
    eprintln!("root_pubkey {:?}", root_pubkey);

    let root_account = rpc_client
        .get_account(&root_pubkey)
        .expect("Could not fetch root account");
    eprintln!("root_account {:?}", root_account);

    let root_signatures = rpc_client
        .get_signatures_for_address(&root_pubkey)
        .expect("Could not fetch root signatures");

    eprintln!("root_signatures {:?}", root_signatures.len());
    for root_signature in root_signatures {
        eprintln!("root_signature {:?}", root_signature);
    }
}

fn default_root_account() -> String {
    "5TBwDKpQhFjdcfNEyoBQeoUYstMf6MwYWeumTcLdW3Yp".to_owned()
}

fn default_rpc_url() -> String {
    "https://api.mainnet-beta.solana.com".to_owned()
}

// fn _main() {
//     let app_options = AppOptions::parse_args_default_or_exit();
//     let rpc_url = app_options.rpc_url;
//     let rpc_client = RpcClient::new(rpc_url);

//     let root_tx_signature = app_options
//         .root_tx_signature
//         .parse()
//         .expect("Could not parse root tx signature");

//     let encoded_confirmed_root_tx = rpc_client
//         .get_transaction(&root_tx_signature, UiTransactionEncoding::JsonParsed)
//         .expect("Could not fetch root tx.");

//     eprintln!(
//         "ecnoded confirmed root tx: slot {}; block_time {:?}",
//         encoded_confirmed_root_tx.slot, encoded_confirmed_root_tx.block_time,
//     );

//     match encoded_confirmed_root_tx.transaction.transaction {
//         solana_transaction_status::EncodedTransaction::LegacyBinary(_) => todo!(),
//         solana_transaction_status::EncodedTransaction::Binary(_, _) => todo!(),
//         solana_transaction_status::EncodedTransaction::Json(ui_tx) => {
//             // ui_tx.signatures
//             match ui_tx.message {
//                 solana_transaction_status::UiMessage::Raw(_) => todo!(),
//                 solana_transaction_status::UiMessage::Parsed(ui_msg) => {
//                     // ui_msg.account_keys
//                     // ui_msg.instructions

//                     for i in ui_msg.account_keys {
//                         eprintln!("{:#?}", i);
//                     }
//                 }
//             };
//         }
//     };

//     // Ideas:
//     // print transaction signature
//     // print transaction result ("Success")
//     // print transaction timestamp
//     // print transaction confirmation status
//     // print transaction block
//     // print transaction recent blockhash
//     // print transaction fee
//     // print transaction log messages

//     // Current TODO:
//     // print each account
//     // encoded_confirmed_root_tx.transaction.transaction.

//     // let encoded_root_tx_with_status = encoded_confirmed_root_tx.transaction;
//     // println!("root_tx {:#?}", encoded_root_tx_with_status);
// }

// fn default_root_tx_signature() -> String {
//     "4zggZMvYNPj217dk3TbWkBjp523K8jzctRnfwyxbTTNqMNwMWfrUSRcNWEhEEqTo37TESCFKMg38z51RpKrUQxZe"
//         .to_owned()
// }
