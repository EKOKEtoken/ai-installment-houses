mod metadata;
mod nft;

use std::path::PathBuf;

use argh::FromArgs;
use candid::{Decode, Encode, Nat, Principal};
use dip721_rs::{NftError, TokenIdentifier};
use ic_agent::agent::AgentBuilder;
use ic_agent::identity::BasicIdentity;
use metadata::Metadata;
use nft::Nft;

/// Minter tool for minting NFTs to the ledger canister
#[derive(FromArgs)]
pub struct Args {
    /// the canister id of the ledger canister
    #[argh(option)]
    canister_id: Principal,
    /// the csv file containing the metadata of the NFTs
    #[argh(option)]
    csv_file: PathBuf,
    /// the path to the identity file
    #[argh(option, short = 'i')]
    identity: PathBuf,
    /// the path to the image dir
    #[argh(option)]
    image_dir: PathBuf,
    /// the id of the token to mint
    #[argh(option)]
    token_id: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    let identity = BasicIdentity::from_pem_file(args.identity)?;
    let agent = AgentBuilder::default().with_identity(identity).build()?;

    // load database
    let metadata = Metadata::load_database(&args.csv_file)?;
    let token_metadata = metadata
        .get(&args.token_id)
        .ok_or_else(|| anyhow::anyhow!("Token not found"))?;
    if token_metadata.initial_owner.is_none() {
        anyhow::bail!("Token has no initial owner");
    }

    // get nft
    let nft = Nft::get_from_metadata(token_metadata.clone(), &args.image_dir)?;

    // mint nft
    let token_id = TokenIdentifier::from(args.token_id);
    let response = agent
        .update(&args.canister_id, "dip721_mint")
        .with_arg(Encode!(
            token_metadata.initial_owner.as_ref().unwrap(),
            &token_id,
            &nft.into_properties()
        )?)
        .call_and_wait()
        .await?;

    let result = Decode!(&response, Result<Nat, NftError>)?;
    if let Err(err) = result {
        anyhow::bail!("Error minting NFT: {err}");
    }

    println!("NFT minted successfully");

    Ok(())
}
