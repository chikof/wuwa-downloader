use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicResponse {
    pub hash_cache_check_acc_switch: i64,
    pub default: Default,
    pub predownload_switch: i64,
    #[serde(rename = "RHIOptionSwitch")]
    pub rhioption_switch: i64,
    #[serde(rename = "RHIOptionList")]
    pub rhioption_list: Vec<RhioptionList>,
    pub resources_login: ResourcesLogin,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub cdn_list: Vec<CdnList>,
    pub changelog: Changelog,
    pub changelog_visible: i64,
    pub resources: String,
    pub resources_base_path: String,
    pub resources_diff: ResourcesDiff,
    pub resources_exclude_path: Vec<Value>,
    pub resources_exclude_path_need_update: Vec<Value>,
    pub sample_hash_switch: i64,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdnList {
    #[serde(rename = "K1")]
    pub k1: i64,
    #[serde(rename = "K2")]
    pub k2: i64,
    #[serde(rename = "P")]
    pub p: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Changelog {
    #[serde(rename = "zh-Hans")]
    pub zh_hans: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesDiff {
    pub current_game_info: CurrentGameInfo,
    pub previous_game_info: PreviousGameInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameInfo {
    pub file_name: String,
    pub md5: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousGameInfo {
    pub file_name: String,
    pub md5: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RhioptionList {
    pub cmd_option: String,
    pub is_show: i64,
    pub text: Text,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    #[serde(rename = "zh-Hans")]
    pub zh_hans: Option<String>,
    pub de: Option<String>,
    #[serde(rename = "zh-Hant")]
    pub zh_hant: Option<String>,
    pub ko: Option<String>,
    pub ja: Option<String>,
    pub en: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesLogin {
    pub host: String,
    pub login_switch: i64,
}

#[derive(Debug, Deserialize)]
pub struct ResourceResponse {
    pub resource: Vec<Resource>,
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    pub dest: String,
    pub md5: String,
}
