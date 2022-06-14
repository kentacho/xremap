use crate::config::key::deserialize_key;
use evdev::Key;
use serde::Deserialize;
use serde_with::{serde_as, DurationMilliSeconds};
use std::time::Duration;

// Values in `modmap.remap`
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum KeyAction {
    #[serde(deserialize_with = "deserialize_key")]
    Key(Key),
    MultiPurposeKey(MultiPurposeKey),
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
