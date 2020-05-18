// Copyright 2020 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Mock functions for the dispatcher / method stubs

extern crate codechain_fml as fml;

use fml::*;
use once_cell::sync::OnceCell;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

thread_local!(static INSTANCE_KEY: Cell<u32> = Cell::new(0));

pub fn set_key(key: u32) {
    INSTANCE_KEY.with(|k| {
        assert_eq!(k.get(), 0);
        k.set(key);
    })
}

fn get_key() -> u32 {
    INSTANCE_KEY.with(|k| {
        assert_ne!(k.get(), 0);
        k.get()
    })
}

static LOG: OnceCell<RwLock<HashMap<u32, String>>> = OnceCell::new();
fn log(s: String) {
    LOG.get_or_init(Default::default).write().unwrap().insert(get_key(), s);
}
pub fn get_log() -> String {
    LOG.get_or_init(Default::default).write().unwrap().remove(&get_key()).unwrap()
}

// To keep the
pub trait TestDefault {
    fn default() -> Self;
}

pub fn delete(port_id: PortId, handle: ServiceObjectId) {
    log(format!("DELETE/{}/{:?}", port_id, handle));
}
pub fn register(port_id: PortId, trait_id: TraitId, handle_to_register: Arc<dyn Service>) -> HandleInstance {
    log(format!("REGISTER/{}/{}/{:?}", port_id, trait_id, handle_to_register.get_handle()));
    Default::default()
}
pub fn call<S: serde::Serialize + std::fmt::Debug, D: serde::de::DeserializeOwned + TestDefault>(
    handle: &HandleInstance,
    method: MethodId,
    args: &S,
) -> D {
    log(format!("CALL/{:?}/{}/{:?}", handle, method, args));
    TestDefault::default()
}
pub fn delete_remote(handle: &HandleInstance) {
    log(format!("DELETE/{:?}", handle));
}
