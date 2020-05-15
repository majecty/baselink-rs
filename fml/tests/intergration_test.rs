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

pub use fml::impl_prelude::*;
pub use fml::service_prelude::*;

#[fml_macro::service(service_env_test, a)]
pub trait TestService: fml::Service {
    /// Make an invitation for a single visit toward itself
    fn fn1(&self, a1: String, a2: u8) -> SBox<dyn TestService>;

    /// Returns name of the next module to visit
    fn fn2(&self, a2: Vec<(u8, String)>) -> String;
}

#[fml_macro::service_impl(impl_env, TestService)]
pub struct TestImpl {
    pub handle: fml::HandleInstance,
}

impl TestService for TestImpl {
    fn fn1(&self, a1: String, a2: u8) -> SBox<dyn TestService> {
        SBox::new(Box::new(TestImpl {
            handle: Default::default(),
        }))
    }

    fn fn2(&self, a2: Vec<(u8, String)>) -> String {
        "Hello".to_owned()
    }
}

#[test]
fn service_1() {
    let s = <dyn TestService as service_env::ImportService<dyn TestService>>::import(
        Default::default()
    );
    let x = s.fn1("qwe".to_owned(), 1).unwrap();
    x.fn2(Default::default());
}
