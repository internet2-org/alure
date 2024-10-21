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

use super::FieldInstr;
use crate::core::{Reg, RegA, SiteId};
use crate::isa::{ExecStep, Instruction};
use crate::{Core, Site};

macro_rules! A {
    [$reg:ident @ $core:ident] => {
        checked!($core.a($reg))
    };
    [$a:ident : $idx:ident @ $core:ident] => {{
        checked!($core.a(RegA::with($a, $idx.into())))
    }};
}

macro_rules! check {
    ($condition:expr) => {{
        if !($condition) {
            return ExecStep::NextFail;
        }
    }};
}

macro_rules! checked {
    ($core:ident . $op:ident($($arg:expr),*)) => {{
        let Some(val) = $core.$op( $( $arg ),* ) else {
            return ExecStep::NextFail;
        };
        val
    }};
}

impl<Id: SiteId> Instruction<Id> for FieldInstr {
    type Context<'ctx> = ();

    fn src_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn dst_regs(&self) -> BTreeSet<Reg> { todo!() }

    fn op_data_bytes(&self) -> u16 { todo!() }

    fn ext_data_bytes(&self) -> u16 { todo!() }

    fn complexity(&self) -> u64 {
        // Double the default complexity since each instruction performs two operations (and each arithmetic
        // operations is x10 of move operation).
        Instruction::<Id>::base_complexity(self) * 20
    }

    fn exec(&self, core: &mut Core<Id>, site: Site<Id>, context: &Self::Context<'_>) -> ExecStep<Site<Id>> {
        match *self {
            FieldInstr::IncMod { src_dst, val } => {
                let src = A![src_dst @ core];
                let val = val as u128;
                let res = checked!(core.add_mod(src, val));
                core.set_a(src_dst, res);
            }
            FieldInstr::DecMod { src_dst, val } => {
                let src = A![src_dst @ core];
                let val = val as u128;
                let val = checked!(core.neg_mod(val));
                let res = checked!(core.add_mod(src, val));
                core.set_a(src_dst, res);
            }
            FieldInstr::NegMod { src_dst } => {
                let src = A![src_dst @ core];
                let res = checked!(core.neg_mod(src));
                core.set_a(src_dst, res);
            }
            FieldInstr::AddMod { reg, dst, src1, src2 } => {
                let src1 = A![reg : src1 @ core];
                let src2 = A![reg : src2 @ core];
                let res = checked!(core.add_mod(src1, src2));
                core.set_a(RegA::with(reg, dst.into()), res);
            }
            FieldInstr::MulMod { reg, dst, src1, src2 } => {
                let src1 = A![reg : src1 @ core];
                let src2 = A![reg : src2 @ core];
                let res = checked!(core.mul_mod(src1, src2));
                core.set_a(RegA::with(reg, dst.into()), res);
            }
        }
        ExecStep::Next
    }
}
