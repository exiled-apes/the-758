use gumdrop::Options;

#[derive(Debug, Options)]
struct AppOptions {
    #[options(help = "Solana rpc server url", default_expr = "default_rpc_url()")]
    rpc_url: String,
}

fn main() {
    println!("Hello, apes!");

    let app_options = AppOptions::parse_args_default_or_exit();

    let rpc_url = app_options.rpc_url;

    println!("{}", rpc_url)
}

fn default_rpc_url() -> String {
    "https://api.mainnet-beta.solana.com".to_owned()
}
