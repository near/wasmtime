(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 2
	i32.const 3
	call $add
	i32.const 5
	call $assert_eq_i32)
 (func $add (param $lhs i32) (param $rhs i32) (result i32)
 	(i32.add
		(local.get $lhs)
		(local.get $rhs)
	)
 )
 (export "main" (func $main)))
