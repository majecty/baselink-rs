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

#![allow(clippy::mutex_atomic)]
// TODO: Remove this
#![allow(clippy::ptr_arg)]

extern crate codechain_basesandbox as cbsb;
extern crate codechain_fml as fml;
extern crate linkme;
#[macro_use]
extern crate intertrait;

#[cfg(test)]
mod key;
mod mod_hello;
mod mod_relayer;
mod mod_scheduler;
#[cfg(test)]
mod module;
mod services;
#[cfg(test)]
mod test1;
#[cfg(test)]
mod test2;

// main functions for binary modules
pub use mod_hello::main_like as mod_hello_main;
pub use mod_relayer::main_like as mod_relayer_main;
pub use mod_scheduler::main_like as mod_scheduler_main;
