// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod sh;
pub use sh::*;

use cotowali_ast::File;

pub trait Emitter {
    fn emit(&mut self, file: &File) -> String;
}

pub enum Emitters {
    Sh,
}

pub fn new_emitter(emitter: Emitters) -> impl Emitter {
    match emitter {
        Emitters::Sh => ShEmitter::new(),
    }
}

pub fn emit(emitter: Emitters, file: &File) -> String {
    new_emitter(emitter).emit(file)
}
