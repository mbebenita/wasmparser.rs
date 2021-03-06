/* Copyright 2017 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! A simple event-driven library for parsing WebAssembly binary files
//! (or streams).
//!
//! The parser library reports events as they happend and only stores
//! parsing information for a brief period of time, making it very fast
//! and memory-efficient. The event-driven model, however, has some drawbacks.
//! If you need random access to the entire WebAssembly data-structure,
//! this is not the right library for you. You could however, build such
//! a data-structure using this library.

pub use parser::Parser;
pub use parser::ParserState;
pub use parser::SectionCode;
pub use parser::Operator;
pub use parser::Type;
pub use parser::CustomSectionKind;
pub use parser::NameType;
pub use parser::Naming;
pub use parser::LocalName;
pub use parser::NameEntry;
pub use parser::ExternalKind;
pub use parser::FuncType;
pub use parser::ResizableLimits;
pub use parser::TableType;
pub use parser::MemoryType;
pub use parser::GlobalType;
pub use parser::MemoryImmediate;
pub use parser::BrTable;
pub use parser::ImportSectionEntryType;
pub use parser::RelocType;
pub use parser::RelocEntry;
pub use parser::LinkingType;

mod parser;
mod tests;
