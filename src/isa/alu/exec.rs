// Reference rust implementation of AluVM (arithmetic logic unit virtual machine).
// To find more on AluVM please check <https://aluvm.org>
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2021-2024 by
//     Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2021-2024 UBIDECO Labs,
//     Laboratories for Distributed and Cognitive Computing, Switzerland.
//     All rights reserved.
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

use std::collections::BTreeSet;

use super::{CtrlInstr, RegInstr};
use crate::core::{Core, Reg, Site, SiteId};
use crate::isa::{ExecStep, Instr, Instruction, InstructionSet, ReservedInstr};

impl<Id: SiteId, Ext: InstructionSet<Id>> Instruction<Id> for Instr<Id, Ext> {
    type Context<'ctx> = ();

    fn src_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn dst_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn op_data_bytes(&self) -> u16 { todo!() }

    fn ext_data_bytes(&self) -> u16 { todo!() }

    fn exec(&self, core: &mut Core<Id>, site: Site<Id>, context: &Self::Context<'_>) -> ExecStep<Site<Id>> { todo!() }
}

impl<Id: SiteId> Instruction<Id> for ReservedInstr {
    type Context<'ctx> = ();

    fn src_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn dst_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn op_data_bytes(&self) -> u16 { todo!() }

    fn ext_data_bytes(&self) -> u16 { todo!() }

    fn exec(&self, core: &mut Core<Id>, site: Site<Id>, context: &Self::Context<'_>) -> ExecStep<Site<Id>> { todo!() }
}

impl<Id: SiteId> Instruction<Id> for CtrlInstr<Id> {
    type Context<'ctx> = ();

    fn src_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn dst_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn op_data_bytes(&self) -> u16 { todo!() }

    fn ext_data_bytes(&self) -> u16 { todo!() }

    fn exec(&self, core: &mut Core<Id>, site: Site<Id>, context: &Self::Context<'_>) -> ExecStep<Site<Id>> { todo!() }
}

impl<Id: SiteId> Instruction<Id> for RegInstr {
    type Context<'ctx> = ();

    fn src_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn dst_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn op_data_bytes(&self) -> u16 { todo!() }

    fn ext_data_bytes(&self) -> u16 { todo!() }

    fn exec(&self, core: &mut Core<Id>, site: Site<Id>, context: &Self::Context<'_>) -> ExecStep<Site<Id>> { todo!() }
}
