use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePoll { question: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
