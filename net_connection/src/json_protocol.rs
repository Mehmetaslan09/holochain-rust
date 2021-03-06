//! This module provides a higher level interface to p2p / network messaging
//! basically handles serialization / deserialization from / to the core
//! protocol message types (NamedBinary and Json).

#![allow(non_snake_case)]

use serde_json;

use failure::Error;
use holochain_core_types::{cas::content::Address, error::HolochainError, json::JsonString};
use std::convert::TryFrom;

use super::protocol::Protocol;

/// Tuple holding all the info required for identifying a metadata.
/// (entry_address, attribute, content)
/// TODO: Content is supposed to be a HashString but for now its JSON because of how Core is
/// currently handling Meta.
pub type MetaTuple = (Address, String, serde_json::Value);
/// (entry_address, attribute)
pub type MetaKey = (Address, String);

fn get_default_state_id() -> String {
    "undefined".to_string()
}

fn get_default_state_bindings() -> Vec<String> {
    Vec::new()
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct StateData {
    pub state: String,
    #[serde(default = "get_default_state_id")]
    pub id: String,
    #[serde(default = "get_default_state_bindings")]
    pub bindings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct ConfigData {
    pub config: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct ConnectData {
    #[serde(rename = "address")]
    pub peer_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct PeerData {
    #[serde(rename = "agentId")]
    pub agent_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct MessageData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "_id")]
    pub request_id: String,

    #[serde(rename = "toAgentId")]
    pub to_agent_id: String,

    #[serde(rename = "fromAgentId")]
    pub from_agent_id: String,

    #[serde(rename = "data")]
    pub content: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct TrackDnaData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "agentId")]
    pub agent_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct SuccessResultData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "_id")]
    pub request_id: String,

    #[serde(rename = "toAgentId")]
    pub to_agent_id: String,

    #[serde(rename = "successInfo")]
    pub success_info: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct FailureResultData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "_id")]
    pub request_id: String,

    #[serde(rename = "toAgentId")]
    pub to_agent_id: String,

    #[serde(rename = "errorInfo")]
    pub error_info: serde_json::Value,
}

//--------------------------------------------------------------------------------------------------
// DHT Entry
//--------------------------------------------------------------------------------------------------

/// Drop some data request from own p2p-module
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct DropEntryData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "_id")]
    pub request_id: String,

    #[serde(rename = "dataAddress")]
    pub entry_address: Address,
}

/// Data Request from some other agent
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct FetchEntryData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "_id")]
    pub request_id: String,

    #[serde(rename = "requesterAgentId")]
    pub requester_agent_id: String,

    #[serde(rename = "address")]
    pub entry_address: Address,
}

/// Generic DHT data message
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson, Default)]
pub struct EntryData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "providerAgentId")]
    pub provider_agent_id: String,

    #[serde(rename = "address")]
    pub entry_address: Address,

    #[serde(rename = "content")]
    pub entry_content: serde_json::Value,
}

/// DHT data response from a request
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson, Default)]
pub struct FetchEntryResultData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "_id")]
    pub request_id: String,
    #[serde(rename = "requesterAgentId")]
    pub requester_agent_id: String,

    #[serde(rename = "providerAgentId")]
    pub provider_agent_id: String,
    #[serde(rename = "address")]
    pub entry_address: Address,
    #[serde(rename = "content")]
    pub entry_content: serde_json::Value,
}

//--------------------------------------------------------------------------------------------------
// DHT metadata
//--------------------------------------------------------------------------------------------------

/// Metadata Request from another agent
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct FetchMetaData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "_id")]
    pub request_id: String,

    #[serde(rename = "requesterAgentId")]
    pub requester_agent_id: String,

    #[serde(rename = "entryAddress")]
    pub entry_address: Address,

    pub attribute: String,
}

/// Generic DHT metadata message
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct DhtMetaData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,

    #[serde(rename = "providerAgentId")]
    pub provider_agent_id: String,

    #[serde(rename = "entryAddress")]
    pub entry_address: Address,

    pub attribute: String,
    // single string or list of hashs
    #[serde(rename = "contentList")]
    pub content_list: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct FetchMetaResultData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,
    #[serde(rename = "_id")]
    pub request_id: String,
    #[serde(rename = "requesterAgentId")]
    pub requester_agent_id: String,
    #[serde(rename = "providerAgentId")]
    pub provider_agent_id: String,
    #[serde(rename = "entryAddress")]
    pub entry_address: Address,
    pub attribute: String,
    // // List of (hash, content) pairs.
    // single string or list of hashs
    #[serde(rename = "contentList")]
    pub content_list: Vec<serde_json::Value>,
}

