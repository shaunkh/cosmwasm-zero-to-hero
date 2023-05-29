#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetPollResponse, InstantiateMsg, QueryMsg};
use crate::state::{Config, Poll, CONFIG, POLLS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmwasm-zero-to-hero:shaunkh";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let validated_admin_address = deps.api.addr_validate(&msg.admin_address)?;
    let config = Config {
        admin_address: validated_admin_address,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll { question } => execute_create_poll(deps, env, info, question),
        ExecuteMsg::Vote { question, choice } => execute_vote(deps, env, info, question, choice),
    }
}

fn execute_create_poll(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    question: String,
) -> Result<Response, ContractError> {
    if POLLS.has(deps.storage, question.clone()) {
        return Err(ContractError::CustomError {
            val: "Key already taken!".to_string(),
        });
    }

    let poll = Poll {
        question: question.clone(),
        yes_votes: 0,
        no_votes: 0,
    };

    POLLS.save(deps.storage, question, &poll)?;

    Ok(Response::new().add_attribute("action", "create_poll"))
}

fn execute_vote(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    question: String,
    choice: String,
) -> Result<Response, ContractError> {
    if !POLLS.has(deps.storage, question.clone()) {
        return Err(ContractError::CustomError {
            val: "Poll not available!".to_string(),
        });
    } else if choice != "yes" && choice != "no" {
        return Err(ContractError::CustomError {
            val: "Invalid choice!".to_string(),
        });
    }

    let mut poll = POLLS.load(deps.storage, question.clone())?;
    poll.yes_votes += if choice == "yes" { 1 } else { 0 };
    poll.no_votes += if choice == "no" { 1 } else { 0 };

    POLLS.save(deps.storage, question, &poll)?;

    Ok(Response::new().add_attribute("action", "vote"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPoll { question } => query_get_poll(deps, _env, question),
    }
}

fn query_get_poll(deps: Deps, _env: Env, question: String) -> StdResult<Binary> {
    let poll = POLLS.may_load(deps.storage, question)?;
    to_binary(&GetPollResponse { poll })
}

#[cfg(test)]
mod tests {

    use std::vec;

    use cosmwasm_std::{
        attr, from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
    };

    use crate::{
        contract::{execute, query},
        msg::{ExecuteMsg, GetPollResponse, InstantiateMsg, QueryMsg},
        state::Poll,
    };

    use super::instantiate;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            admin_address: "admin".to_string(),
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes, vec![attr("action", "instantiate")]);
    }

    #[test]
    fn test_create_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            admin_address: "admin".to_string(),
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            question: "What is your favorite color?".to_string(),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes, vec![attr("action", "create_poll")]);

        let msg = QueryMsg::GetPoll {
            question: "What is your favorite color?".to_string(),
        };

        let res = query(deps.as_ref(), env.clone(), msg).unwrap();

        let get_poll_response: GetPollResponse = from_binary(&res).unwrap();
        assert_eq!(
            get_poll_response,
            GetPollResponse {
                poll: Some(Poll {
                    question: "What is your favorite color?".to_string(),
                    yes_votes: 0,
                    no_votes: 0,
                })
            }
        );

        let msg = ExecuteMsg::CreatePoll {
            question: "What is your favorite color?".to_string(),
        };

        let res = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }

    #[test]
    fn test_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            admin_address: "admin".to_string(),
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            question: "What is your favorite color?".to_string(),
        };

        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Vote {
            question: "What is your favorite color?".to_string(),
            choice: "yes".to_string(),
        };

        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(res.attributes, vec![attr("action", "vote")]);

        let msg = QueryMsg::GetPoll {
            question: "What is your favorite color?".to_string(),
        };

        let res = query(deps.as_ref(), env, msg).unwrap();

        let get_poll_response: GetPollResponse = from_binary(&res).unwrap();
        assert_eq!(
            get_poll_response,
            GetPollResponse {
                poll: Some(Poll {
                    question: "What is your favorite color?".to_string(),
                    yes_votes: 1,
                    no_votes: 0,
                })
            }
        );
    }
}
