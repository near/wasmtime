(module
 (import "env" "assert_eq" (func $assert_eq (param i64) (param i64)))
 (func $main
	(local $counter i32)
	(local $fp i64)
	(local $f i64)
	;; Decrease counter, so the branch to $loop happens as long as $counter > 0.
	;; This allows to safe a comparison (e.g. $counter > 0) and instead just do
	;; (br_if $loop (local.get $counter)).
	(local.set $counter (i32.const 10000))
	(local.set $fp (i64.const 0))
	(local.set $f (i64.const 1))
	(loop $loop
	    (local.get $fp)
	    (local.get $f)
		(local.set $fp (local.get $f))
		(local.set $f (i64.add))
		(local.set $counter
			(i32.sub
				(local.get $counter)
				(i32.const 1)))
		(br_if $loop
			(local.get $counter)
		)
	)
	(local.get $fp)
	(i64.const -2872092127636481573)
	call $assert_eq)
(start $main))
