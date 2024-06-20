// Copyright 2023 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(missing_docs, dead_code)]

use std::collections::HashSet;
use std::pin::Pin;

use futures::Stream;

use crate::backend::{BackendResult, CommitId, FileId};
use crate::content_hash::ContentHash;
use crate::repo_path::RepoPathBuf;

/// An individual copy source.
#[derive(ContentHash, Ord, PartialOrd, Hash, Debug, PartialEq, Eq, Clone)]
pub struct CopySource {
    /// The source path a target was copied from.
    ///
    /// It is not required that the source path is different than the target
    /// path. A custom backend may choose to represent 'rollbacks' as copies
    /// from a file unto itself, from a specific prior commit.
    path: RepoPathBuf,
    file: FileId,
    /// The source commit the target was copied from. If not specified, then the
    /// parent of the target commit is the source commit. Backends may use this
    /// field to implement 'integration' logic, where a source may be
    /// periodically merged into a target, similar to a branch, but the
    /// branching occurs at the file level rather than the repository level. It
    /// also follows naturally that any copy source targeted to a specific
    /// commit should avoid copy propagation on rebasing, which is desirable
    /// for 'fork' style copies.
    ///
    /// If specified, it is required that the commit id is an ancestor of the
    /// commit with which this copy source is associated.
    commit: Option<CommitId>,
}

#[derive(ContentHash, Debug, PartialEq, Eq, Clone)]
pub enum CopySources {
    Resolved(CopySource),
    Conflict(HashSet<CopySource>),
}

/// An individual copy event, from file A -> B.
pub struct CopyRecord {
    /// The destination of the copy, B.
    target: RepoPathBuf,
    /// The CommitId where the copy took place.
    id: CommitId,
    /// The source of the copy, A.
    sources: CopySources,
}

/// Backend options for fetching copy records.
pub struct CopyRecordOpts {
    // TODO: Probably something for git similarity detection
}

pub type CopyRecordStream = Pin<Box<dyn Stream<Item = BackendResult<CopyRecord>>>>;
