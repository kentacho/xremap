use crate::config::key::deserialize_key;
use evdev::Key;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DurationMilliSeconds};
use std::time::Duration;

use super::keymap_action::{Actions, KeymapAction};

// Values in `modmap.remap`
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum ModmapAction {
    #[serde(deserialize_with = "deserialize_key")]
    Key(Key),
    MultiPurposeKey(MultiPurposeKey),
    PressReleaseKey(PressReleaseKey),
    StickyKey(StickyKey),
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct MultiPurposeKey {
    #[serde(deserialize_with = "deserialize_key")]
    pub held: Key,
    #[serde(deserialize_with = "deserialize_key")]
    pub alone: Key,
    #[serde_as(as = "DurationMilliSeconds")]
    #[serde(default = "default_alone_timeout", rename = "alone_timeout_millis")]
    pub alone_timeout: Duration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PressReleaseKey {
    #[serde(deserialize_with = "deserialize_actions")]
    pub press: Vec<KeymapAction>,
    #[serde(deserialize_with = "deserialize_actions")]
    pub release: Vec<KeymapAction>,
}

pub fn deserialize_actions<'de, D>(deserializer: D) -> Result<Vec<KeymapAction>, D::Error>
where
    D: Deserializer<'de>,
{
    let actions = Actions::deserialize(deserializer)?;
    return Ok(actions.into_vec());
}

fn default_alone_timeout() -> Duration {
    Duration::from_millis(1000)
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct StickyKey {
    #[serde(deserialize_with = "deserialize_key")]
    pub sticky: Key,
    #[serde_as(as = "DurationMilliSeconds")]
    #[serde(default = "default_sticky_timeout", rename = "sticky_timeout_millis")]
    pub sticky_timeout: Duration,
}

fn default_sticky_timeout() -> Duration {
    Duration::from_millis(500)
}
