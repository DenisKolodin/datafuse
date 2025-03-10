// Copyright 2020 Datafuse Labs.
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

#![feature(thread_local)]

#[cfg(test)]
mod runtime_test;

#[cfg(test)]
mod progress_test;

#[cfg(test)]
mod stoppable_test;

mod profiling;
mod progress;
mod runtime;
mod runtime_tracker;
mod shutdown_signal;
mod stop_handle;
mod stoppable;
mod thread;
mod uniq_id;

pub use profiling::Profiling;
pub use progress::Progress;
pub use progress::ProgressCallback;
pub use progress::ProgressValues;
pub use runtime::BlockingWait;
pub use runtime::Dropper;
pub use runtime::Runtime;
pub use runtime::TrySpawn;
pub use runtime_tracker::RuntimeTracker;
pub use runtime_tracker::ThreadTracker;
pub use shutdown_signal::signal_stream;
pub use shutdown_signal::SignalStream;
pub use shutdown_signal::SignalType;
pub use stop_handle::StopHandle;
pub use stoppable::Stoppable;
pub use thread::Thread;
pub use tokio;
pub use uniq_id::GlobalSequence;
pub use uniq_id::GlobalUniqName;
pub use uuid;
