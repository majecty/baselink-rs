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

use crate as fml;
use crate as codechain_fml;

pub mod mock;
mod service_env_test {
    pub use super::fml::service_prelude::service_env_mock::*;
    pub use super::mock as service_context;
}

use fml::impl_prelude::*;
use fml::service_prelude::*;
use fml::*;
use std::io::Cursor;
use std::sync::Arc;

fn distinct_handle(i: u16) -> HandleInstance {
    HandleInstance {
        id: ServiceObjectId {
            index: i,
        },
        port_id_exporter: i,
        port_id_importer: i,
    }
}

/*
#[fml_macro::service(service_env_test, a)]
pub trait TestService: fml::Service {
    /// Make an invitation for a single visit toward itself
    fn fn1(&self, a1: String, a2: &str, a3: &[u8]) -> SArc<dyn TestService>;

    /// Returns name of the next module to visit
    fn fn2(&self, a2: &u8) -> String;

    fn fn3(&self) -> String;
}
*/

pub trait TestService: fml::Service {
    /// Make an invitation for a single visit toward itself
    fn fn1(&self, a1: String, a2: &str, a3: &[u8]) -> SArc<dyn TestService>;
    /// Returns name of the next module to visit
    fn fn2(&self, a2: &u8) -> String;
    fn fn3(&self) -> String;
}
#[allow(non_upper_case_globals)]
static ID_TRAIT_TestService: service_env_test::TraitIdAtomic = service_env_test::TraitIdAtomic::new(0);
#[allow(non_upper_case_globals)]
#[distributed_slice(service_env_test::TID_REG)]
static ID_TRAIT_ENTRY_TestService: (&'static str, fn(id: service_env_test::TraitId)) =
    ("TestService", id_trait_setter_TestService);
#[allow(non_snake_case)]
fn id_trait_setter_TestService(id: service_env_test::TraitId) {
    ID_TRAIT_TestService.store(id, service_env_test::ID_ORDERING);
}
#[allow(non_upper_case_globals)]
static ID_METHOD_TestService_fn1: service_env_test::MethodIdAtomic = service_env_test::MethodIdAtomic::new(7);
#[distributed_slice(service_env_test::MID_REG)]
#[allow(non_upper_case_globals)]
static ID_METHOD_ENTRY_TestService_fn1: (&'static str, &'static str, fn(id: service_env_test::MethodId)) =
    ("TestService", "fn1", id_method_setter_TestService_fn1);
#[allow(non_snake_case)]
fn id_method_setter_TestService_fn1(id: service_env_test::MethodId) {
    ID_METHOD_TestService_fn1.store(id, service_env_test::ID_ORDERING);
}
#[allow(non_upper_case_globals)]
static ID_METHOD_TestService_fn2: service_env_test::MethodIdAtomic = service_env_test::MethodIdAtomic::new(8);
#[distributed_slice(service_env_test::MID_REG)]
#[allow(non_upper_case_globals)]
static ID_METHOD_ENTRY_TestService_fn2: (&'static str, &'static str, fn(id: service_env_test::MethodId)) =
    ("TestService", "fn2", id_method_setter_TestService_fn2);
#[allow(non_snake_case)]
fn id_method_setter_TestService_fn2(id: service_env_test::MethodId) {
    ID_METHOD_TestService_fn2.store(id, service_env_test::ID_ORDERING);
}
#[allow(non_upper_case_globals)]
static ID_METHOD_TestService_fn3: service_env_test::MethodIdAtomic = service_env_test::MethodIdAtomic::new(9);
#[distributed_slice(service_env_test::MID_REG)]
#[allow(non_upper_case_globals)]
static ID_METHOD_ENTRY_TestService_fn3: (&'static str, &'static str, fn(id: service_env_test::MethodId)) =
    ("TestService", "fn3", id_method_setter_TestService_fn3);
#[allow(non_snake_case)]
fn id_method_setter_TestService_fn3(id: service_env_test::MethodId) {
    ID_METHOD_TestService_fn3.store(id, service_env_test::ID_ORDERING);
}
impl service_env_test::ExportService<dyn TestService> for dyn TestService {
    fn export(
        port_id: service_env_test::PortId,
        handle: std::sync::Arc<dyn TestService>,
    ) -> service_env_test::HandleInstance {
        service_env_test::service_context::register(
            port_id,
            handle.cast::<dyn service_env_test::Service>().expect("Trait casting failed"),
        )
    }
}
impl service_env_test::DispatchService<dyn TestService> for dyn TestService {
    fn dispatch(
        object: &dyn TestService,
        method: service_env_test::MethodId,
        arguments: &[u8],
        return_buffer: std::io::Cursor<&mut Vec<u8>>,
    ) {
        if method == ID_METHOD_TestService_fn1.load(service_env_test::ID_ORDERING) {
            let (a1, a2, a3): (String, String, Vec<_>) =
                serde_cbor::from_reader(&arguments[std::mem::size_of::<service_env_test::PacketHeader>()..]).unwrap();
            let result = object.fn1(a1, &a2, &a3);
            serde_cbor::to_writer(return_buffer, &result).unwrap();
            return
        }
        if method == ID_METHOD_TestService_fn2.load(service_env_test::ID_ORDERING) {
            let (a1,): (u8,) =
                serde_cbor::from_reader(&arguments[std::mem::size_of::<service_env_test::PacketHeader>()..]).unwrap();
            let result = object.fn2(&a1);
            serde_cbor::to_writer(return_buffer, &result).unwrap();
            return
        }
        if method == ID_METHOD_TestService_fn3.load(service_env_test::ID_ORDERING) {
            let (): () =
                serde_cbor::from_reader(&arguments[std::mem::size_of::<service_env_test::PacketHeader>()..]).unwrap();
            let result = object.fn3();
            serde_cbor::to_writer(return_buffer, &result).unwrap();
            return
        }
        panic!("Invalid handle call. Fatal Error.")
    }
}
impl service_env_test::IdOfService<dyn TestService> for dyn TestService {
    fn id() -> service_env_test::TraitId {
        ID_TRAIT_TestService.load(service_env_test::ID_ORDERING)
    }
}
#[derive(Debug)]
pub struct TestServiceImported {
    handle: service_env_test::HandleInstance,
}
impl TestService for TestServiceImported {
    fn fn1(&self, a1: String, a2: &str, a3: &[u8]) -> SArc<dyn TestService> {
        service_env_test::service_context::call(
            &self.handle,
            ID_METHOD_TestService_fn1.load(service_env_test::ID_ORDERING),
            &(a1, a2, a3),
        )
    }
    fn fn2(&self, a2: &u8) -> String {
        service_env_test::service_context::call(
            &self.handle,
            ID_METHOD_TestService_fn2.load(service_env_test::ID_ORDERING),
            &(a2,),
        )
    }
    fn fn3(&self) -> String {
        service_env_test::service_context::call(
            &self.handle,
            ID_METHOD_TestService_fn3.load(service_env_test::ID_ORDERING),
            &(),
        )
    }
}
impl service_env_test::Service for TestServiceImported {
    fn get_handle(&self) -> &service_env_test::HandleInstance {
        &self.handle
    }
    fn get_handle_mut(&mut self) -> &mut service_env_test::HandleInstance {
        &mut self.handle
    }
    fn get_trait_id(&self) -> service_env_test::TraitId {
        ID_TRAIT_TestService.load(service_env_test::ID_ORDERING)
    }
}
impl Drop for TestServiceImported {
    fn drop(&mut self) {
        service_env_test::service_context::delete(&self.handle)
    }
}
impl service_env_test::ImportService<dyn TestService> for dyn TestService {
    fn import(handle: service_env_test::HandleInstance) -> std::sync::Arc<dyn TestService> {
        std::sync::Arc::new(TestServiceImported {
            handle,
        })
    }
}
impl service_env_test::ServiceDispatcher for TestServiceImported {
    fn dispatch(
        &self,
        _method: service_env_test::MethodId,
        _arguments: &[u8],
        _return_buffer: std::io::Cursor<&mut Vec<u8>>,
    ) {
        panic!()
    }
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

#[cast_to([sync])]
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
fn cast() {
    let object = Arc::new(TestImpl {
        handle: Default::default(),
        name: Default::default(),
    });
    let t1: Arc<dyn TestService> = object;
    let t2: Arc<dyn Service> = t1.cast().unwrap();
    let _: Arc<dyn TestService> = t2.cast().unwrap();
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
    fml::port::server::port_thread_local::set_key(777);

    let si = <dyn TestService as service_env::ImportService<dyn TestService>>::import(distinct_handle(1234));
    si.fn1("s1".to_owned(), "s2", &[3]);
    {
        let (op, handle, method, (a1, a2, a3)): (String, HandleInstance, MethodId, (String, String, Vec<u8>)) =
            serde_cbor::from_slice(&mock::pop_log()).unwrap();
        assert_eq!(op, "call");
        assert_eq!(handle, distinct_handle(1234));
        // This number '7' is very specific to macro implementation.
        assert_eq!(method, 7);
        assert_eq!(a1, "s1");
        assert_eq!(a2, "s2");
        assert_eq!(a3, &[3]);
    }

    let se: Arc<dyn TestService> = Arc::new(TestImpl {
        handle: distinct_handle(2345),
        name: "Hi".to_owned(),
    });
    let mut buffer: Vec<u8> = vec![0; std::mem::size_of::<PacketHeader>()];
    let cursor = {
        let mut c = Cursor::new(&mut buffer);
        c.set_position(std::mem::size_of::<PacketHeader>() as u64);
        c
    };
    let mut args: Vec<u8> = vec![0; std::mem::size_of::<PacketHeader>()];
    let cursor2 = {
        let mut c = Cursor::new(&mut args);
        c.set_position(std::mem::size_of::<PacketHeader>() as u64);
        c
    };
    serde_cbor::to_writer(cursor2, &("s1", "s2", &[3])).unwrap();

    service_dispatch!(TestService, &*se, 7, &args, cursor);
    {
        let _: HandleInstance = serde_cbor::from_slice(&buffer[std::mem::size_of::<PacketHeader>()..]).unwrap();
        let newly_exported: Arc<dyn TestService> = mock::pop_service_log().cast::<dyn TestService>().unwrap();
        assert_eq!(newly_exported.fn3(), "s1s21");
        let (op, port_id): (String, PortId) = serde_cbor::from_slice(&mock::pop_log()).unwrap();
        assert_eq!(op, "register");
        assert_eq!(port_id, 777);
    }

    drop(si);
    {
        let (op, handle): (String, HandleInstance) = serde_cbor::from_slice(&mock::pop_log()).unwrap();
        assert_eq!(op, "delete");
        assert_eq!(handle, distinct_handle(1234));
    }
}
