use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Poll;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePoll { question: String },
    Vote { question: String, choice: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetPollResponse)]
    GetPoll { question: String },
}

#[cw_serde]
pub struct GetPollResponse {
    pub poll: Option<Poll>,
}
