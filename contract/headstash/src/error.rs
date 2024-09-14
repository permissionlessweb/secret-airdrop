use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("This headstash contract has not been set as an eligible minter yet.")]
    HeadstashNotSnip120uMinter {},

    #[error("unauthorized")]
    Unauthorized {},
    
    #[error("serde_json error")]
    SerdeJSON(#[from] serde_json::Error),

    #[error("Contract got an unexpected Reply")]
    UnexpectedReply(),

    #[error("Duplicate snip120u were provided")]
    DuplicateSnip120u(),
}
