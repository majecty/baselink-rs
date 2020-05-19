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

extern crate codechain_fml as fml;
extern crate fml_macro;
extern crate linkme;
#[macro_use]
extern crate intertrait;

pub mod mock;
mod service_env_test {
    pub use super::mock as service_context;
    pub use fml::service_prelude::service_env_mock::*;
}

use fml::impl_prelude::*;
use fml::service_prelude::*;
use fml::*;
use std::io::Cursor;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct F {}

#[fml_macro::service(service_env_test, a)]
pub trait TestService: fml::Service {
    /// Make an invitation for a single visit toward itself
    fn fn1(&self, a1: String, a2: &str, a3: &[u8]) -> SArc<dyn TestService>;

    /// Returns name of the next module to visit
    fn fn2(&self, a2: &u8) -> String;

    fn fn3(&self) -> String;
}

impl mock::TestDefault for SArc<dyn TestService> {
    fn default() -> Self {
        SArc::new(Arc::new(TestImpl {
            handle: Default::default(),
            name: Default::default(),
        }))
    }
}

impl mock::TestDefault for String {
    fn default() -> Self {
        "Default".to_owned()
    }
}

impl mock::TestDefault for () {
    fn default() -> Self {
        ()
    }
}

#[fml_macro::service_impl(impl_env, TestService)]
pub struct TestImpl {
    pub handle: fml::HandleInstance,
    pub name: String,
}

impl TestService for TestImpl {
    fn fn1(&self, a1: String, a2: &str, a3: &[u8]) -> SArc<dyn TestService> {
        SArc::new(Arc::new(TestImpl {
            handle: Default::default(),
            name: format!("{}{}{}", a1, a2, a3.len()),
        }))
    }

    fn fn2(&self, a2: &u8) -> String {
        format!("{}", a2)
    }

    fn fn3(&self) -> String {
        self.name.clone()
    }
}

#[test]
fn service_1() {
    mock::set_key(1);
    let s = <dyn TestService as service_env::ImportService<dyn TestService>>::import(Default::default());
    let x = s.fn1("qwe".to_owned(), "qweqwe", b"123").unwrap();
    x.fn2(&3);
}

#[test]
fn service_2() {
    mock::set_key(2);
    let si = <dyn TestService as service_env::ImportService<dyn TestService>>::import(Default::default());
    si.fn1("s1".to_owned(), "s2", &[3]);
    let (op, handle, method, (a1, a2, a3)): (String, HandleInstance, MethodId, (String, String, Vec<u8>)) = serde_cbor::from_slice(&mock::pop_log()).unwrap();
    assert_eq!(op, "call");
    // This number '7' is very specific to macro implementation.
    assert_eq!(method, 7);
    assert_eq!(a1, "a1");
    assert_eq!(a2, "a2");
    assert_eq!(a3, &[3]);

    let se: Arc<dyn TestService> = Arc::new(TestImpl{handle: Default::default(), name: "Hi".to_owned()});
    let mut buffer: Vec<u8> = vec![0; std::mem::size_of::<PacketHeader>()];
    let cursor = {
        let mut c = Cursor::new(&mut buffer);
        c.set_position(std::mem::size_of::<PacketHeader>() as u64);
        c
    };
    service_dispatch!(TestService, &*se, 7, &serde_cbor::to_vec(&("s1", "s2", &[3])).unwrap(), cursor);
    



    //let x = s.fn1("qwe".to_owned(), 1).unwrap();
}
