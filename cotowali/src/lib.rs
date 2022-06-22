// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{io::Write, process, rc::Rc};

use cotowali_emitter::{emit, Emitters};
use cotowali_error::ComposedResult;
use cotowali_lexer::tokenize;
use cotowali_parser::parse;
use cotowali_source::Source;

pub fn compile(source: Source) -> ComposedResult<String> {
    let s = Rc::new(source);
    let file = parse(tokenize(&s))?;
    Ok(emit(Emitters::Sh, &file))
}

pub fn run(source: Source) -> ComposedResult<process::Output> {
    let out = compile(source)?;

    let mut out_file = tempfile::Builder::new()
        .prefix("cotowali_run_")
        .suffix(".sh")
        .tempfile()
        .expect("failed to create tempfile");

    out_file
        .write_all(out.as_bytes())
        .expect("faild to write output");

    Ok(process::Command::new("sh")
        .args([out_file.path()])
        .output()
        .expect("failed to execute compiled binary"))
}
