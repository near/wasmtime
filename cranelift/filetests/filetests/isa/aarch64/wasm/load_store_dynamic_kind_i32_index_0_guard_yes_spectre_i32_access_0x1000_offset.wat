;;! target = "aarch64"
;;!
;;! settings = ['enable_heap_access_spectre_mitigation=true']
;;!
;;! compile = true
;;!
;;! [globals.vmctx]
;;! type = "i64"
;;! vmctx = true
;;!
;;! [globals.heap_base]
;;! type = "i64"
;;! load = { base = "vmctx", offset = 0, readonly = true }
;;!
;;! [globals.heap_bound]
;;! type = "i64"
;;! load = { base = "vmctx", offset = 8, readonly = true }
;;!
;;! [[heaps]]
;;! base = "heap_base"
;;! min_size = 0x10000
;;! offset_guard_size = 0
;;! index_type = "i32"
;;! style = { kind = "dynamic", bound = "heap_bound" }

;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
;; !!! GENERATED BY 'make-load-store-tests.sh' DO NOT EDIT !!!
;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

(module
  (memory i32 1)

  (func (export "do_store") (param i32 i32)
    local.get 0
    local.get 1
    i32.store offset=0x1000)

  (func (export "do_load") (param i32) (result i32)
    local.get 0
    i32.load offset=0x1000))

;; function u0:0:
;; block0:
;;   mov w12, w0
;;   ldr x13, [x2, #8]
;;   movz x14, #4100
;;   sub x13, x13, x14
;;   ldr x14, [x2]
;;   add x14, x14, x0, UXTW
;;   add x14, x14, #4096
;;   movz x15, #0
;;   subs xzr, x12, x13
;;   csel x13, x15, x14, hi
;;   csdb
;;   str w1, [x13]
;;   b label1
;; block1:
;;   ret
;;
;; function u0:1:
;; block0:
;;   mov w12, w0
;;   ldr x13, [x1, #8]
;;   movz x14, #4100
;;   sub x13, x13, x14
;;   ldr x14, [x1]
;;   add x14, x14, x0, UXTW
;;   add x14, x14, #4096
;;   movz x15, #0
;;   subs xzr, x12, x13
;;   csel x13, x15, x14, hi
;;   csdb
;;   ldr w0, [x13]
;;   b label1
;; block1:
;;   ret
