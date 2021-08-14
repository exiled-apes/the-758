use gumdrop::Options;
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::UiTransactionEncoding;

#[derive(Debug, Options)]
struct AppOptions {
    #[options(help = "Root tx", default_expr = "default_root_tx_signature()")]
    root_tx_signature: String,

    #[options(help = "Solana rpc server url", default_expr = "default_rpc_url()")]
    rpc_url: String,
}

fn main() {
    println!("Hello, apes!");

    let app_options = AppOptions::parse_args_default_or_exit();

    let rpc_url = app_options.rpc_url;
    let rpc_client = RpcClient::new(rpc_url);

    let root_tx_signature = app_options
        .root_tx_signature
        .parse()
        .expect("Could not parse root tx signature");

    let root_tx = rpc_client.get_transaction(&root_tx_signature, UiTransactionEncoding::JsonParsed);

    println!("root_tx {:#?}", root_tx);
}

fn default_rpc_url() -> String {
    "https://api.mainnet-beta.solana.com".to_owned()
}

fn default_root_tx_signature() -> String {
    "4zggZMvYNPj217dk3TbWkBjp523K8jzctRnfwyxbTTNqMNwMWfrUSRcNWEhEEqTo37TESCFKMg38z51RpKrUQxZe"
        .to_owned()
}