/// Drop some data request from own p2p-module
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct DropMetaData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,
    #[serde(rename = "_id")]
    pub request_id: String,

    #[serde(rename = "entryAddress")]
    pub entry_address: Address,

    pub attribute: String,
}

//--------------------------------------------------------------------------------------------------
// List (publish & hold)
//--------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct GetListData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,
    #[serde(rename = "_id")]
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct EntryListData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,
    #[serde(rename = "_id")]
    pub request_id: String,

    #[serde(rename = "entryAddressList")]
    pub entry_address_list: Vec<Address>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
pub struct MetaListData {
    #[serde(rename = "dnaAddress")]
    pub dna_address: Address,
    #[serde(rename = "_id")]
    pub request_id: String,

    // List of meta identifiers, a pair: (entry_address, attribute, hashed_content)
    #[serde(rename = "metaList")]
    pub meta_list: Vec<MetaTuple>,
}

//--------------------------------------------------------------------------------------------------
// JsonProtocol Enum
//--------------------------------------------------------------------------------------------------

/// Enum holding all message types that serialize as json in the 'hc-core <-> P2P network module' protocol.
/// There are 4 categories of messages:
///  - Command: An order from the local node to the p2p module. Local node expects a reponse. Starts with a verb.
///  - Handle-command: An order from the p2p module to the local node. The p2p module expects a response. Start withs 'Handle' followed by a verb.
///  - Result: A response to a Command. Starts with the name of the Command it responds to and ends with 'Result'.
///  - Notification: Notify that something happened. Not expecting any response. Ends with verb in past form, i.e. '-ed'.
/// Fetch = Request between node and the network (other nodes)
/// Get   = Request within a node between p2p module and core
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, DefaultJson)]
#[serde(tag = "method")]
pub enum JsonProtocol {
    // -- Generic responses -- //
    /// Success response to a request (any message with an _id field.)
    #[serde(rename = "successResult")]
    SuccessResult(SuccessResultData),
    /// Failure response to a request (any message with an _id field.)
    /// Can also be a response to a mal-formed request.
    #[serde(rename = "failureResult")]
    FailureResult(FailureResultData),

    // -- Connection -- //
    /// Order the p2p module to be part of the network of the specified DNA.
    #[serde(rename = "trackDna")]
    TrackDna(TrackDnaData),

    /// Order the p2p module to leave the network of the specified DNA.
    #[serde(rename = "untrackDna")]
    UntrackDna(TrackDnaData),

    /// Connect to the specified multiaddr
    #[serde(rename = "connect")]
    Connect(ConnectData),
    /// Notification of a connection from another peer.
    #[serde(rename = "peerConnected")]
    PeerConnected(PeerData),

    // -- Config (deprecated?) -- //
    /// Request the current state from the p2p module
    #[serde(rename = "requestState")]
    GetState,
    /// p2p module's current state response.
    #[serde(rename = "state")]
    GetStateResult(StateData),
    /// Request the default config from the p2p module
    #[serde(rename = "requestDefaultConfig")]
    GetDefaultConfig,
    /// the p2p module's default config response
    #[serde(rename = "defaultConfig")]
    GetDefaultConfigResult(ConfigData),
    /// Set the p2p config
    #[serde(rename = "setConfig")]
    SetConfig(ConfigData),

    // -- Direct Messaging -- //
    /// Send a message to another peer on the network
    #[serde(rename = "sendMessage")]
    SendMessage(MessageData),
    /// the response from a previous `SendMessage`
    #[serde(rename = "sendMessageResult")]
    SendMessageResult(MessageData),
    /// Request to handle a message another peer has sent us.
    #[serde(rename = "handleSendMessage")]
    HandleSendMessage(MessageData),
    /// Our response to a message from another peer.
    #[serde(rename = "handleSendMessageResult")]
    HandleSendMessageResult(MessageData),

    // -- Entry -- //
    /// Request data from the dht network
    #[serde(rename = "fetchEntry")]
    FetchEntry(FetchEntryData),
    /// Response from requesting dht data from the network
    #[serde(rename = "fetchEntryResult")]
    FetchEntryResult(FetchEntryResultData),
    /// Another node, or the network module itself is requesting data from us
    #[serde(rename = "handleFetchEntry")]
    HandleFetchEntry(FetchEntryData),
    /// Successful data response for a `HandleFetchDhtData` request
    #[serde(rename = "handleFetchEntryResult")]
    HandleFetchEntryResult(FetchEntryResultData),

