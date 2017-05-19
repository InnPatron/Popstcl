#![allow(dead_code)]
use super::internal::{Value, Permissions, EntryPermissions};

#[derive(Clone, Debug, PartialEq)]
pub struct EnvEntry {
    value: Value,
    permissions: EntryPermissions,
}

impl EnvEntry {
    pub fn new(value: Value, permissions: EntryPermissions) -> EnvEntry {
        EnvEntry {
            value: value,
            permissions: permissions,
        }
    }

    pub fn no_restrictions(val: Value) -> EnvEntry {
        EnvEntry {
            value: val,
            permissions: EntryPermissions::new()
                .internal_read()
                .internal_write()
                .external_read()
                .external_write(),
        }
    }

    pub fn external_pipe(val: Value) -> EnvEntry {
        EnvEntry {
            value: val,
            permissions: EntryPermissions::new()
                .internal_read()
                .external_write()
                .external_read(),
        }
    }

    pub fn internal_value(val: Value) -> EnvEntry {
        EnvEntry {
            value: val,
            permissions: EntryPermissions::new().internal_read().internal_write(),
        }
    }

    pub fn observable_internal(val: Value) -> EnvEntry {
        EnvEntry {
            value: val,
            permissions: EntryPermissions::new()
                .internal_read()
                .internal_write()
                .external_read(),
        }
    }

    pub fn readonly(val: Value) -> EnvEntry {
        EnvEntry {
            value: val,
            permissions: EntryPermissions::new().internal_read().external_read(),
        }
    }

    pub fn permissions(&self) -> &EntryPermissions {
        &self.permissions
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn clone_value(&self) -> Value {
        self.value.clone()
    }
}
