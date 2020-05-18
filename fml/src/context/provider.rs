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

/// This macro generats necessary code to provide whatever type of user defined context
/// in a synchronized way especially for the single_process mode. Note that it is not
/// guaranteed to be safe to call set_context() or remove_context() concurrently. The only
/// safe case is calling get_context() concurrently. And also, &'static Context is not guaranteed to
/// be actually static, so call remove_context() carefully.
#[macro_export]
macro_rules! context_provider {
    ($context: ty) => {
        type Context = $context;
        #[cfg(feature = "single_process")]
        pub mod context_provider_mod {
            use super::*;
            use once_cell::sync::OnceCell;
            use std::collections::HashMap;
            use std::sync::RwLock;

            // We need to enclose the context in the Box so that it won't move.
            static POOL: OnceCell<RwLock<HashMap<codechain_fml::InstanceKey, Box<Context>>>> = OnceCell::new();
            pub fn get() -> &'static Context {
                let ptr: *const _ = &*POOL.get().unwrap().read().unwrap().get(&codechain_fml::get_key()).unwrap();
                // TODO: Read the related section in Rustonomicon and make sure that this is safe.
                unsafe { &*ptr }
            }
            pub fn set(ctx: Context) {
                POOL.get_or_init(|| Default::default());
                let mut pool = POOL.get().unwrap().write().unwrap();
                assert!(!pool.contains_key(&codechain_fml::get_key()));
                pool.insert(codechain_fml::get_key(), Box::new(ctx));
            }
            pub fn remove() {
                POOL.get().unwrap().write().unwrap().remove(&codechain_fml::get_key()).unwrap();
            }
        }

        #[cfg(not(feature = "single_process"))]
        pub mod context_provider_mod {
            use super::*;
            static mut CONTEXT: Option<Context> = None;

            pub fn get() -> &'static Context {
                unsafe { CONTEXT.as_ref().unwrap() }
            }

            pub fn set(ctx: Context) {
                unsafe {
                    CONTEXT.replace(ctx);
                }
            }

            pub fn remove() {
                unsafe {
                    CONTEXT.take().unwrap();
                }
            }
        }
    };
}