    /// Publish data to the dht.
    #[serde(rename = "publishEntry")]
    PublishEntry(EntryData),
    /// Store data on a node's dht slice.
    #[serde(rename = "handleStoreEntry")]
    HandleStoreEntry(EntryData),
    #[serde(rename = "handleDropEntry")]
    HandleDropEntry(DropEntryData),

    // -- Meta -- //
    /// Request metadata from the dht
    #[serde(rename = "fetchMeta")]
    FetchMeta(FetchMetaData),
    /// Response by the network for our metadata request
    #[serde(rename = "fetchMetaResult")]
    FetchMetaResult(FetchMetaResultData),
    /// Another node, or the network module itself, is requesting data from us
    #[serde(rename = "handleFetchMeta")]
    HandleFetchMeta(FetchMetaData),
    /// Successful metadata response for a `HandleFetchMeta` request
    #[serde(rename = "handleFetchMetaResult")]
    HandleFetchMetaResult(FetchMetaResultData),

    /// Publish metadata to the dht.
    #[serde(rename = "publishMeta")]
    PublishMeta(DhtMetaData),
    /// Store metadata on a node's dht slice.
    #[serde(rename = "handleStoreMeta")]
    HandleStoreMeta(DhtMetaData),
    /// Drop metadata on a node's dht slice.
    #[serde(rename = "handleDropData")]
    HandleDropMeta(DropMetaData),

    // -- Entry lists -- //
    #[serde(rename = "handleGetPublishingEntryList")]
    HandleGetPublishingEntryList(GetListData),
    #[serde(rename = "handleGetPublishingEntryListResult")]
    HandleGetPublishingEntryListResult(EntryListData),

    #[serde(rename = "handleGetHoldingEntryList")]
    HandleGetHoldingEntryList(GetListData),
    #[serde(rename = "handleGetHoldingEntryListResult")]
    HandleGetHoldingEntryListResult(EntryListData),

    // -- Meta lists -- //
    #[serde(rename = "handleGetPublishingMetaList")]
    HandleGetPublishingMetaList(GetListData),
    #[serde(rename = "handleGetPublishingMetaListResult")]
    HandleGetPublishingMetaListResult(MetaListData),

    #[serde(rename = "handleGetHoldingMetaList")]
    HandleGetHoldingMetaList(GetListData),
    #[serde(rename = "handleGetHoldingMetaListResult")]
    HandleGetHoldingMetaListResult(MetaListData),
}

/// Conversions
impl<'a> TryFrom<&'a Protocol> for JsonProtocol {
    type Error = Error;
    fn try_from(p: &Protocol) -> Result<Self, Error> {
        if let Protocol::Json(json) = p {
            match JsonProtocol::try_from(json) {
                Ok(w) => {
                    return Ok(w);
                }
                Err(e) => bail!("{:?}", e),
            };
        }
        bail!("could not convert into JsonProtocol: {:?}", p);
    }
}

impl TryFrom<Protocol> for JsonProtocol {
    type Error = Error;
    fn try_from(p: Protocol) -> Result<Self, Error> {
        JsonProtocol::try_from(&p)
    }
}

impl<'a> From<&'a JsonProtocol> for Protocol {
    fn from(w: &JsonProtocol) -> Self {
        Protocol::Json(JsonString::from(w))
    }
}

impl From<JsonProtocol> for Protocol {
    fn from(w: JsonProtocol) -> Self {
        Protocol::from(&w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_convert {
        ($e:expr) => {
            let orig = $e;
            let p = Protocol::from(orig.clone());
            let w = JsonProtocol::try_from(p).unwrap();
            assert_eq!(orig, w);
        };
    }

    #[test]
    fn it_can_convert_GetState() {
        test_convert!(JsonProtocol::GetState);
    }

    #[test]
    fn it_can_convert_GetStateResult() {
        test_convert!(JsonProtocol::GetStateResult(StateData {
            state: "test_state".to_string(),
            id: "test_id".to_string(),
            bindings: vec!["test_binding".to_string()],
        }));
    }

    #[test]
    fn it_can_convert_funky_state() {
        let w = JsonProtocol::try_from(JsonString::from(
            r#"{
            "method": "state",
            "state": "test_state"
        }"#,
        ))
        .unwrap();
        if let JsonProtocol::GetStateResult(s) = w {
            assert_eq!("undefined", &s.id);
            assert_eq!(0, s.bindings.len());
        } else {
            panic!("bad enum type");
        }
    }

    #[test]
    fn it_can_convert_GetDefaultConfig() {
        test_convert!(JsonProtocol::GetDefaultConfig);
    }

    #[test]
    fn it_can_convert_GetDefaultConfigResult() {
        test_convert!(JsonProtocol::GetDefaultConfigResult(ConfigData {
            config: "test".to_string(),
        }));
    }

    #[test]
    fn it_can_convert_SetConfig() {
        test_convert!(JsonProtocol::SetConfig(ConfigData {
            config: "test".to_string(),
        }));
    }

    #[test]
    fn it_can_convert_Connect() {
        test_convert!(JsonProtocol::Connect(ConnectData {
            peer_address: "test".into(),
        }));
    }

    #[test]
    fn it_can_convert_PeerConnected() {
        test_convert!(JsonProtocol::PeerConnected(PeerData {
            agent_id: "test_id".to_string(),
        }));
    }

    #[test]
    fn it_can_convert_SendMessage() {
        test_convert!(JsonProtocol::SendMessage(MessageData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            to_agent_id: "test_to".to_string(),
            from_agent_id: "test_from".to_string(),
            content: json!("hello"),
        }));
    }

