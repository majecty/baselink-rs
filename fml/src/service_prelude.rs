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

// This module provides required components in FML while expanding FML macro.

pub mod service_env {
    pub use crate::context::global;
    pub use crate::handle::dispatch::ServiceDispatcher;
    pub use crate::handle::id::{MID_REG, TID_REG};
    pub use crate::handle::service_context;
    pub use crate::handle::{DispatchService, ExportService, ImportService, SArc};
    pub use crate::handle::{HandleInstance, MethodId, MethodIdAtomic, Service, TraitId, TraitIdAtomic, ID_ORDERING};
    pub use crate::port::{PacketHeader, Port, PortId};
}

// we omit service_context
pub mod service_env_mock {
    pub use crate::context::global;
    pub use crate::handle::dispatch::ServiceDispatcher;
    pub use crate::handle::id::{MID_REG, TID_REG};
    pub use crate::handle::{DispatchService, ExportService, ImportService, SArc};
    pub use crate::handle::{HandleInstance, MethodId, MethodIdAtomic, Service, TraitId, TraitIdAtomic, ID_ORDERING};
    pub use crate::port::{PacketHeader, Port, PortId};
}

pub use intertrait::{cast::CastArc, Caster};
