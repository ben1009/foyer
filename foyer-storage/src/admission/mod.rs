//  Copyright 2023 MrCroxx
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use crate::{catalog::Catalog, metrics::Metrics};
use foyer_common::code::{Key, Value};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct AdmissionContext<K>
where
    K: Key,
{
    pub catalog: Arc<Catalog<K>>,
    pub metrics: Arc<Metrics>,
}

#[expect(unused_variables)]
pub trait AdmissionPolicy: Send + Sync + 'static + Debug {
    type Key: Key;
    type Value: Value;

    fn init(&self, context: AdmissionContext<Self::Key>) {}

    fn judge(&self, key: &Self::Key, weight: usize) -> bool;

    fn on_insert(&self, key: &Self::Key, weight: usize, judge: bool);

    fn on_drop(&self, key: &Self::Key, weight: usize, judge: bool);
}

pub mod rated_random;
pub mod rated_ticket;
