
enum Blockchain {
    Bitcoin,
    Ethereum,
    Polkadot,
    Solana,
}



function chainToToken(chain: Blockchain) {
    switch (chain) {
        case Blockchain.Bitcoin: console.log("BTC"); 
        case Blockchain.Ethereum: console.log("ETH"); 
        case Blockchain.Polkadot: console.log("DOT"); 
    }
}

chainToToken(Blockchain.Ethereum);