use std::collections::BTreeMap;

use crate::binary::{invalid_data, Arena, Record};
use crate::error::UserNotificationsError;
use crate::ffi::abi::{UNDictionaryEntryRaw, UNSliceRaw, UNStringRaw, UNUserInfoRaw, UNValueRaw};

pub type NotificationUserInfo = BTreeMap<String, NotificationValue>;

#[derive(Debug, Clone)]
pub enum NotificationValue {
    Null,
    Bool(bool),
    Integer(i64),
    Unsigned(u64),
    Real(f64),
    String(String),
    Data(Vec<u8>),
    Date(f64),
    Array(Vec<Self>),
    Dictionary(NotificationUserInfo),
}

impl NotificationValue {
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

impl PartialEq for NotificationValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Unsigned(a), Self::Unsigned(b)) => a == b,
            (Self::Real(a), Self::Real(b)) | (Self::Date(a), Self::Date(b)) => {
                a.to_bits() == b.to_bits()
            }
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Data(a), Self::Data(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => a == b,
            (Self::Dictionary(a), Self::Dictionary(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for NotificationValue {}

impl From<String> for NotificationValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for NotificationValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<bool> for NotificationValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for NotificationValue {
    fn from(value: i32) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<i64> for NotificationValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<u32> for NotificationValue {
    fn from(value: u32) -> Self {
        Self::Unsigned(u64::from(value))
    }
}

impl From<u64> for NotificationValue {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<f32> for NotificationValue {
    fn from(value: f32) -> Self {
        Self::Real(f64::from(value))
    }
}

impl From<f64> for NotificationValue {
    fn from(value: f64) -> Self {
        Self::Real(value)
    }
}

impl Record for NotificationUserInfo {
    type Raw = UNUserInfoRaw;

    fn raw(&self, arena: &mut Arena) -> Self::Raw {
        let entries = self
            .iter()
            .map(|(key, value)| UNDictionaryEntryRaw {
                key: key.raw(arena),
                value: value.raw(arena),
            })
            .collect();
        UNUserInfoRaw {
            entries: arena.array(entries),
        }
    }

    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        unsafe { crate::binary::slice::<UNDictionaryEntryRaw>(raw.entries) }?
            .iter()
            .map(|entry| {
                Ok((unsafe { String::read(&entry.key) }?, unsafe {
                    NotificationValue::read(&entry.value)
                }?))
            })
            .collect()
    }
}

impl Record for NotificationValue {
    type Raw = UNValueRaw;

    fn raw(&self, arena: &mut Arena) -> Self::Raw {
        let mut raw = UNValueRaw {
            tag: 0,
            signed_value: 0,
            unsigned_value: 0,
            real_value: 0.0,
            string: UNStringRaw {
                data: core::ptr::null(),
                len: 0,
            },
            values: UNSliceRaw {
                data: core::ptr::null(),
                len: 0,
            },
            entries: UNSliceRaw {
                data: core::ptr::null(),
                len: 0,
            },
        };
        match self {
            Self::Null => {}
            Self::Bool(value) => {
                raw.tag = 1;
                raw.unsigned_value = u64::from(*value);
            }
            Self::Integer(value) => {
                raw.tag = 2;
                raw.signed_value = *value;
            }
            Self::Unsigned(value) => {
                raw.tag = 3;
                raw.unsigned_value = *value;
            }
            Self::Real(value) => {
                raw.tag = 4;
                raw.real_value = *value;
            }
            Self::String(value) => {
                raw.tag = 5;
                raw.string = value.raw(arena);
            }
            Self::Data(value) => {
                raw.tag = 6;
                raw.values = arena.array(value.clone());
            }
            Self::Date(value) => {
                raw.tag = 7;
                raw.real_value = *value;
            }
            Self::Array(value) => {
                raw.tag = 8;
                raw.values = value.raw(arena);
            }
            Self::Dictionary(value) => {
                raw.tag = 9;
                raw.entries = value.raw(arena).entries;
            }
        }
        raw
    }

    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(match raw.tag {
            0 => Self::Null,
            1 => Self::Bool(raw.unsigned_value != 0),
            2 => Self::Integer(raw.signed_value),
            3 => Self::Unsigned(raw.unsigned_value),
            4 => Self::Real(raw.real_value),
            5 => Self::String(unsafe { String::read(&raw.string) }?),
            6 => Self::Data(unsafe { crate::binary::slice::<u8>(raw.values) }?.to_vec()),
            7 => Self::Date(raw.real_value),
            8 => Self::Array(unsafe { Vec::<Self>::read(&raw.values) }?),
            9 => Self::Dictionary(unsafe {
                NotificationUserInfo::read(&UNUserInfoRaw {
                    entries: raw.entries,
                })
            }?),
            _ => return Err(invalid_data()),
        })
    }
}
