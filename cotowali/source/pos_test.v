// Copyright (c) 2021 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
module source

fn test_pos() {
	p := Pos{
		i: 5
		len: 2
		line: 2
		last_line: 2
		col: 2
		last_col: 3
	}
	assert p.begin() == 5
	assert p.end() == 7

	assert pos(p) == p

	// auto set last_line and last_col
	assert pos(line: 2, col: 2, i: 5, len: 2) == p

	// auto set len to 1
	assert pos(Pos{}).len == 1

	// if multiline, don,t owerride last_col
	assert pos(i: 0, len: 3, line: 1, last_line: 2, col: 1, last_col: 1).last_col == 1

	s := new_source('path', 'code')
	p2 := Pos{
		...p
		source: s
	}
	assert s.new_pos(p) == p2
}

fn test_pos_extend() {
	p1 := pos(line: 1, col: 5, i: 4, len: 2)
	p2 := pos(line: 5, col: 1, i: 20, len: 3)
	result := pos(i: 4, len: 19, line: 1, last_line: 5, col: 5, last_col: 3)
	assert p1.merge(p2) == result
	assert p2.merge(p1) == result
}

fn test_include() {
	p := Pos{
		i: 1
		len: 8
		line: 1
		last_line: 3
		col: 2
		last_col: 3
	}
	assert p.includes(pos(i: 2, len: 1))
	assert p.includes(pos(line: 1, last_line: 2, col: 3))
	assert p.includes(pos(line: 2, last_line: 2, col: 5))
	assert p.includes(pos(line: 1, last_line: 1, col: 2))
	assert p.includes(pos(line: 3, last_line: 3, col: 3, last_col: 3))
	assert p.includes(pos(line: 1, last_line: 3, col: 2, last_col: 3))
	assert !p.includes(pos(i: 2, len: 10))
	assert !p.includes(pos(line: 1, col: 1))
	assert !p.includes(pos(line: 0))
	assert !p.includes(pos(line: 4))
	assert !p.includes(pos(line: 1, col: 1))
	assert !p.includes(pos(line: 2, last_line: 4))
	assert !p.includes(pos(line: 1, col: 1))
	assert !p.includes(pos(line: 2, last_line: 3, col: 1, last_col: 4))
	assert !p.includes(pos(line: 2, last_line: 3, col: 3, last_col: 4))
}

fn test_pos_none() {
	assert !(pos(i: 0).is_none())
	assert none_pos().is_none()
}
