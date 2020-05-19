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

extern crate codechain_basesandbox as cbsb;

mod context;
mod service;
pub mod impl_prelude;
mod port;
pub mod queue;
pub mod service_prelude;
pub mod statistics;

pub use context::{
    global, termination, single_process_support::get_key, single_process_support::set_key, FmlConfig, InstanceKey, PortTable,
};
pub use service::id::{setup_identifiers, IdMap};
pub use service::SArc;
pub use service::{
    dispatch::PortDispatcher, dispatch::ServiceDispatcher, HandleInstance, MethodId, Service, ServiceObjectId, TraitId,
};
pub use port::{PacketHeader, Port, PortId};
