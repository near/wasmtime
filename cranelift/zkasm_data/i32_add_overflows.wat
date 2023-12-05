(module
 (import "env" "assert_eq" (func $assert_eq (param i64) (param i64)))
 (func $main
	i64.const 4294967295
	i64.const 1000
	i64.add
	i64.const 4294968295
	call $assert_eq)
 (start $main))
