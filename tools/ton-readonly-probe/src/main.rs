use anyhow::{Context, Result};
use std::{env, str::FromStr};
use tonlibjson_client::{
    address::AccountAddressData,
    ton::TonClientBuilder,
};

#[tokio::main]
async fn main() -> Result<()> {
    let address = env::var("TON_ACCOUNT_ADDRESS")
        .context("set TON_ACCOUNT_ADDRESS to a public TON address")?;

    // Validate raw or user-friendly TON address syntax before network access.
    let parsed = AccountAddressData::from_str(&address)
        .context("TON_ACCOUNT_ADDRESS is not a valid TON address")?;

    println!("mode=READ_ONLY");
    println!("network=TON_MAINNET");
    println!("workchain={}", parsed.chain_id);
    println!("address={}", parsed.to_raw_string());
    println!("stage=CONNECTING");

    let mut client = TonClientBuilder::default()
        .build()
        .context("failed to initialize TON lite-server client")?;

    client.ready().await.context("TON lite-server readiness check failed")?;
    println!("stage=FETCHING_ACCOUNT_STATE");

    let state = client
        .get_account_state(&address)
        .await
        .context("TON account-state request failed")?;

    println!("stage=ACCOUNT_STATE_RECEIVED");
    println!("account_state={state:#?}");
    println!("stage=COMPLETE");
    println!("signing=DISABLED");
    println!("message_submission=DISABLED");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_raw_mainnet_address() {
        let address = format!("0:{}", "00".repeat(32));
        assert!(AccountAddressData::from_str(&address).is_ok());
    }

    #[test]
    fn rejects_malformed_address() {
        assert!(AccountAddressData::from_str("not-a-ton-address").is_err());
    }
}
