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
#[macro_use]
mod provider;

use crate::port::Port;
use crate::port::PortId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FmlConfig {
    /// Number of inbound call handlers
    pub server_threads: usize,
    /// Maximum outbound call slots
    pub call_slots: usize,
}

/// The entire global context that is enough to make services function.
pub struct PortTable {
    pub config_fml: FmlConfig,
    /// (Counterparty module's name, Counterarty's port id, Actual port)
    /// TODO: Though it uses HashMap now, we can issue PortIds in a series from 0 to ...
    /// Thus it may be optimized to use plain array later.
    pub map: HashMap<PortId, (String, PortId, Port)>,
    /// If this is true, the host is trying to shutdown all the modules
    /// You won't request deletion of handle because it doesn't matter.
    pub no_drop: bool,
}

/*
impl PortTable {
    /// TODO: Remove this in favor of the LinkBootstrapping
    /// Find first occurence of given moudle Id and return corresponding PortId
    pub fn find(&self, id: &str) -> Result<PortId, ()> {
        Ok(*self.map.iter().find(|&(_, (config, ..))| config.id == id).ok_or(())?.0)
    }
}
*/

/// This manages thread-local keys for module instance discrimination
/// in the intra-process setup.
/// This instance key setup will happen always but
/// costly global context resolution will be optionally compiled only with the
/// --features "single_process".
/// Note that you must manually set this key before invoke any call if you created
/// threads during service handling
pub mod single_process_support {
    pub type InstanceKey = u32;
    use std::cell::Cell;
    thread_local!(static INSTANCE_KEY: Cell<InstanceKey> = Cell::new(0));

    pub fn set_key(key: InstanceKey) {
        INSTANCE_KEY.with(|k| {
            assert_eq!(k.get(), 0, "You must set the instance key on your thread");
            k.set(key);
        })
    }

    pub fn get_key() -> InstanceKey {
        INSTANCE_KEY.with(|k| {
            assert_ne!(k.get(), 0, "You must set the instance key on your thread");
            k.get()
        })
    }
}
pub use single_process_support::InstanceKey;

pub mod global {
    use super::*;
    use single_process_support as codechain_fml;

    context_provider! {RwLock<PortTable>}
    pub fn get() -> &'static Context {
        context_provider_mod::get()
    }

    pub fn set(ctx: Context) {
        context_provider_mod::set(ctx)
    }

    pub fn remove() {
        context_provider_mod::remove()
    }
}
