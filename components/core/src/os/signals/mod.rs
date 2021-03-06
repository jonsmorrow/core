// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Contains the cross-platform signal behavior.
// If signal handling ever becomes part of the rust stdlib, consider removing
// our homespun implementation. Check for status of that here:
// https://github.com/rust-lang/rfcs/issues/1368

use os::process;

#[allow(dead_code)]
pub enum SignalEvent {
    Shutdown,
    WaitForChild,
    Passthrough(process::Signal),
}

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use self::unix::{check_for_signal, init};

#[cfg(windows)]
pub use self::windows::{check_for_signal, init};
