;; This passes its input to `seal_xcm_query` and returns the return value to its caller.
(module
	(import "seal0" "xcm_query" (func $xcm_query (param i32 i32 i32) (result i32)))
	(import "seal0" "seal_input" (func $seal_input (param i32 i32)))
	(import "seal0" "seal_return" (func $seal_return (param i32 i32 i32)))
	(import "env" "memory" (memory 1 1))

	;; 0x1000 = 4k in little endian
	;; size of input buffer
	(data (i32.const 0) "\00\10")

	(func (export "call")
		;; Receive the encoded call
		(call $seal_input
			(i32.const 4)	;; Pointer to the input buffer
			(i32.const 0)	;; Size of the length buffer
		)
		;; Input data layout.
		;; [0..4) - size of the call
		;; [4..12) - timeout
		;; [12..49) - match_querier
		;; [8..10)  response

		;; Just use the call passed as input and store result to memory
		(i32.store (i32.const 0)
			(call $xcm_query
				(i32.const 4)   ;; Pointer where the timeout is stored
				(i32.const 12)	;; Pointer where the match_querier is stored
				(i32.const 49)	;; ...
			)
		)
		(call $seal_return
			(i32.const 0)	;; flags
			(i32.const 49)	;; returned value
			(i32.const 8)	;; length of returned value
		)
	)

	(func (export "deploy"))
)