    #[test]
    fn it_can_convert_SendMessageResult() {
        test_convert!(JsonProtocol::SendMessageResult(MessageData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            to_agent_id: "test_to".to_string(),
            from_agent_id: "test_from".to_string(),
            content: json!("hello"),
        }));
    }

    #[test]
    fn it_can_convert_HandleSendMessage() {
        test_convert!(JsonProtocol::HandleSendMessage(MessageData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            to_agent_id: "test_to".to_string(),
            from_agent_id: "test_from".to_string(),
            content: json!("hello"),
        }));
    }

    #[test]
    fn it_can_convert_HandleSendMessageResult() {
        test_convert!(JsonProtocol::HandleSendMessageResult(MessageData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            to_agent_id: "test_to".to_string(),
            from_agent_id: "test_from".to_string(),
            content: json!("hello"),
        }));
    }

    #[test]
    fn it_can_convert_FetchEntry() {
        test_convert!(JsonProtocol::FetchEntry(FetchEntryData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            requester_agent_id: "test_to".to_string(),
            entry_address: "Hk42".into(),
        }));
    }
    #[test]
    fn it_can_convert_HandleFetchEntry() {
        test_convert!(JsonProtocol::HandleFetchEntry(FetchEntryData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            requester_agent_id: "test_to".to_string(),
            entry_address: "Hk42".into(),
        }));
    }
    #[test]
    fn it_can_convert_FetchEntryResult() {
        test_convert!(JsonProtocol::FetchEntryResult(FetchEntryResultData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            requester_agent_id: "test_to".to_string(),
            provider_agent_id: "test_from".to_string(),
            entry_address: "Hk42".into(),
            entry_content: json!("hello"),
        }));
    }
    #[test]
    fn it_can_convert_HandleFetchEntryResult() {
        test_convert!(JsonProtocol::HandleFetchEntryResult(FetchEntryResultData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            requester_agent_id: "test_to".to_string(),
            provider_agent_id: "test_from".to_string(),
            entry_address: "Hk42".into(),
            entry_content: json!("hello"),
        }));
    }
    #[test]
    fn it_can_convert_PublishEntry() {
        test_convert!(JsonProtocol::PublishEntry(EntryData {
            dna_address: "test_dna".into(),
            provider_agent_id: "test_from".to_string(),
            entry_address: "Hk42".into(),
            entry_content: json!("hello"),
        }));
    }
    #[test]
    fn it_can_convert_HandleStoreEntry() {
        test_convert!(JsonProtocol::HandleStoreEntry(EntryData {
            dna_address: "test_dna".into(),
            provider_agent_id: "test_from".to_string(),
            entry_address: "Hk42".into(),
            entry_content: json!("hello"),
        }));
    }
    #[test]
    fn it_can_convert_HandleDropEntry() {
        test_convert!(JsonProtocol::HandleDropEntry(DropEntryData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            entry_address: "Hk42".into(),
        }));
    }

    #[test]
    fn it_can_convert_FetchMeta() {
        test_convert!(JsonProtocol::FetchMeta(FetchMetaData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            requester_agent_id: "test_to".to_string(),
            entry_address: "Hk42".into(),
            attribute: "meta_attribute".to_string(),
        }));
    }
    #[test]
    fn it_can_convert_HandleFetchMeta() {
        test_convert!(JsonProtocol::HandleFetchMeta(FetchMetaData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            requester_agent_id: "test_to".to_string(),
            entry_address: "Hk42".into(),
            attribute: "meta_attribute".to_string(),
        }));
    }
    #[test]
    fn it_can_convert_FetchMetaResult() {
        test_convert!(JsonProtocol::FetchMetaResult(FetchMetaResultData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            requester_agent_id: "test_to".to_string(),
            provider_agent_id: "test_from".to_string(),
            entry_address: "Hk42".into(),
            attribute: "meta_attribute".to_string(),
            content_list: vec![json!("hello-meta")],
        }));
    }
    #[test]
    fn it_can_convert_HandleFetchMetaResult() {
        test_convert!(JsonProtocol::HandleFetchMetaResult(FetchMetaResultData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            requester_agent_id: "test_to".to_string(),
            provider_agent_id: "test_from".to_string(),
            entry_address: "Hk42".into(),
            attribute: "meta_attribute".to_string(),
            content_list: vec![json!("hello-meta")],
        }));
    }
    #[test]
    fn it_can_convert_PublishMeta() {
        test_convert!(JsonProtocol::PublishMeta(DhtMetaData {
            dna_address: "test_dna".into(),
            provider_agent_id: "test_from".to_string(),
            entry_address: "Hk42".into(),
            attribute: "meta_attribute".to_string(),
            content_list: vec![json!("hello-meta")],
        }));
    }
    #[test]
    fn it_can_convert_HandleStoreMeta() {
        test_convert!(JsonProtocol::HandleStoreMeta(DhtMetaData {
            dna_address: "test_dna".into(),
            provider_agent_id: "test_from".to_string(),
            entry_address: "Hk42".into(),
            attribute: "meta_attribute".to_string(),
            content_list: vec![json!("hello-meta")],
        }));
    }
    #[test]
    fn it_can_convert_HandleDropMeta() {
        test_convert!(JsonProtocol::HandleDropMeta(DropMetaData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
            entry_address: "Hk42".into(),
            attribute: "meta_attribute".to_string(),
        }));
    }

    #[test]
    fn it_can_convert_HandleGetPublishingEntryList() {
        test_convert!(JsonProtocol::HandleGetPublishingEntryList(GetListData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
        }));
    }
    #[test]
    fn it_can_convert_HandleGetPublishingEntryListResult() {
        test_convert!(JsonProtocol::HandleGetPublishingEntryListResult(
            EntryListData {
                dna_address: "test_dna".into(),
                request_id: "test_id".to_string(),
                entry_address_list: vec!["data1".into(), "data2".into()],
            }
        ));
    }
    #[test]
    fn it_can_convert_HandleGetHoldingEntryList() {
        test_convert!(JsonProtocol::HandleGetHoldingEntryList(GetListData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
        }));
    }
    #[test]
    fn it_can_convert_HandleGetHoldingEntryListResult() {
        test_convert!(JsonProtocol::HandleGetHoldingEntryListResult(
            EntryListData {
                dna_address: "test_dna".into(),
                request_id: "test_id".to_string(),
                entry_address_list: vec!["data1".into(), "data2".into()],
            }
        ));
    }

    #[test]
    fn it_can_convert_HandleGetPublishingMetaList() {
        test_convert!(JsonProtocol::HandleGetPublishingMetaList(GetListData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
        }));
    }
    #[test]
    fn it_can_convert_HandleGetPublishingMetaListResult() {
        test_convert!(JsonProtocol::HandleGetPublishingMetaListResult(
            MetaListData {
                dna_address: "test_dna".into(),
                request_id: "test_id".to_string(),
                meta_list: vec![
                    (
                        "data1".into(),
                        "meta_attribute".to_string(),
                        "entry_address".into()
                    ),
                    (
                        "data2".into(),
                        "meta_attribute".to_string(),
                        "other_entry_address".into()
                    )
                ],
            }
        ));
    }
    #[test]
    fn it_can_convert_HandleGetHoldingMetaList() {
        test_convert!(JsonProtocol::HandleGetHoldingMetaList(GetListData {
            dna_address: "test_dna".into(),
            request_id: "test_id".to_string(),
        }));
    }
    #[test]
    fn it_can_convert_HandleGetHoldingMetaListResult() {
        test_convert!(JsonProtocol::HandleGetHoldingMetaListResult(MetaListData {
            request_id: "test_id".to_string(),
            dna_address: "test_dna".into(),
            meta_list: vec![
                (
                    "data1".into(),
                    "meta_attribute".to_string(),
                    "entry_address".into()
                ),
                (
                    "data2".into(),
                    "meta_attribute".to_string(),
                    "other_entry_address".into()
                )
            ],
        }));
    }
}
