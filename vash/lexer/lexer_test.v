module lexer

import vash.token { Token, TokenKind }
import vash.pos { Pos }

fn test(code string, tokens []Token) {
	lexer := new(path: '', code: code)
	mut i := 0
	for t1 in lexer {
		if !(i < tokens.len) {
			assert false
			return
		}
		t2 := tokens[i]
		assert t1 == t2
		i++
	}
}

fn ktest(code string, kinds []TokenKind) {
	lexer := new(path: '', code: code)
	mut i := 0
	for t1 in lexer {
		if !(i < kinds.len) {
			assert false
			return
		}
		k2 := kinds[i]
		assert t1.kind == k2
		i++
	}
}

fn t(kind TokenKind, text string) Token {
	return Token{kind, text, pos.new_none()}
}

fn test_lexer() {
	test(' 🐈__ a ', [
		// Pos{i, line, col, len, last_line, last_col}
		Token{.unknown, '🐈__', Pos{1, 1, 2, 6, 1, 4}},
		Token{.ident, 'a', Pos{8, 1, 6, 1, 1, 6}},
		Token{.eof, '', Pos{10, 1, 8, 1, 1, 8}},
	])

	ktest('f()', [.ident, .l_paren, .r_paren, .eof])

	test('\n\r\n\r', [
		t(.eol, '\n'),
		t(.eol, '\r\n'),
		t(.eol, '\r'),
		t(.eof, ''),
		t(.eof, ''),
	])

	test('(0 + 1 - 2)', [
		t(.l_paren, '('),
		t(.int_lit, '0'),
		t(.op_plus, '+'),
		t(.int_lit, '1'),
		t(.op_minus, '-'),
		t(.int_lit, '2'),
		t(.r_paren, ')'),
		t(.eof, ''),
	])
}

fn test_ident() {
	texts := ['a', 'a.a', './a.a', '/usr/local/bin/', '~/.bin/cmd.py']
	for text in texts {
		ktest(text, [.ident, .eof])
	}
}
