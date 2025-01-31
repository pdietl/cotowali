// Copyright (c) 2021 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
module pwsh

import cotowali.ast { CallCommandExpr, CallExpr, FnDecl }

fn (mut e Emitter) call_command_expr(expr CallCommandExpr, opt ExprOpt) {
	e.write('$expr.command')
	for arg in expr.args {
		e.write(' ')
		/*
		if arg is ast.StringLiteral {
			if arg.is_glob() {
				e.string_literal_value(arg)
				continue
			}
		}
		*/
		e.expr(arg, paren: true)
	}
}

fn (mut e Emitter) call_expr(expr CallExpr, opt ExprOpt) {
	if expr.is_builtin_function_call(.read) {
		e.write('read \$input')
	} else {
		e.write('${e.ident_for(expr.func)}')
	}

	if receiver := expr.receiver() {
		e.write(' ')
		e.expr(receiver, paren: true)
	}

	for arg in expr.args {
		e.write(' ')
		e.expr(arg, paren: true)
	}
}

fn (mut e Emitter) fn_decl(node FnDecl) {
	if !node.has_body {
		return
	}

	old_fn := e.current_fn
	e.current_fn = &node
	defer {
		e.current_fn = old_fn
	}

	if node.is_test() {
		if !e.ctx.config.is_test {
			return
		}
		defer {
			e.stmt(ast.Expr(node.get_run_test_call_expr()))
		}
	}

	fn_info := node.function_info()

	e.write('function ${e.ident_for(node)}(')
	{
		for i, param in node.params {
			if i == node.params.len - 1 && fn_info.variadic {
				break
			}
			if i > 0 {
				e.write(', ')
			}
			e.write(e.pwsh_var(param))
			if default := param.default() {
				e.write(' = ')
				e.expr(default)
			}
		}
	}
	e.writeln(')')

	e.writeln('{')
	e.indent()
	{
		if fn_info.variadic {
			e.writeln('${e.pwsh_var(node.params.last())} = \$args')
		}

		if param := node.pipe_in_param() {
			val := if ast.Expr(param).type_symbol().resolved().kind() in [.tuple, .array] {
				r'@($input)'
			} else {
				r'@($input)[0]'
			}
			e.writeln('${e.pwsh_var(param)} = $val')
		}

		e.block(node.body)
	}
	e.unindent()
	e.writeln('}')
}
