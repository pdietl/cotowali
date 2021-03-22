module scope

import vash.util { auto_id }

pub type ScopeObject = Var | TypeSymbol

pub struct Var {
pub:
	name string
	id  u64
pub mut:
	typ TypeSymbol
}

pub fn new_var(name string) Var {
	return Var {
		name: name
		typ: unknown_type
		id: auto_id()
	}
}

pub struct Scope {
mut:
	parent &Scope
	children []&Scope
	objects map[string]ScopeObject
}

pub fn new_global_scope() &Scope {
	return &Scope {
		parent: 0
	}
}

pub fn (s &Scope) parent() ?&Scope {
	if isnil(s.parent) {
		return none
	}
	return s.parent
}

pub fn (s &Scope) children() []&Scope {
	return s.children
}

pub fn (mut s Scope) create() &Scope {
	child := &Scope{ parent: s }
	s.children << child
	return child
}

pub fn (mut s Scope) register(object ScopeObject) ?ScopeObject {
	$if !prod {
		if object.id == 0 {
			dump(object)
			panic('object.id is not assigned')
		}
	}
	key := object.name
	if key in s.objects {
		return error('$key is exists')
	}
	s.objects[key] = object
	return object
}
