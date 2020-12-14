use ethers::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //use std::convert::TryFrom;
    //let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let provider = Provider::new(Ws::connect("ws://localhost:8545").await?);

    let wallet = "000000000000000000000000000000000000000000000000000000616c696365"
        .parse::<LocalWallet>()?;
    let bank = SignerMiddleware::new(provider.clone(), wallet);
    println!("Bank address: {}", bank.address());

    let wallet = "0000000000000000000000000000000000000000000000000000000000000Ace"
        .parse::<LocalWallet>()?;
    let client = SignerMiddleware::new(provider.clone(), wallet);
    println!("Client address: {}", client.address());

    let b0 = provider.get_balance(bank.address(), None).await?;
    dbg!(b0);
    let b1 = provider.get_balance(client.address(), None).await?;
    dbg!(b1);

    let tx = TransactionRequest::new()
        .from(bank.address())
        .to(client.address())
        .value(1);
    dbg!(&tx);

    let tx_hash = bank.send_transaction(tx, None).await?;
    dbg!(tx_hash);

    println!("Pending transaction {}...", tx_hash);
    let receipt = bank.pending_transaction(tx_hash).await?;
    dbg!(&receipt);

    let tx = bank.get_transaction(receipt.transaction_hash).await?;
    dbg!(tx);

    Ok(())
}
