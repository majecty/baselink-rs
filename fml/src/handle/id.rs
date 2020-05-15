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

use super::{MethodId, TraitId};
use crate::context::{InstanceKey, INSTANCE_KEY_MAX};
use linkme::distributed_slice;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

// While you debug or test FML, there are two dangerous issues.
//
// 1. You don't build each module in a separated binary.
// Thus the linkme distributed_slice will be shared among modules.
// This is ok for two identifier registries,
// since they are just to embed integer identifier for various places.
// And luckily, this action would have been done in a same way anyway
// even if we build separately.
//
// 2. You might run single module multiple times concurrently.
// In this case each instance of module will share the global variables, even the OnceCell.
// There are two variables: Global context and checker flag of setup_identifiers.
// These two are managed along with InstanceKey, to distinguish which instance has accessed.
// See also

// linkme crate smartly collects all the registrations generated by the proc-macro
// into a sinlge array in the link time.
// Note that too long linkme-related variable name would cause serious compiler error in MacOS
// So we deliberately make it have a short name

// Id of traits, which represent services.
type TraitIdentifierSetter = fn(id: TraitId);
#[distributed_slice]
pub static TID_REG: [(&'static str, TraitIdentifierSetter)] = [..];

// Id of methods in services.
// Note that here the two strings mean (trait name, method name)
// Also you can skip calling this, then the method id will be set up for default value
// decided by the order of declaration.
type MethodIdentifierSetter = fn(id: MethodId);
#[distributed_slice]
pub static MID_REG: [(&'static str, &'static str, MethodIdentifierSetter)] = [..];

/// This will be provided by the coordinator.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct IdMap {
    // These two maps are system-wide; All module will get same ones
    pub trait_map: HashMap<String, TraitId>,
    pub method_map: HashMap<(String, String), MethodId>,
}

static ONCE_CHECK: OnceCell<Mutex<[bool; INSTANCE_KEY_MAX]>> = OnceCell::new();
/// This must be called only once during the entire lifetime of module instance.
/// If you build multiple instances into a single binary, it is ok to call multiple
/// times since the following global flag will smartly skip consequent calls
pub fn setup_identifiers(instance_key: InstanceKey, descriptor: &IdMap) {
    if ONCE_CHECK.get_or_init(|| Mutex::new([false; INSTANCE_KEY_MAX])).lock().unwrap()[instance_key as usize] {
        panic!("setup_identifiers() has been called multiple times!")
    }
    if ONCE_CHECK.get().unwrap().lock().unwrap().iter().any(|&x| x) {
        // You're ok to call this multiple times from different module, but not gonna re-setup.
        return
    }

    // distributed_slices integrity test
    {
        let mut bucket: HashSet<String> = HashSet::new();
        for (ident, _) in TID_REG {
            bucket.insert((*ident).to_owned());
        }
        assert_eq!(bucket.len(), TID_REG.len());
    }
    {
        let mut bucket: HashSet<(String, String)> = HashSet::new();
        for (ident1, ident2, _) in MID_REG {
            bucket.insert(((*ident1).to_owned(), (*ident2).to_owned()));
        }
        assert_eq!(bucket.len(), MID_REG.len());
    }

    for (trait_name, setter) in TID_REG {
        setter(*descriptor.trait_map.get(*trait_name).expect("Invalid handle descriptor"));
    }

    // method ids have default values decided by the order, so it is ok to leave them in an ordinary case.
    if !descriptor.method_map.is_empty() {
        for (trait_name, method_name, setter) in MID_REG {
            setter(
                *descriptor
                    .method_map
                    .get(&((*trait_name).to_owned(), (*method_name).to_owned()))
                    .expect("Invalid handle descriptor"),
            );
        }
    }

    ONCE_CHECK.get().unwrap().lock().unwrap()[instance_key as usize] = true;
}
