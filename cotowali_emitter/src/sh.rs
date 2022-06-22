// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Write;

use crate::Emitter;
use cotowali_ast::{Expr, File, Stmt};
use cotowali_utils::must;

pub struct ShEmitter {
    out: String,
}

impl ShEmitter {
    pub fn new() -> Self {
        Self {
            out: String::with_capacity(1024),
        }
    }

    fn write(&mut self, s: &str) {
        must!(self.out.write_str(s))
    }

    fn writeln(&mut self, s: &str) {
        self.write(s);
        self.ln();
    }
    fn ln(&mut self) {
        self.write("\n");
    }

    fn file(&mut self, file: &File) {
        self.writeln("#!/bin/sh");
        for stmt in file.stmts.iter() {
            self.stmt(stmt);
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                self.write(r"printf '%s\n' ");
                self.expr(expr);
                self.writeln("");
            }
        }
    }

    fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLiteral(n) => self.write(&n.to_string()),
        }
    }
}

impl Emitter for ShEmitter {
    fn emit(&mut self, file: &File) -> String {
        self.file(file);
        std::mem::replace(&mut self.out, Default::default())
    }
}
