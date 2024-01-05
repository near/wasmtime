;; TODO(akashin): Add Rust code to generate this file to repository.
;; For now see https://github.com/near/wasmtime/issues/90#issuecomment-1823140768.
(module
  (type (;0;) (func (param i64 i64)))
  (type (;1;) (func))
  (import "env" "assert_eq_i64" (func (;0;) (type 0)))
  (func (;1;) (type 1)
    (local i64 i64 i32)
    i64.const 1
    local.set 0
    i64.const 0
    local.set 1
    i32.const 10000
    local.set 2
    loop  ;; label = @1
      local.get 0
      local.get 1
      i64.add
      local.tee 1
      local.get 0
      i64.add
      local.tee 0
      local.get 1
      i64.add
      local.tee 1
      local.get 0
      i64.add
      local.tee 0
      local.get 1
      i64.add
      local.tee 1
      local.get 0
      i64.add
      local.tee 0
      local.get 1
      i64.add
      local.tee 1
      local.get 0
      i64.add
      local.tee 0
      local.get 1
      i64.add
      local.tee 1
      local.get 0
      i64.add
      local.set 0
      local.get 2
      i32.const -10
      i32.add
      local.tee 2
      br_if 0 (;@1;)
    end
    local.get 1
    i64.const -2872092127636481573
    call 0)
	(start 1)
  (memory (;0;) 16)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "main" (func 1))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
