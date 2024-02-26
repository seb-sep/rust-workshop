
pub enum Blockchain {
    Bitcoin,
    Ethereum,
    Polkadot
}

fn chain_to_token(chain: &Blockchain) -> &str {
    match chain {
        Blockchain::Bitcoin => "BTC",
        Blockchain::Ethereum => "ETH",
        Blockchain::Polkadot => "DOT",
    }
}

pub enum WalletType {
    UTXO(String),
    Account(f64),
}

fn list_append(l: &mut Vec<WalletType>, wallet: WalletType) {
    l.push(wallet)
}

pub fn enum_fun() {
    let mut l: Vec<WalletType> = Vec::new();
    l.push(WalletType::Account(0.5));
    l.push(WalletType::UTXO(String::from("some transaction")));

    list_append(&mut l, WalletType::UTXO(String::from("transaction")));
}
