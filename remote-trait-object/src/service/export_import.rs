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

use super::Dispatch;
use super::*;
use parking_lot::RwLock;
use std::sync::Arc;

// These traits are associated with some specific service trait.
// These tratis will be implement by `dyn ServiceTrait` where `T = dyn ServiceTrait` as well.
// Macro will implement this trait with the target(expanding) service trait.

pub trait ToDispatcher {
    fn to_dispatcher(self) -> Arc<dyn Dispatch>;
}

pub trait FromBox {
    fn from_box(self: Box<Self>) -> Arc<dyn Dispatch>;
}

pub trait FromArc {
    fn from_arc(a: Arc<Self>) -> Arc<dyn Dispatch>;
}

pub trait FromArcRwlock {
    fn from_arc_rwlock(a: Arc<RwLock<Self>>) -> Arc<dyn Dispatch>;
}

impl<T: FromBox + ?Sized> ToDispatcher for Box<T> {
    fn to_dispatcher(self) -> Arc<dyn Dispatch> {
        self.from_box()
    }
}

impl<T: FromArc + ?Sized> ToDispatcher for Arc<T> {
    fn to_dispatcher(self) -> Arc<dyn Dispatch> {
        FromArc::from_arc(self)
    }
}

impl<T: FromArcRwlock + ?Sized> ToDispatcher for Arc<RwLock<T>> {
    fn to_dispatcher(self) -> Arc<dyn Dispatch> {
        FromArcRwlock::from_arc_rwlock(self)
    }
}

/// Unused T is for avoiding violation of the orphan rule, like `ToDispatcher`
pub trait ToRemote<T: ?Sized + Service> {
    fn to_remote(port: Weak<dyn Port>, handle: HandleToExchange) -> Self;
}

// These functions are utilities for the generic traits above
pub fn export_service(context: &crate::context::Context, service: impl ToDispatcher) -> HandleToExchange {
    context.get_port().upgrade().unwrap().register(service.to_dispatcher())
}

pub fn import_service_box<T: ?Sized + Service>(context: &crate::context::Context, handle: HandleToExchange) -> Box<T>
where
    Box<T>: ToRemote<T>, {
    <Box<T> as ToRemote<T>>::to_remote(context.get_port(), handle)
}

pub fn import_service_arc<T: ?Sized + Service>(context: &crate::context::Context, handle: HandleToExchange) -> Arc<T>
where
    Arc<T>: ToRemote<T>, {
    <Arc<T> as ToRemote<T>>::to_remote(context.get_port(), handle)
}

pub fn import_service_rwlock<T: ?Sized + Service>(
    context: &crate::context::Context,
    handle: HandleToExchange,
) -> Arc<RwLock<T>>
where
    Arc<RwLock<T>>: ToRemote<T>, {
    <Arc<RwLock<T>> as ToRemote<T>>::to_remote(context.get_port(), handle)
}
