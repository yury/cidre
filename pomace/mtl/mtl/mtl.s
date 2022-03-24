	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0	sdk_version 12, 3
	.globl	_get_rsel_name                  ; -- Begin function get_rsel_name
	.p2align	2
_get_rsel_name:                         ; @get_rsel_name
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_copy_nullable_rsel_name        ; -- Begin function copy_nullable_rsel_name
	.p2align	2
_copy_nullable_rsel_name:               ; @copy_nullable_rsel_name
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_nullable_wsel_setName          ; -- Begin function nullable_wsel_setName
	.p2align	2
_nullable_wsel_setName:                 ; @nullable_wsel_setName
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.2@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.2@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_reset                     ; -- Begin function wsel_reset
	.p2align	2
_wsel_reset:                            ; @wsel_reset
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.4@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.4@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_length                    ; -- Begin function rsel_length
	.p2align	2
_rsel_length:                           ; @rsel_length
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.6@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.6@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_enqueue                   ; -- Begin function wsel_enqueue
	.p2align	2
_wsel_enqueue:                          ; @wsel_enqueue
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.8@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.8@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_commit                    ; -- Begin function wsel_commit
	.p2align	2
_wsel_commit:                           ; @wsel_commit
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.10@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.10@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_waitUntilScheduled        ; -- Begin function wsel_waitUntilScheduled
	.p2align	2
_wsel_waitUntilScheduled:               ; @wsel_waitUntilScheduled
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.12@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.12@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_waitUntilCompleted        ; -- Begin function wsel_waitUntilCompleted
	.p2align	2
_wsel_waitUntilCompleted:               ; @wsel_waitUntilCompleted
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.14@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.14@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_registryID                ; -- Begin function rsel_registryID
	.p2align	2
_rsel_registryID:                       ; @rsel_registryID
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.16@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.16@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_maxThreadsPerThreadgroup  ; -- Begin function rsel_maxThreadsPerThreadgroup
	.p2align	2
_rsel_maxThreadsPerThreadgroup:         ; @rsel_maxThreadsPerThreadgroup
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp]                        ; 8-byte Folded Spill
	stur	x0, [x29, #-8]
	ldur	x8, [x29, #-8]
	str	x8, [sp, #8]                    ; 8-byte Folded Spill
	adrp	x9, _OBJC_SELECTOR_REFERENCES_.18@PAGE
	ldr	x9, [x9, _OBJC_SELECTOR_REFERENCES_.18@PAGEOFF]
	str	x9, [sp, #16]                   ; 8-byte Folded Spill
	cbz	x8, LBB10_2
	b	LBB10_1
LBB10_1:
	ldr	x1, [sp, #16]                   ; 8-byte Folded Reload
	ldr	x0, [sp, #8]                    ; 8-byte Folded Reload
	ldr	x8, [sp]                        ; 8-byte Folded Reload
	bl	_objc_msgSend
	b	LBB10_3
LBB10_2:
	ldr	x8, [sp]                        ; 8-byte Folded Reload
	str	xzr, [x8]
	str	xzr, [x8, #8]
	str	xzr, [x8, #16]
	b	LBB10_3
LBB10_3:
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_hasUnifiedMemory          ; -- Begin function rsel_hasUnifiedMemory
	.p2align	2
_rsel_hasUnifiedMemory:                 ; @rsel_hasUnifiedMemory
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.20@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.20@PAGEOFF]
	bl	_objc_msgSend
	and	w0, w0, #0x1
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_readWriteTextureSupport   ; -- Begin function rsel_readWriteTextureSupport
	.p2align	2
_rsel_readWriteTextureSupport:          ; @rsel_readWriteTextureSupport
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.22@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.22@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_argumentBuffersSupport    ; -- Begin function rsel_argumentBuffersSupport
	.p2align	2
_rsel_argumentBuffersSupport:           ; @rsel_argumentBuffersSupport
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.24@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.24@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newCommandQueue           ; -- Begin function rsel_newCommandQueue
	.p2align	2
_rsel_newCommandQueue:                  ; @rsel_newCommandQueue
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.26@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.26@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newCommandQueueWithMaxCommandBufferCount ; -- Begin function rsel_newCommandQueueWithMaxCommandBufferCount
	.p2align	2
_rsel_newCommandQueueWithMaxCommandBufferCount: ; @rsel_newCommandQueueWithMaxCommandBufferCount
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.28@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.28@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newTextureWithDescriptor  ; -- Begin function rsel_newTextureWithDescriptor
	.p2align	2
_rsel_newTextureWithDescriptor:         ; @rsel_newTextureWithDescriptor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.30@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.30@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newTextureWithDescriptor_iosurface_plane ; -- Begin function rsel_newTextureWithDescriptor_iosurface_plane
	.p2align	2
_rsel_newTextureWithDescriptor_iosurface_plane: ; @rsel_newTextureWithDescriptor_iosurface_plane
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	str	x3, [sp]
	ldur	x0, [x29, #-8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	ldr	x4, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.32@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.32@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newDefaultLibrary         ; -- Begin function rsel_newDefaultLibrary
	.p2align	2
_rsel_newDefaultLibrary:                ; @rsel_newDefaultLibrary
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.34@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.34@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newLibraryWithSource_options_error ; -- Begin function rsel_newLibraryWithSource_options_error
	.p2align	2
_rsel_newLibraryWithSource_options_error: ; @rsel_newLibraryWithSource_options_error
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	str	x3, [sp]
	ldur	x0, [x29, #-8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	ldr	x4, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.36@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.36@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_sel_newLibraryWithSource_options_completionHandler ; -- Begin function sel_newLibraryWithSource_options_completionHandler
	.p2align	2
_sel_newLibraryWithSource_options_completionHandler: ; @sel_newLibraryWithSource_options_completionHandler
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #96
	stp	x29, x30, [sp, #80]             ; 16-byte Folded Spill
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	stur	x1, [x29, #-16]
	stur	x2, [x29, #-24]
	stur	x3, [x29, #-32]
	ldur	x0, [x29, #-8]
	ldur	x2, [x29, #-16]
	ldur	x3, [x29, #-24]
	add	x4, sp, #8
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	str	x8, [sp, #8]
	mov	w8, #-1073741824
	str	w8, [sp, #16]
	str	wzr, [sp, #20]
	adrp	x8, ___sel_newLibraryWithSource_options_completionHandler_block_invoke@PAGE
	add	x8, x8, ___sel_newLibraryWithSource_options_completionHandler_block_invoke@PAGEOFF
	str	x8, [sp, #24]
	adrp	x8, "___block_descriptor_40_e34_v24?0\"<MTLLibrary>\"8\"NSError\"16l"@PAGE
	add	x8, x8, "___block_descriptor_40_e34_v24?0\"<MTLLibrary>\"8\"NSError\"16l"@PAGEOFF
	str	x8, [sp, #32]
	ldur	x8, [x29, #-32]
	str	x8, [sp, #40]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.38@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.38@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #80]             ; 16-byte Folded Reload
	add	sp, sp, #96
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function __sel_newLibraryWithSource_options_completionHandler_block_invoke
___sel_newLibraryWithSource_options_completionHandler_block_invoke: ; @__sel_newLibraryWithSource_options_completionHandler_block_invoke
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #64
	stp	x29, x30, [sp, #48]             ; 16-byte Folded Spill
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	stur	x8, [x29, #-8]
	stur	x1, [x29, #-16]
	str	x2, [sp, #24]
	mov	x8, x0
	str	x8, [sp, #16]
	ldr	x8, [x0, #32]
	ldr	x8, [x8]
	str	x8, [sp, #8]
	ldr	x8, [sp, #8]
	ldr	x0, [x0, #32]
	ldur	x1, [x29, #-16]
	ldr	x2, [sp, #24]
	blr	x8
	ldp	x29, x30, [sp, #48]             ; 16-byte Folded Reload
	add	sp, sp, #64
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newComputePipelineStateWithFunction_error ; -- Begin function rsel_newComputePipelineStateWithFunction_error
	.p2align	2
_rsel_newComputePipelineStateWithFunction_error: ; @rsel_newComputePipelineStateWithFunction_error
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	ldur	x0, [x29, #-8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.40@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.40@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newBufferWithLength_options ; -- Begin function rsel_newBufferWithLength_options
	.p2align	2
_rsel_newBufferWithLength_options:      ; @rsel_newBufferWithLength_options
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	ldur	x0, [x29, #-8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.42@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.42@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newFence                  ; -- Begin function rsel_newFence
	.p2align	2
_rsel_newFence:                         ; @rsel_newFence
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.44@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.44@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newEvent                  ; -- Begin function rsel_newEvent
	.p2align	2
_rsel_newEvent:                         ; @rsel_newEvent
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.46@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.46@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_maxBufferLength           ; -- Begin function rsel_maxBufferLength
	.p2align	2
_rsel_maxBufferLength:                  ; @rsel_maxBufferLength
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.48@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.48@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLCompileOptions_new          ; -- Begin function MTLCompileOptions_new
	.p2align	2
_MTLCompileOptions_new:                 ; @MTLCompileOptions_new
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_@PAGEOFF]
	bl	_objc_opt_new
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_fastMathEnabled           ; -- Begin function rsel_fastMathEnabled
	.p2align	2
_rsel_fastMathEnabled:                  ; @rsel_fastMathEnabled
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.50@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.50@PAGEOFF]
	bl	_objc_msgSend
	and	w0, w0, #0x1
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setFastMathEnabled        ; -- Begin function wsel_setFastMathEnabled
	.p2align	2
_wsel_setFastMathEnabled:               ; @wsel_setFastMathEnabled
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	mov	w8, #1
	and	w8, w1, w8
	strb	w8, [sp, #7]
	ldr	x0, [sp, #8]
	ldrb	w8, [sp, #7]
	adrp	x9, _OBJC_SELECTOR_REFERENCES_.52@PAGE
	ldr	x1, [x9, _OBJC_SELECTOR_REFERENCES_.52@PAGEOFF]
	and	w2, w8, #0x1
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_languageVersion           ; -- Begin function rsel_languageVersion
	.p2align	2
_rsel_languageVersion:                  ; @rsel_languageVersion
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.54@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.54@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setLanguageVersion        ; -- Begin function wsel_setLanguageVersion
	.p2align	2
_wsel_setLanguageVersion:               ; @wsel_setLanguageVersion
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.56@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.56@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_copy_rsel_label                ; -- Begin function copy_rsel_label
	.p2align	2
_copy_rsel_label:                       ; @copy_rsel_label
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.58@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.58@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_get_rsel_label                 ; -- Begin function get_rsel_label
	.p2align	2
_get_rsel_label:                        ; @get_rsel_label
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.58@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.58@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setLabel                  ; -- Begin function wsel_setLabel
	.p2align	2
_wsel_setLabel:                         ; @wsel_setLabel
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.60@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.60@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_device                    ; -- Begin function rsel_device
	.p2align	2
_rsel_device:                           ; @rsel_device
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.62@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.62@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_resourceOptions           ; -- Begin function rsel_resourceOptions
	.p2align	2
_rsel_resourceOptions:                  ; @rsel_resourceOptions
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.64@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.64@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setResourceOptions        ; -- Begin function wsel_setResourceOptions
	.p2align	2
_wsel_setResourceOptions:               ; @wsel_setResourceOptions
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.66@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.66@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_storageMode               ; -- Begin function rsel_storageMode
	.p2align	2
_rsel_storageMode:                      ; @rsel_storageMode
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.68@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.68@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setStorageMode            ; -- Begin function wsel_setStorageMode
	.p2align	2
_wsel_setStorageMode:                   ; @wsel_setStorageMode
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.70@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.70@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_cpuCacheMode              ; -- Begin function rsel_cpuCacheMode
	.p2align	2
_rsel_cpuCacheMode:                     ; @rsel_cpuCacheMode
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.72@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.72@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setCpuCacheMode           ; -- Begin function wsel_setCpuCacheMode
	.p2align	2
_wsel_setCpuCacheMode:                  ; @wsel_setCpuCacheMode
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.74@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.74@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_hazardTrackingMode        ; -- Begin function rsel_hazardTrackingMode
	.p2align	2
_rsel_hazardTrackingMode:               ; @rsel_hazardTrackingMode
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.76@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.76@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setHazardTrackingMode     ; -- Begin function wsel_setHazardTrackingMode
	.p2align	2
_wsel_setHazardTrackingMode:            ; @wsel_setHazardTrackingMode
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.78@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.78@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_textureType               ; -- Begin function rsel_textureType
	.p2align	2
_rsel_textureType:                      ; @rsel_textureType
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.80@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.80@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setTextureType            ; -- Begin function wsel_setTextureType
	.p2align	2
_wsel_setTextureType:                   ; @wsel_setTextureType
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.82@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.82@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_pixelFormat               ; -- Begin function rsel_pixelFormat
	.p2align	2
_rsel_pixelFormat:                      ; @rsel_pixelFormat
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.84@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.84@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setPixelFormat            ; -- Begin function wsel_setPixelFormat
	.p2align	2
_wsel_setPixelFormat:                   ; @wsel_setPixelFormat
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.86@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.86@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_sel_addScheduledHandler        ; -- Begin function sel_addScheduledHandler
	.p2align	2
_sel_addScheduledHandler:               ; @sel_addScheduledHandler
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #80
	stp	x29, x30, [sp, #64]             ; 16-byte Folded Spill
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	stur	x1, [x29, #-16]
	ldur	x0, [x29, #-8]
	add	x2, sp, #8
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	str	x8, [sp, #8]
	mov	w8, #-1073741824
	str	w8, [sp, #16]
	str	wzr, [sp, #20]
	adrp	x8, ___sel_addScheduledHandler_block_invoke@PAGE
	add	x8, x8, ___sel_addScheduledHandler_block_invoke@PAGEOFF
	str	x8, [sp, #24]
	adrp	x8, "___block_descriptor_40_e28_v16?0\"<MTLCommandBuffer>\"8l"@PAGE
	add	x8, x8, "___block_descriptor_40_e28_v16?0\"<MTLCommandBuffer>\"8l"@PAGEOFF
	str	x8, [sp, #32]
	ldur	x8, [x29, #-16]
	str	x8, [sp, #40]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.90@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.90@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #64]             ; 16-byte Folded Reload
	add	sp, sp, #80
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function __sel_addScheduledHandler_block_invoke
___sel_addScheduledHandler_block_invoke: ; @__sel_addScheduledHandler_block_invoke
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	stur	x8, [x29, #-8]
	str	x1, [sp, #16]
	mov	x8, x0
	str	x8, [sp, #8]
	ldr	x8, [x0, #32]
	ldr	x8, [x8]
	str	x8, [sp]
	ldr	x8, [sp]
	ldr	x0, [x0, #32]
	ldr	x1, [sp, #16]
	blr	x8
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_sel_addCompletedHandler        ; -- Begin function sel_addCompletedHandler
	.p2align	2
_sel_addCompletedHandler:               ; @sel_addCompletedHandler
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #80
	stp	x29, x30, [sp, #64]             ; 16-byte Folded Spill
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	stur	x1, [x29, #-16]
	ldur	x0, [x29, #-8]
	add	x2, sp, #8
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	str	x8, [sp, #8]
	mov	w8, #-1073741824
	str	w8, [sp, #16]
	str	wzr, [sp, #20]
	adrp	x8, ___sel_addCompletedHandler_block_invoke@PAGE
	add	x8, x8, ___sel_addCompletedHandler_block_invoke@PAGEOFF
	str	x8, [sp, #24]
	adrp	x8, "___block_descriptor_40_e28_v16?0\"<MTLCommandBuffer>\"8l"@PAGE
	add	x8, x8, "___block_descriptor_40_e28_v16?0\"<MTLCommandBuffer>\"8l"@PAGEOFF
	str	x8, [sp, #32]
	ldur	x8, [x29, #-16]
	str	x8, [sp, #40]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.92@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.92@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #64]             ; 16-byte Folded Reload
	add	sp, sp, #80
	ret
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function __sel_addCompletedHandler_block_invoke
___sel_addCompletedHandler_block_invoke: ; @__sel_addCompletedHandler_block_invoke
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	stur	x8, [x29, #-8]
	str	x1, [sp, #16]
	mov	x8, x0
	str	x8, [sp, #8]
	ldr	x8, [x0, #32]
	ldr	x8, [x8]
	str	x8, [sp]
	ldr	x8, [sp]
	ldr	x0, [x0, #32]
	ldr	x1, [sp, #16]
	blr	x8
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLTextureDescriptor_texture2DDescriptorWithPixelFormat_width_height_mipmapped ; -- Begin function MTLTextureDescriptor_texture2DDescriptorWithPixelFormat_width_height_mipmapped
	.p2align	2
_MTLTextureDescriptor_texture2DDescriptorWithPixelFormat_width_height_mipmapped: ; @MTLTextureDescriptor_texture2DDescriptorWithPixelFormat_width_height_mipmapped
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	mov	w8, #1
	and	w8, w3, w8
	strb	w8, [sp, #7]
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.93@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.93@PAGEOFF]
	ldur	x2, [x29, #-8]
	ldr	x3, [sp, #16]
	ldr	x4, [sp, #8]
	ldrb	w8, [sp, #7]
	adrp	x9, _OBJC_SELECTOR_REFERENCES_.95@PAGE
	ldr	x1, [x9, _OBJC_SELECTOR_REFERENCES_.95@PAGEOFF]
	and	w5, w8, #0x1
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLTextureDescriptor_textureCubeDescriptorWithPixelFormat_size_mipmapped ; -- Begin function MTLTextureDescriptor_textureCubeDescriptorWithPixelFormat_size_mipmapped
	.p2align	2
_MTLTextureDescriptor_textureCubeDescriptorWithPixelFormat_size_mipmapped: ; @MTLTextureDescriptor_textureCubeDescriptorWithPixelFormat_size_mipmapped
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	mov	w8, #1
	and	w8, w2, w8
	strb	w8, [sp, #15]
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.93@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.93@PAGEOFF]
	ldur	x2, [x29, #-8]
	ldr	x3, [sp, #16]
	ldrb	w8, [sp, #15]
	adrp	x9, _OBJC_SELECTOR_REFERENCES_.97@PAGE
	ldr	x1, [x9, _OBJC_SELECTOR_REFERENCES_.97@PAGEOFF]
	and	w4, w8, #0x1
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLTextureDescriptor_textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage ; -- Begin function MTLTextureDescriptor_textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage
	.p2align	2
_MTLTextureDescriptor_textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage: ; @MTLTextureDescriptor_textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	str	x3, [sp]
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.93@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.93@PAGEOFF]
	ldur	x2, [x29, #-8]
	ldr	x3, [sp, #16]
	ldr	x4, [sp, #8]
	ldr	x5, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.99@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.99@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_width                     ; -- Begin function rsel_width
	.p2align	2
_rsel_width:                            ; @rsel_width
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.101@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.101@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setWidth                  ; -- Begin function wsel_setWidth
	.p2align	2
_wsel_setWidth:                         ; @wsel_setWidth
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.103@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.103@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_height                    ; -- Begin function rsel_height
	.p2align	2
_rsel_height:                           ; @rsel_height
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.105@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.105@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setHeight                 ; -- Begin function wsel_setHeight
	.p2align	2
_wsel_setHeight:                        ; @wsel_setHeight
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.107@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.107@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_depth                     ; -- Begin function rsel_depth
	.p2align	2
_rsel_depth:                            ; @rsel_depth
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.109@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.109@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setDepth                  ; -- Begin function wsel_setDepth
	.p2align	2
_wsel_setDepth:                         ; @wsel_setDepth
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.111@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.111@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_mipmapLevelCount          ; -- Begin function rsel_mipmapLevelCount
	.p2align	2
_rsel_mipmapLevelCount:                 ; @rsel_mipmapLevelCount
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.113@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.113@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setMipmapLevelCount       ; -- Begin function wsel_setMipmapLevelCount
	.p2align	2
_wsel_setMipmapLevelCount:              ; @wsel_setMipmapLevelCount
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.115@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.115@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLTextureDescriptor_rsel_sampleCount ; -- Begin function MTLTextureDescriptor_rsel_sampleCount
	.p2align	2
_MTLTextureDescriptor_rsel_sampleCount: ; @MTLTextureDescriptor_rsel_sampleCount
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.117@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.117@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLTextureDescriptor_wsel_setSampleCount ; -- Begin function MTLTextureDescriptor_wsel_setSampleCount
	.p2align	2
_MTLTextureDescriptor_wsel_setSampleCount: ; @MTLTextureDescriptor_wsel_setSampleCount
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.119@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.119@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_arrayLength               ; -- Begin function rsel_arrayLength
	.p2align	2
_rsel_arrayLength:                      ; @rsel_arrayLength
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.121@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.121@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setArrayLength            ; -- Begin function wsel_setArrayLength
	.p2align	2
_wsel_setArrayLength:                   ; @wsel_setArrayLength
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.123@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.123@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_usage                     ; -- Begin function rsel_usage
	.p2align	2
_rsel_usage:                            ; @rsel_usage
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.125@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.125@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setUsage                  ; -- Begin function wsel_setUsage
	.p2align	2
_wsel_setUsage:                         ; @wsel_setUsage
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.127@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.127@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_allowGPUOptimizedContents ; -- Begin function rsel_allowGPUOptimizedContents
	.p2align	2
_rsel_allowGPUOptimizedContents:        ; @rsel_allowGPUOptimizedContents
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.129@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.129@PAGEOFF]
	bl	_objc_msgSend
	and	w0, w0, #0x1
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setAllowGPUOptimizedContents ; -- Begin function wsel_setAllowGPUOptimizedContents
	.p2align	2
_wsel_setAllowGPUOptimizedContents:     ; @wsel_setAllowGPUOptimizedContents
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	mov	w8, #1
	and	w8, w1, w8
	strb	w8, [sp, #7]
	ldr	x0, [sp, #8]
	ldrb	w8, [sp, #7]
	adrp	x9, _OBJC_SELECTOR_REFERENCES_.131@PAGE
	ldr	x1, [x9, _OBJC_SELECTOR_REFERENCES_.131@PAGEOFF]
	and	w2, w8, #0x1
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_swizzle                   ; -- Begin function rsel_swizzle
	.p2align	2
_rsel_swizzle:                          ; @rsel_swizzle
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	ldr	x0, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.133@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.133@PAGEOFF]
	bl	_objc_msgSend
	stur	w0, [x29, #-4]
	ldur	w0, [x29, #-4]
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setSwizzle                ; -- Begin function wsel_setSwizzle
	.p2align	2
_wsel_setSwizzle:                       ; @wsel_setSwizzle
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp, #16]
	ldr	w8, [sp, #16]
	stur	w8, [x29, #-4]
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.135@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.135@PAGEOFF]
	ldur	w8, [x29, #-4]
	str	w8, [sp]
	ldr	x2, [sp]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_parentTexture             ; -- Begin function rsel_parentTexture
	.p2align	2
_rsel_parentTexture:                    ; @rsel_parentTexture
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.137@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.137@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newTextureViewWithPixelFormat ; -- Begin function rsel_newTextureViewWithPixelFormat
	.p2align	2
_rsel_newTextureViewWithPixelFormat:    ; @rsel_newTextureViewWithPixelFormat
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.139@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.139@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_iosurface                 ; -- Begin function rsel_iosurface
	.p2align	2
_rsel_iosurface:                        ; @rsel_iosurface
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.141@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.141@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_iosurfacePlane            ; -- Begin function rsel_iosurfacePlane
	.p2align	2
_rsel_iosurfacePlane:                   ; @rsel_iosurfacePlane
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.143@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.143@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLFunctionConstantValues_new  ; -- Begin function MTLFunctionConstantValues_new
	.p2align	2
_MTLFunctionConstantValues_new:         ; @MTLFunctionConstantValues_new
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.144@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.144@PAGEOFF]
	bl	_objc_opt_new
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setConstantValue_type_atIndex ; -- Begin function wsel_setConstantValue_type_atIndex
	.p2align	2
_wsel_setConstantValue_type_atIndex:    ; @wsel_setConstantValue_type_atIndex
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	str	x3, [sp]
	ldur	x0, [x29, #-8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	ldr	x4, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.146@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.146@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setConstantValues_type_withRange ; -- Begin function wsel_setConstantValues_type_withRange
	.p2align	2
_wsel_setConstantValues_type_withRange: ; @wsel_setConstantValues_type_withRange
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #64
	stp	x29, x30, [sp, #48]             ; 16-byte Folded Spill
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x3, [x29, #-16]
	stur	x4, [x29, #-8]
	str	x0, [sp, #24]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	ldr	x0, [sp, #24]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.148@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.148@PAGEOFF]
	ldur	x4, [x29, #-16]
	ldur	x5, [x29, #-8]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #48]             ; 16-byte Folded Reload
	add	sp, sp, #64
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setConstantValue_type_withName ; -- Begin function wsel_setConstantValue_type_withName
	.p2align	2
_wsel_setConstantValue_type_withName:   ; @wsel_setConstantValue_type_withName
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	str	x3, [sp]
	ldur	x0, [x29, #-8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	ldr	x4, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.150@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.150@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLHeapDescriptor_new          ; -- Begin function MTLHeapDescriptor_new
	.p2align	2
_MTLHeapDescriptor_new:                 ; @MTLHeapDescriptor_new
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.151@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.151@PAGEOFF]
	bl	_objc_opt_new
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_size                      ; -- Begin function rsel_size
	.p2align	2
_rsel_size:                             ; @rsel_size
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.153@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.153@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setSize                   ; -- Begin function wsel_setSize
	.p2align	2
_wsel_setSize:                          ; @wsel_setSize
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.155@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.155@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLHeapType_MTLHeapDescriptor_rsel_type ; -- Begin function MTLHeapType_MTLHeapDescriptor_rsel_type
	.p2align	2
_MTLHeapType_MTLHeapDescriptor_rsel_type: ; @MTLHeapType_MTLHeapDescriptor_rsel_type
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.157@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.157@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLHeapType_MTLHeapDescriptor_wsel_setType ; -- Begin function MTLHeapType_MTLHeapDescriptor_wsel_setType
	.p2align	2
_MTLHeapType_MTLHeapDescriptor_wsel_setType: ; @MTLHeapType_MTLHeapDescriptor_wsel_setType
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.159@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.159@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLHeapType_MTLHeap_rsel_type  ; -- Begin function MTLHeapType_MTLHeap_rsel_type
	.p2align	2
_MTLHeapType_MTLHeap_rsel_type:         ; @MTLHeapType_MTLHeap_rsel_type
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.157@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.157@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLHeapType_MTLHeap_rsel_currentAllocatedSize ; -- Begin function MTLHeapType_MTLHeap_rsel_currentAllocatedSize
	.p2align	2
_MTLHeapType_MTLHeap_rsel_currentAllocatedSize: ; @MTLHeapType_MTLHeap_rsel_currentAllocatedSize
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.161@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.161@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_maxAvailableSizeWithAlignment ; -- Begin function rsel_maxAvailableSizeWithAlignment
	.p2align	2
_rsel_maxAvailableSizeWithAlignment:    ; @rsel_maxAvailableSizeWithAlignment
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.163@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.163@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newBufferWithBytes_length_options ; -- Begin function rsel_newBufferWithBytes_length_options
	.p2align	2
_rsel_newBufferWithBytes_length_options: ; @rsel_newBufferWithBytes_length_options
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	str	x3, [sp]
	ldur	x0, [x29, #-8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	ldr	x4, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.165@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.165@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newDepthStencilStateWithDescriptor ; -- Begin function rsel_newDepthStencilStateWithDescriptor
	.p2align	2
_rsel_newDepthStencilStateWithDescriptor: ; @rsel_newDepthStencilStateWithDescriptor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.167@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.167@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newFunctionWithName       ; -- Begin function rsel_newFunctionWithName
	.p2align	2
_rsel_newFunctionWithName:              ; @rsel_newFunctionWithName
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.169@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.169@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newFunctionWithName_constantValues_error ; -- Begin function rsel_newFunctionWithName_constantValues_error
	.p2align	2
_rsel_newFunctionWithName_constantValues_error: ; @rsel_newFunctionWithName_constantValues_error
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	str	x1, [sp, #16]
	str	x2, [sp, #8]
	str	x3, [sp]
	ldur	x0, [x29, #-8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #8]
	ldr	x4, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.171@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.171@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_functionNames             ; -- Begin function rsel_functionNames
	.p2align	2
_rsel_functionNames:                    ; @rsel_functionNames
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.173@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.173@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_newArgumentEncoderWithBufferIndex ; -- Begin function rsel_newArgumentEncoderWithBufferIndex
	.p2align	2
_rsel_newArgumentEncoderWithBufferIndex: ; @rsel_newArgumentEncoderWithBufferIndex
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.175@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.175@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_encodedLength             ; -- Begin function rsel_encodedLength
	.p2align	2
_rsel_encodedLength:                    ; @rsel_encodedLength
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.177@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.177@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_alignment                 ; -- Begin function rsel_alignment
	.p2align	2
_rsel_alignment:                        ; @rsel_alignment
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.179@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.179@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_options                   ; -- Begin function rsel_options
	.p2align	2
_rsel_options:                          ; @rsel_options
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.181@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.181@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLFunctionDescriptor_functionDescriptor ; -- Begin function MTLFunctionDescriptor_functionDescriptor
	.p2align	2
_MTLFunctionDescriptor_functionDescriptor: ; @MTLFunctionDescriptor_functionDescriptor
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.182@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.182@PAGEOFF]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.184@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.184@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_isBlendingEnabled         ; -- Begin function rsel_isBlendingEnabled
	.p2align	2
_rsel_isBlendingEnabled:                ; @rsel_isBlendingEnabled
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.186@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.186@PAGEOFF]
	bl	_objc_msgSend
	and	w0, w0, #0x1
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setBlendingEnabled        ; -- Begin function wsel_setBlendingEnabled
	.p2align	2
_wsel_setBlendingEnabled:               ; @wsel_setBlendingEnabled
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	mov	w8, #1
	and	w8, w1, w8
	strb	w8, [sp, #7]
	ldr	x0, [sp, #8]
	ldrb	w8, [sp, #7]
	adrp	x9, _OBJC_SELECTOR_REFERENCES_.188@PAGE
	ldr	x1, [x9, _OBJC_SELECTOR_REFERENCES_.188@PAGEOFF]
	and	w2, w8, #0x1
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_sourceRGBBlendFactor      ; -- Begin function rsel_sourceRGBBlendFactor
	.p2align	2
_rsel_sourceRGBBlendFactor:             ; @rsel_sourceRGBBlendFactor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.190@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.190@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setSourceRGBBlendFactor   ; -- Begin function wsel_setSourceRGBBlendFactor
	.p2align	2
_wsel_setSourceRGBBlendFactor:          ; @wsel_setSourceRGBBlendFactor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.192@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.192@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_destinationRGBBlendFactor ; -- Begin function rsel_destinationRGBBlendFactor
	.p2align	2
_rsel_destinationRGBBlendFactor:        ; @rsel_destinationRGBBlendFactor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.194@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.194@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setDestinationRGBBlendFactor ; -- Begin function wsel_setDestinationRGBBlendFactor
	.p2align	2
_wsel_setDestinationRGBBlendFactor:     ; @wsel_setDestinationRGBBlendFactor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.196@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.196@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_rgbBlendOperation         ; -- Begin function rsel_rgbBlendOperation
	.p2align	2
_rsel_rgbBlendOperation:                ; @rsel_rgbBlendOperation
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.198@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.198@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setRgbBlendOperation      ; -- Begin function wsel_setRgbBlendOperation
	.p2align	2
_wsel_setRgbBlendOperation:             ; @wsel_setRgbBlendOperation
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.200@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.200@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_sourceAlphaBlendFactor    ; -- Begin function rsel_sourceAlphaBlendFactor
	.p2align	2
_rsel_sourceAlphaBlendFactor:           ; @rsel_sourceAlphaBlendFactor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.202@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.202@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setSourceAlphaBlendFactor ; -- Begin function wsel_setSourceAlphaBlendFactor
	.p2align	2
_wsel_setSourceAlphaBlendFactor:        ; @wsel_setSourceAlphaBlendFactor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.204@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.204@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_destinationAlphaBlendFactor ; -- Begin function rsel_destinationAlphaBlendFactor
	.p2align	2
_rsel_destinationAlphaBlendFactor:      ; @rsel_destinationAlphaBlendFactor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.206@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.206@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setDestinationAlphaBlendFactor ; -- Begin function wsel_setDestinationAlphaBlendFactor
	.p2align	2
_wsel_setDestinationAlphaBlendFactor:   ; @wsel_setDestinationAlphaBlendFactor
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.208@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.208@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_alphaBlendOperation       ; -- Begin function rsel_alphaBlendOperation
	.p2align	2
_rsel_alphaBlendOperation:              ; @rsel_alphaBlendOperation
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.210@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.210@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setAlphaBlendOperation    ; -- Begin function wsel_setAlphaBlendOperation
	.p2align	2
_wsel_setAlphaBlendOperation:           ; @wsel_setAlphaBlendOperation
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.212@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.212@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_render_pipeline_rsel_writeMask ; -- Begin function render_pipeline_rsel_writeMask
	.p2align	2
_render_pipeline_rsel_writeMask:        ; @render_pipeline_rsel_writeMask
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.214@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.214@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_render_pipeline_wsel_setWriteMask ; -- Begin function render_pipeline_wsel_setWriteMask
	.p2align	2
_render_pipeline_wsel_setWriteMask:     ; @render_pipeline_wsel_setWriteMask
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.216@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.216@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_vertexArguments           ; -- Begin function rsel_vertexArguments
	.p2align	2
_rsel_vertexArguments:                  ; @rsel_vertexArguments
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.218@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.218@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_fragmentArguments         ; -- Begin function rsel_fragmentArguments
	.p2align	2
_rsel_fragmentArguments:                ; @rsel_fragmentArguments
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.220@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.220@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_tileArguments             ; -- Begin function rsel_tileArguments
	.p2align	2
_rsel_tileArguments:                    ; @rsel_tileArguments
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.222@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.222@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLRenderPipelineDescriptor_new ; -- Begin function MTLRenderPipelineDescriptor_new
	.p2align	2
_MTLRenderPipelineDescriptor_new:       ; @MTLRenderPipelineDescriptor_new
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.223@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.223@PAGEOFF]
	bl	_objc_opt_new
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_vertexFunction            ; -- Begin function rsel_vertexFunction
	.p2align	2
_rsel_vertexFunction:                   ; @rsel_vertexFunction
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.225@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.225@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setVertexFunction         ; -- Begin function wsel_setVertexFunction
	.p2align	2
_wsel_setVertexFunction:                ; @wsel_setVertexFunction
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.227@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.227@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_fragmentFunction          ; -- Begin function rsel_fragmentFunction
	.p2align	2
_rsel_fragmentFunction:                 ; @rsel_fragmentFunction
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.229@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.229@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setFragmentFunction       ; -- Begin function wsel_setFragmentFunction
	.p2align	2
_wsel_setFragmentFunction:              ; @wsel_setFragmentFunction
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.231@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.231@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_rasterSampleCount         ; -- Begin function rsel_rasterSampleCount
	.p2align	2
_rsel_rasterSampleCount:                ; @rsel_rasterSampleCount
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.233@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.233@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setRasterSampleCount      ; -- Begin function wsel_setRasterSampleCount
	.p2align	2
_wsel_setRasterSampleCount:             ; @wsel_setRasterSampleCount
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.235@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.235@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_maxTotalThreadsPerThreadgroup ; -- Begin function rsel_maxTotalThreadsPerThreadgroup
	.p2align	2
_rsel_maxTotalThreadsPerThreadgroup:    ; @rsel_maxTotalThreadsPerThreadgroup
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.237@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.237@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_threadgroupSizeMatchesTileSize ; -- Begin function rsel_threadgroupSizeMatchesTileSize
	.p2align	2
_rsel_threadgroupSizeMatchesTileSize:   ; @rsel_threadgroupSizeMatchesTileSize
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.239@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.239@PAGEOFF]
	bl	_objc_msgSend
	and	w0, w0, #0x1
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_imageblockSampleLength    ; -- Begin function rsel_imageblockSampleLength
	.p2align	2
_rsel_imageblockSampleLength:           ; @rsel_imageblockSampleLength
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.241@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.241@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_MTLComputePipelineDescriptor_new ; -- Begin function MTLComputePipelineDescriptor_new
	.p2align	2
_MTLComputePipelineDescriptor_new:      ; @MTLComputePipelineDescriptor_new
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.242@PAGE
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.242@PAGEOFF]
	bl	_objc_opt_new
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_threadExecutionWidth      ; -- Begin function rsel_threadExecutionWidth
	.p2align	2
_rsel_threadExecutionWidth:             ; @rsel_threadExecutionWidth
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.244@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.244@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_staticThreadgroupMemoryLength ; -- Begin function rsel_staticThreadgroupMemoryLength
	.p2align	2
_rsel_staticThreadgroupMemoryLength:    ; @rsel_staticThreadgroupMemoryLength
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.246@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.246@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_endEncoding               ; -- Begin function wsel_endEncoding
	.p2align	2
_wsel_endEncoding:                      ; @wsel_endEncoding
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.248@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.248@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_insertDebugSignpost       ; -- Begin function wsel_insertDebugSignpost
	.p2align	2
_wsel_insertDebugSignpost:              ; @wsel_insertDebugSignpost
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.250@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.250@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_pushDebugGroup            ; -- Begin function wsel_pushDebugGroup
	.p2align	2
_wsel_pushDebugGroup:                   ; @wsel_pushDebugGroup
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	ldr	x0, [sp, #8]
	ldr	x2, [sp]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.252@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.252@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_popDebugGroup             ; -- Begin function wsel_popDebugGroup
	.p2align	2
_wsel_popDebugGroup:                    ; @wsel_popDebugGroup
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.254@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.254@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_renderCommandEncoder      ; -- Begin function rsel_renderCommandEncoder
	.p2align	2
_rsel_renderCommandEncoder:             ; @rsel_renderCommandEncoder
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.256@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.256@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_:                  ; @OBJC_METH_VAR_NAME_
	.asciz	"name"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_
_OBJC_SELECTOR_REFERENCES_:
	.quad	l_OBJC_METH_VAR_NAME_

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.1:                ; @OBJC_METH_VAR_NAME_.1
	.asciz	"setName:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.2
_OBJC_SELECTOR_REFERENCES_.2:
	.quad	l_OBJC_METH_VAR_NAME_.1

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.3:                ; @OBJC_METH_VAR_NAME_.3
	.asciz	"reset"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.4
_OBJC_SELECTOR_REFERENCES_.4:
	.quad	l_OBJC_METH_VAR_NAME_.3

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.5:                ; @OBJC_METH_VAR_NAME_.5
	.asciz	"length"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.6
_OBJC_SELECTOR_REFERENCES_.6:
	.quad	l_OBJC_METH_VAR_NAME_.5

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.7:                ; @OBJC_METH_VAR_NAME_.7
	.asciz	"enqueue"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.8
_OBJC_SELECTOR_REFERENCES_.8:
	.quad	l_OBJC_METH_VAR_NAME_.7

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.9:                ; @OBJC_METH_VAR_NAME_.9
	.asciz	"commit"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.10
_OBJC_SELECTOR_REFERENCES_.10:
	.quad	l_OBJC_METH_VAR_NAME_.9

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.11:               ; @OBJC_METH_VAR_NAME_.11
	.asciz	"waitUntilScheduled"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.12
_OBJC_SELECTOR_REFERENCES_.12:
	.quad	l_OBJC_METH_VAR_NAME_.11

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.13:               ; @OBJC_METH_VAR_NAME_.13
	.asciz	"waitUntilCompleted"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.14
_OBJC_SELECTOR_REFERENCES_.14:
	.quad	l_OBJC_METH_VAR_NAME_.13

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.15:               ; @OBJC_METH_VAR_NAME_.15
	.asciz	"registryID"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.16
_OBJC_SELECTOR_REFERENCES_.16:
	.quad	l_OBJC_METH_VAR_NAME_.15

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.17:               ; @OBJC_METH_VAR_NAME_.17
	.asciz	"maxThreadsPerThreadgroup"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.18
_OBJC_SELECTOR_REFERENCES_.18:
	.quad	l_OBJC_METH_VAR_NAME_.17

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.19:               ; @OBJC_METH_VAR_NAME_.19
	.asciz	"hasUnifiedMemory"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.20
_OBJC_SELECTOR_REFERENCES_.20:
	.quad	l_OBJC_METH_VAR_NAME_.19

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.21:               ; @OBJC_METH_VAR_NAME_.21
	.asciz	"readWriteTextureSupport"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.22
_OBJC_SELECTOR_REFERENCES_.22:
	.quad	l_OBJC_METH_VAR_NAME_.21

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.23:               ; @OBJC_METH_VAR_NAME_.23
	.asciz	"argumentBuffersSupport"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.24
_OBJC_SELECTOR_REFERENCES_.24:
	.quad	l_OBJC_METH_VAR_NAME_.23

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.25:               ; @OBJC_METH_VAR_NAME_.25
	.asciz	"newCommandQueue"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.26
_OBJC_SELECTOR_REFERENCES_.26:
	.quad	l_OBJC_METH_VAR_NAME_.25

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.27:               ; @OBJC_METH_VAR_NAME_.27
	.asciz	"newCommandQueueWithMaxCommandBufferCount:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.28
_OBJC_SELECTOR_REFERENCES_.28:
	.quad	l_OBJC_METH_VAR_NAME_.27

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.29:               ; @OBJC_METH_VAR_NAME_.29
	.asciz	"newTextureWithDescriptor:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.30
_OBJC_SELECTOR_REFERENCES_.30:
	.quad	l_OBJC_METH_VAR_NAME_.29

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.31:               ; @OBJC_METH_VAR_NAME_.31
	.asciz	"newTextureWithDescriptor:iosurface:plane:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.32
_OBJC_SELECTOR_REFERENCES_.32:
	.quad	l_OBJC_METH_VAR_NAME_.31

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.33:               ; @OBJC_METH_VAR_NAME_.33
	.asciz	"newDefaultLibrary"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.34
_OBJC_SELECTOR_REFERENCES_.34:
	.quad	l_OBJC_METH_VAR_NAME_.33

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.35:               ; @OBJC_METH_VAR_NAME_.35
	.asciz	"newLibraryWithSource:options:error:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.36
_OBJC_SELECTOR_REFERENCES_.36:
	.quad	l_OBJC_METH_VAR_NAME_.35

	.section	__TEXT,__cstring,cstring_literals
l_.str:                                 ; @.str
	.asciz	"v24@?0@\"<MTLLibrary>\"8@\"NSError\"16"

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_:                     ; @OBJC_CLASS_NAME_
	.space	1

	.private_extern	"___block_descriptor_40_e34_v24?0\"<MTLLibrary>\"8\"NSError\"16l" ; @"__block_descriptor_40_e34_v24\01?0\01\22<MTLLibrary>\228\01\22NSError\2216l"
	.section	__DATA,__const
	.globl	"___block_descriptor_40_e34_v24?0\"<MTLLibrary>\"8\"NSError\"16l"
	.weak_def_can_be_hidden	"___block_descriptor_40_e34_v24?0\"<MTLLibrary>\"8\"NSError\"16l"
	.p2align	3
"___block_descriptor_40_e34_v24?0\"<MTLLibrary>\"8\"NSError\"16l":
	.quad	0                               ; 0x0
	.quad	40                              ; 0x28
	.quad	l_.str
	.quad	l_OBJC_CLASS_NAME_

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.37:               ; @OBJC_METH_VAR_NAME_.37
	.asciz	"newLibraryWithSource:options:completionHandler:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.38
_OBJC_SELECTOR_REFERENCES_.38:
	.quad	l_OBJC_METH_VAR_NAME_.37

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.39:               ; @OBJC_METH_VAR_NAME_.39
	.asciz	"newComputePipelineStateWithFunction:error:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.40
_OBJC_SELECTOR_REFERENCES_.40:
	.quad	l_OBJC_METH_VAR_NAME_.39

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.41:               ; @OBJC_METH_VAR_NAME_.41
	.asciz	"newBufferWithLength:options:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.42
_OBJC_SELECTOR_REFERENCES_.42:
	.quad	l_OBJC_METH_VAR_NAME_.41

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.43:               ; @OBJC_METH_VAR_NAME_.43
	.asciz	"newFence"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.44
_OBJC_SELECTOR_REFERENCES_.44:
	.quad	l_OBJC_METH_VAR_NAME_.43

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.45:               ; @OBJC_METH_VAR_NAME_.45
	.asciz	"newEvent"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.46
_OBJC_SELECTOR_REFERENCES_.46:
	.quad	l_OBJC_METH_VAR_NAME_.45

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.47:               ; @OBJC_METH_VAR_NAME_.47
	.asciz	"maxBufferLength"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.48
_OBJC_SELECTOR_REFERENCES_.48:
	.quad	l_OBJC_METH_VAR_NAME_.47

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_"
_OBJC_CLASSLIST_REFERENCES_$_:
	.quad	_OBJC_CLASS_$_MTLCompileOptions

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.49:               ; @OBJC_METH_VAR_NAME_.49
	.asciz	"fastMathEnabled"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.50
_OBJC_SELECTOR_REFERENCES_.50:
	.quad	l_OBJC_METH_VAR_NAME_.49

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.51:               ; @OBJC_METH_VAR_NAME_.51
	.asciz	"setFastMathEnabled:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.52
_OBJC_SELECTOR_REFERENCES_.52:
	.quad	l_OBJC_METH_VAR_NAME_.51

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.53:               ; @OBJC_METH_VAR_NAME_.53
	.asciz	"languageVersion"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.54
_OBJC_SELECTOR_REFERENCES_.54:
	.quad	l_OBJC_METH_VAR_NAME_.53

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.55:               ; @OBJC_METH_VAR_NAME_.55
	.asciz	"setLanguageVersion:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.56
_OBJC_SELECTOR_REFERENCES_.56:
	.quad	l_OBJC_METH_VAR_NAME_.55

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.57:               ; @OBJC_METH_VAR_NAME_.57
	.asciz	"label"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.58
_OBJC_SELECTOR_REFERENCES_.58:
	.quad	l_OBJC_METH_VAR_NAME_.57

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.59:               ; @OBJC_METH_VAR_NAME_.59
	.asciz	"setLabel:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.60
_OBJC_SELECTOR_REFERENCES_.60:
	.quad	l_OBJC_METH_VAR_NAME_.59

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.61:               ; @OBJC_METH_VAR_NAME_.61
	.asciz	"device"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.62
_OBJC_SELECTOR_REFERENCES_.62:
	.quad	l_OBJC_METH_VAR_NAME_.61

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.63:               ; @OBJC_METH_VAR_NAME_.63
	.asciz	"resourceOptions"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.64
_OBJC_SELECTOR_REFERENCES_.64:
	.quad	l_OBJC_METH_VAR_NAME_.63

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.65:               ; @OBJC_METH_VAR_NAME_.65
	.asciz	"setResourceOptions:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.66
_OBJC_SELECTOR_REFERENCES_.66:
	.quad	l_OBJC_METH_VAR_NAME_.65

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.67:               ; @OBJC_METH_VAR_NAME_.67
	.asciz	"storageMode"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.68
_OBJC_SELECTOR_REFERENCES_.68:
	.quad	l_OBJC_METH_VAR_NAME_.67

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.69:               ; @OBJC_METH_VAR_NAME_.69
	.asciz	"setStorageMode:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.70
_OBJC_SELECTOR_REFERENCES_.70:
	.quad	l_OBJC_METH_VAR_NAME_.69

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.71:               ; @OBJC_METH_VAR_NAME_.71
	.asciz	"cpuCacheMode"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.72
_OBJC_SELECTOR_REFERENCES_.72:
	.quad	l_OBJC_METH_VAR_NAME_.71

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.73:               ; @OBJC_METH_VAR_NAME_.73
	.asciz	"setCpuCacheMode:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.74
_OBJC_SELECTOR_REFERENCES_.74:
	.quad	l_OBJC_METH_VAR_NAME_.73

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.75:               ; @OBJC_METH_VAR_NAME_.75
	.asciz	"hazardTrackingMode"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.76
_OBJC_SELECTOR_REFERENCES_.76:
	.quad	l_OBJC_METH_VAR_NAME_.75

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.77:               ; @OBJC_METH_VAR_NAME_.77
	.asciz	"setHazardTrackingMode:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.78
_OBJC_SELECTOR_REFERENCES_.78:
	.quad	l_OBJC_METH_VAR_NAME_.77

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.79:               ; @OBJC_METH_VAR_NAME_.79
	.asciz	"textureType"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.80
_OBJC_SELECTOR_REFERENCES_.80:
	.quad	l_OBJC_METH_VAR_NAME_.79

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.81:               ; @OBJC_METH_VAR_NAME_.81
	.asciz	"setTextureType:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.82
_OBJC_SELECTOR_REFERENCES_.82:
	.quad	l_OBJC_METH_VAR_NAME_.81

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.83:               ; @OBJC_METH_VAR_NAME_.83
	.asciz	"pixelFormat"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.84
_OBJC_SELECTOR_REFERENCES_.84:
	.quad	l_OBJC_METH_VAR_NAME_.83

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.85:               ; @OBJC_METH_VAR_NAME_.85
	.asciz	"setPixelFormat:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.86
_OBJC_SELECTOR_REFERENCES_.86:
	.quad	l_OBJC_METH_VAR_NAME_.85

	.section	__TEXT,__cstring,cstring_literals
l_.str.87:                              ; @.str.87
	.asciz	"v16@?0@\"<MTLCommandBuffer>\"8"

	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_.88:                  ; @OBJC_CLASS_NAME_.88
	.space	1

	.private_extern	"___block_descriptor_40_e28_v16?0\"<MTLCommandBuffer>\"8l" ; @"__block_descriptor_40_e28_v16\01?0\01\22<MTLCommandBuffer>\228l"
	.section	__DATA,__const
	.globl	"___block_descriptor_40_e28_v16?0\"<MTLCommandBuffer>\"8l"
	.weak_def_can_be_hidden	"___block_descriptor_40_e28_v16?0\"<MTLCommandBuffer>\"8l"
	.p2align	3
"___block_descriptor_40_e28_v16?0\"<MTLCommandBuffer>\"8l":
	.quad	0                               ; 0x0
	.quad	40                              ; 0x28
	.quad	l_.str.87
	.quad	l_OBJC_CLASS_NAME_.88

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.89:               ; @OBJC_METH_VAR_NAME_.89
	.asciz	"addScheduledHandler:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.90
_OBJC_SELECTOR_REFERENCES_.90:
	.quad	l_OBJC_METH_VAR_NAME_.89

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.91:               ; @OBJC_METH_VAR_NAME_.91
	.asciz	"addCompletedHandler:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.92
_OBJC_SELECTOR_REFERENCES_.92:
	.quad	l_OBJC_METH_VAR_NAME_.91

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.93"
_OBJC_CLASSLIST_REFERENCES_$_.93:
	.quad	_OBJC_CLASS_$_MTLTextureDescriptor

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.94:               ; @OBJC_METH_VAR_NAME_.94
	.asciz	"texture2DDescriptorWithPixelFormat:width:height:mipmapped:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.95
_OBJC_SELECTOR_REFERENCES_.95:
	.quad	l_OBJC_METH_VAR_NAME_.94

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.96:               ; @OBJC_METH_VAR_NAME_.96
	.asciz	"textureCubeDescriptorWithPixelFormat:size:mipmapped:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.97
_OBJC_SELECTOR_REFERENCES_.97:
	.quad	l_OBJC_METH_VAR_NAME_.96

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.98:               ; @OBJC_METH_VAR_NAME_.98
	.asciz	"textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.99
_OBJC_SELECTOR_REFERENCES_.99:
	.quad	l_OBJC_METH_VAR_NAME_.98

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.100:              ; @OBJC_METH_VAR_NAME_.100
	.asciz	"width"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.101
_OBJC_SELECTOR_REFERENCES_.101:
	.quad	l_OBJC_METH_VAR_NAME_.100

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.102:              ; @OBJC_METH_VAR_NAME_.102
	.asciz	"setWidth:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.103
_OBJC_SELECTOR_REFERENCES_.103:
	.quad	l_OBJC_METH_VAR_NAME_.102

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.104:              ; @OBJC_METH_VAR_NAME_.104
	.asciz	"height"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.105
_OBJC_SELECTOR_REFERENCES_.105:
	.quad	l_OBJC_METH_VAR_NAME_.104

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.106:              ; @OBJC_METH_VAR_NAME_.106
	.asciz	"setHeight:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.107
_OBJC_SELECTOR_REFERENCES_.107:
	.quad	l_OBJC_METH_VAR_NAME_.106

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.108:              ; @OBJC_METH_VAR_NAME_.108
	.asciz	"depth"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.109
_OBJC_SELECTOR_REFERENCES_.109:
	.quad	l_OBJC_METH_VAR_NAME_.108

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.110:              ; @OBJC_METH_VAR_NAME_.110
	.asciz	"setDepth:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.111
_OBJC_SELECTOR_REFERENCES_.111:
	.quad	l_OBJC_METH_VAR_NAME_.110

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.112:              ; @OBJC_METH_VAR_NAME_.112
	.asciz	"mipmapLevelCount"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.113
_OBJC_SELECTOR_REFERENCES_.113:
	.quad	l_OBJC_METH_VAR_NAME_.112

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.114:              ; @OBJC_METH_VAR_NAME_.114
	.asciz	"setMipmapLevelCount:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.115
_OBJC_SELECTOR_REFERENCES_.115:
	.quad	l_OBJC_METH_VAR_NAME_.114

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.116:              ; @OBJC_METH_VAR_NAME_.116
	.asciz	"sampleCount"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.117
_OBJC_SELECTOR_REFERENCES_.117:
	.quad	l_OBJC_METH_VAR_NAME_.116

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.118:              ; @OBJC_METH_VAR_NAME_.118
	.asciz	"setSampleCount:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.119
_OBJC_SELECTOR_REFERENCES_.119:
	.quad	l_OBJC_METH_VAR_NAME_.118

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.120:              ; @OBJC_METH_VAR_NAME_.120
	.asciz	"arrayLength"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.121
_OBJC_SELECTOR_REFERENCES_.121:
	.quad	l_OBJC_METH_VAR_NAME_.120

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.122:              ; @OBJC_METH_VAR_NAME_.122
	.asciz	"setArrayLength:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.123
_OBJC_SELECTOR_REFERENCES_.123:
	.quad	l_OBJC_METH_VAR_NAME_.122

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.124:              ; @OBJC_METH_VAR_NAME_.124
	.asciz	"usage"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.125
_OBJC_SELECTOR_REFERENCES_.125:
	.quad	l_OBJC_METH_VAR_NAME_.124

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.126:              ; @OBJC_METH_VAR_NAME_.126
	.asciz	"setUsage:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.127
_OBJC_SELECTOR_REFERENCES_.127:
	.quad	l_OBJC_METH_VAR_NAME_.126

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.128:              ; @OBJC_METH_VAR_NAME_.128
	.asciz	"allowGPUOptimizedContents"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.129
_OBJC_SELECTOR_REFERENCES_.129:
	.quad	l_OBJC_METH_VAR_NAME_.128

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.130:              ; @OBJC_METH_VAR_NAME_.130
	.asciz	"setAllowGPUOptimizedContents:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.131
_OBJC_SELECTOR_REFERENCES_.131:
	.quad	l_OBJC_METH_VAR_NAME_.130

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.132:              ; @OBJC_METH_VAR_NAME_.132
	.asciz	"swizzle"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.133
_OBJC_SELECTOR_REFERENCES_.133:
	.quad	l_OBJC_METH_VAR_NAME_.132

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.134:              ; @OBJC_METH_VAR_NAME_.134
	.asciz	"setSwizzle:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.135
_OBJC_SELECTOR_REFERENCES_.135:
	.quad	l_OBJC_METH_VAR_NAME_.134

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.136:              ; @OBJC_METH_VAR_NAME_.136
	.asciz	"parentTexture"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.137
_OBJC_SELECTOR_REFERENCES_.137:
	.quad	l_OBJC_METH_VAR_NAME_.136

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.138:              ; @OBJC_METH_VAR_NAME_.138
	.asciz	"newTextureViewWithPixelFormat:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.139
_OBJC_SELECTOR_REFERENCES_.139:
	.quad	l_OBJC_METH_VAR_NAME_.138

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.140:              ; @OBJC_METH_VAR_NAME_.140
	.asciz	"iosurface"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.141
_OBJC_SELECTOR_REFERENCES_.141:
	.quad	l_OBJC_METH_VAR_NAME_.140

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.142:              ; @OBJC_METH_VAR_NAME_.142
	.asciz	"iosurfacePlane"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.143
_OBJC_SELECTOR_REFERENCES_.143:
	.quad	l_OBJC_METH_VAR_NAME_.142

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.144"
_OBJC_CLASSLIST_REFERENCES_$_.144:
	.quad	_OBJC_CLASS_$_MTLFunctionConstantValues

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.145:              ; @OBJC_METH_VAR_NAME_.145
	.asciz	"setConstantValue:type:atIndex:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.146
_OBJC_SELECTOR_REFERENCES_.146:
	.quad	l_OBJC_METH_VAR_NAME_.145

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.147:              ; @OBJC_METH_VAR_NAME_.147
	.asciz	"setConstantValues:type:withRange:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.148
_OBJC_SELECTOR_REFERENCES_.148:
	.quad	l_OBJC_METH_VAR_NAME_.147

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.149:              ; @OBJC_METH_VAR_NAME_.149
	.asciz	"setConstantValue:type:withName:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.150
_OBJC_SELECTOR_REFERENCES_.150:
	.quad	l_OBJC_METH_VAR_NAME_.149

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.151"
_OBJC_CLASSLIST_REFERENCES_$_.151:
	.quad	_OBJC_CLASS_$_MTLHeapDescriptor

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.152:              ; @OBJC_METH_VAR_NAME_.152
	.asciz	"size"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.153
_OBJC_SELECTOR_REFERENCES_.153:
	.quad	l_OBJC_METH_VAR_NAME_.152

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.154:              ; @OBJC_METH_VAR_NAME_.154
	.asciz	"setSize:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.155
_OBJC_SELECTOR_REFERENCES_.155:
	.quad	l_OBJC_METH_VAR_NAME_.154

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.156:              ; @OBJC_METH_VAR_NAME_.156
	.asciz	"type"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.157
_OBJC_SELECTOR_REFERENCES_.157:
	.quad	l_OBJC_METH_VAR_NAME_.156

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.158:              ; @OBJC_METH_VAR_NAME_.158
	.asciz	"setType:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.159
_OBJC_SELECTOR_REFERENCES_.159:
	.quad	l_OBJC_METH_VAR_NAME_.158

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.160:              ; @OBJC_METH_VAR_NAME_.160
	.asciz	"currentAllocatedSize"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.161
_OBJC_SELECTOR_REFERENCES_.161:
	.quad	l_OBJC_METH_VAR_NAME_.160

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.162:              ; @OBJC_METH_VAR_NAME_.162
	.asciz	"maxAvailableSizeWithAlignment:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.163
_OBJC_SELECTOR_REFERENCES_.163:
	.quad	l_OBJC_METH_VAR_NAME_.162

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.164:              ; @OBJC_METH_VAR_NAME_.164
	.asciz	"newBufferWithBytes:length:options:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.165
_OBJC_SELECTOR_REFERENCES_.165:
	.quad	l_OBJC_METH_VAR_NAME_.164

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.166:              ; @OBJC_METH_VAR_NAME_.166
	.asciz	"newDepthStencilStateWithDescriptor:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.167
_OBJC_SELECTOR_REFERENCES_.167:
	.quad	l_OBJC_METH_VAR_NAME_.166

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.168:              ; @OBJC_METH_VAR_NAME_.168
	.asciz	"newFunctionWithName:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.169
_OBJC_SELECTOR_REFERENCES_.169:
	.quad	l_OBJC_METH_VAR_NAME_.168

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.170:              ; @OBJC_METH_VAR_NAME_.170
	.asciz	"newFunctionWithName:constantValues:error:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.171
_OBJC_SELECTOR_REFERENCES_.171:
	.quad	l_OBJC_METH_VAR_NAME_.170

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.172:              ; @OBJC_METH_VAR_NAME_.172
	.asciz	"functionNames"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.173
_OBJC_SELECTOR_REFERENCES_.173:
	.quad	l_OBJC_METH_VAR_NAME_.172

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.174:              ; @OBJC_METH_VAR_NAME_.174
	.asciz	"newArgumentEncoderWithBufferIndex:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.175
_OBJC_SELECTOR_REFERENCES_.175:
	.quad	l_OBJC_METH_VAR_NAME_.174

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.176:              ; @OBJC_METH_VAR_NAME_.176
	.asciz	"encodedLength"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.177
_OBJC_SELECTOR_REFERENCES_.177:
	.quad	l_OBJC_METH_VAR_NAME_.176

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.178:              ; @OBJC_METH_VAR_NAME_.178
	.asciz	"alignment"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.179
_OBJC_SELECTOR_REFERENCES_.179:
	.quad	l_OBJC_METH_VAR_NAME_.178

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.180:              ; @OBJC_METH_VAR_NAME_.180
	.asciz	"options"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.181
_OBJC_SELECTOR_REFERENCES_.181:
	.quad	l_OBJC_METH_VAR_NAME_.180

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.182"
_OBJC_CLASSLIST_REFERENCES_$_.182:
	.quad	_OBJC_CLASS_$_MTLFunctionDescriptor

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.183:              ; @OBJC_METH_VAR_NAME_.183
	.asciz	"functionDescriptor"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.184
_OBJC_SELECTOR_REFERENCES_.184:
	.quad	l_OBJC_METH_VAR_NAME_.183

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.185:              ; @OBJC_METH_VAR_NAME_.185
	.asciz	"isBlendingEnabled"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.186
_OBJC_SELECTOR_REFERENCES_.186:
	.quad	l_OBJC_METH_VAR_NAME_.185

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.187:              ; @OBJC_METH_VAR_NAME_.187
	.asciz	"setBlendingEnabled:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.188
_OBJC_SELECTOR_REFERENCES_.188:
	.quad	l_OBJC_METH_VAR_NAME_.187

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.189:              ; @OBJC_METH_VAR_NAME_.189
	.asciz	"sourceRGBBlendFactor"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.190
_OBJC_SELECTOR_REFERENCES_.190:
	.quad	l_OBJC_METH_VAR_NAME_.189

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.191:              ; @OBJC_METH_VAR_NAME_.191
	.asciz	"setSourceRGBBlendFactor:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.192
_OBJC_SELECTOR_REFERENCES_.192:
	.quad	l_OBJC_METH_VAR_NAME_.191

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.193:              ; @OBJC_METH_VAR_NAME_.193
	.asciz	"destinationRGBBlendFactor"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.194
_OBJC_SELECTOR_REFERENCES_.194:
	.quad	l_OBJC_METH_VAR_NAME_.193

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.195:              ; @OBJC_METH_VAR_NAME_.195
	.asciz	"setDestinationRGBBlendFactor:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.196
_OBJC_SELECTOR_REFERENCES_.196:
	.quad	l_OBJC_METH_VAR_NAME_.195

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.197:              ; @OBJC_METH_VAR_NAME_.197
	.asciz	"rgbBlendOperation"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.198
_OBJC_SELECTOR_REFERENCES_.198:
	.quad	l_OBJC_METH_VAR_NAME_.197

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.199:              ; @OBJC_METH_VAR_NAME_.199
	.asciz	"setRgbBlendOperation:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.200
_OBJC_SELECTOR_REFERENCES_.200:
	.quad	l_OBJC_METH_VAR_NAME_.199

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.201:              ; @OBJC_METH_VAR_NAME_.201
	.asciz	"sourceAlphaBlendFactor"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.202
_OBJC_SELECTOR_REFERENCES_.202:
	.quad	l_OBJC_METH_VAR_NAME_.201

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.203:              ; @OBJC_METH_VAR_NAME_.203
	.asciz	"setSourceAlphaBlendFactor:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.204
_OBJC_SELECTOR_REFERENCES_.204:
	.quad	l_OBJC_METH_VAR_NAME_.203

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.205:              ; @OBJC_METH_VAR_NAME_.205
	.asciz	"destinationAlphaBlendFactor"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.206
_OBJC_SELECTOR_REFERENCES_.206:
	.quad	l_OBJC_METH_VAR_NAME_.205

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.207:              ; @OBJC_METH_VAR_NAME_.207
	.asciz	"setDestinationAlphaBlendFactor:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.208
_OBJC_SELECTOR_REFERENCES_.208:
	.quad	l_OBJC_METH_VAR_NAME_.207

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.209:              ; @OBJC_METH_VAR_NAME_.209
	.asciz	"alphaBlendOperation"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.210
_OBJC_SELECTOR_REFERENCES_.210:
	.quad	l_OBJC_METH_VAR_NAME_.209

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.211:              ; @OBJC_METH_VAR_NAME_.211
	.asciz	"setAlphaBlendOperation:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.212
_OBJC_SELECTOR_REFERENCES_.212:
	.quad	l_OBJC_METH_VAR_NAME_.211

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.213:              ; @OBJC_METH_VAR_NAME_.213
	.asciz	"writeMask"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.214
_OBJC_SELECTOR_REFERENCES_.214:
	.quad	l_OBJC_METH_VAR_NAME_.213

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.215:              ; @OBJC_METH_VAR_NAME_.215
	.asciz	"setWriteMask:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.216
_OBJC_SELECTOR_REFERENCES_.216:
	.quad	l_OBJC_METH_VAR_NAME_.215

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.217:              ; @OBJC_METH_VAR_NAME_.217
	.asciz	"vertexArguments"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.218
_OBJC_SELECTOR_REFERENCES_.218:
	.quad	l_OBJC_METH_VAR_NAME_.217

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.219:              ; @OBJC_METH_VAR_NAME_.219
	.asciz	"fragmentArguments"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.220
_OBJC_SELECTOR_REFERENCES_.220:
	.quad	l_OBJC_METH_VAR_NAME_.219

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.221:              ; @OBJC_METH_VAR_NAME_.221
	.asciz	"tileArguments"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.222
_OBJC_SELECTOR_REFERENCES_.222:
	.quad	l_OBJC_METH_VAR_NAME_.221

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.223"
_OBJC_CLASSLIST_REFERENCES_$_.223:
	.quad	_OBJC_CLASS_$_MTLRenderPipelineDescriptor

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.224:              ; @OBJC_METH_VAR_NAME_.224
	.asciz	"vertexFunction"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.225
_OBJC_SELECTOR_REFERENCES_.225:
	.quad	l_OBJC_METH_VAR_NAME_.224

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.226:              ; @OBJC_METH_VAR_NAME_.226
	.asciz	"setVertexFunction:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.227
_OBJC_SELECTOR_REFERENCES_.227:
	.quad	l_OBJC_METH_VAR_NAME_.226

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.228:              ; @OBJC_METH_VAR_NAME_.228
	.asciz	"fragmentFunction"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.229
_OBJC_SELECTOR_REFERENCES_.229:
	.quad	l_OBJC_METH_VAR_NAME_.228

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.230:              ; @OBJC_METH_VAR_NAME_.230
	.asciz	"setFragmentFunction:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.231
_OBJC_SELECTOR_REFERENCES_.231:
	.quad	l_OBJC_METH_VAR_NAME_.230

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.232:              ; @OBJC_METH_VAR_NAME_.232
	.asciz	"rasterSampleCount"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.233
_OBJC_SELECTOR_REFERENCES_.233:
	.quad	l_OBJC_METH_VAR_NAME_.232

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.234:              ; @OBJC_METH_VAR_NAME_.234
	.asciz	"setRasterSampleCount:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.235
_OBJC_SELECTOR_REFERENCES_.235:
	.quad	l_OBJC_METH_VAR_NAME_.234

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.236:              ; @OBJC_METH_VAR_NAME_.236
	.asciz	"maxTotalThreadsPerThreadgroup"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.237
_OBJC_SELECTOR_REFERENCES_.237:
	.quad	l_OBJC_METH_VAR_NAME_.236

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.238:              ; @OBJC_METH_VAR_NAME_.238
	.asciz	"threadgroupSizeMatchesTileSize"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.239
_OBJC_SELECTOR_REFERENCES_.239:
	.quad	l_OBJC_METH_VAR_NAME_.238

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.240:              ; @OBJC_METH_VAR_NAME_.240
	.asciz	"imageblockSampleLength"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.241
_OBJC_SELECTOR_REFERENCES_.241:
	.quad	l_OBJC_METH_VAR_NAME_.240

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.242"
_OBJC_CLASSLIST_REFERENCES_$_.242:
	.quad	_OBJC_CLASS_$_MTLComputePipelineDescriptor

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.243:              ; @OBJC_METH_VAR_NAME_.243
	.asciz	"threadExecutionWidth"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.244
_OBJC_SELECTOR_REFERENCES_.244:
	.quad	l_OBJC_METH_VAR_NAME_.243

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.245:              ; @OBJC_METH_VAR_NAME_.245
	.asciz	"staticThreadgroupMemoryLength"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.246
_OBJC_SELECTOR_REFERENCES_.246:
	.quad	l_OBJC_METH_VAR_NAME_.245

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.247:              ; @OBJC_METH_VAR_NAME_.247
	.asciz	"endEncoding"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.248
_OBJC_SELECTOR_REFERENCES_.248:
	.quad	l_OBJC_METH_VAR_NAME_.247

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.249:              ; @OBJC_METH_VAR_NAME_.249
	.asciz	"insertDebugSignpost:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.250
_OBJC_SELECTOR_REFERENCES_.250:
	.quad	l_OBJC_METH_VAR_NAME_.249

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.251:              ; @OBJC_METH_VAR_NAME_.251
	.asciz	"pushDebugGroup:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.252
_OBJC_SELECTOR_REFERENCES_.252:
	.quad	l_OBJC_METH_VAR_NAME_.251

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.253:              ; @OBJC_METH_VAR_NAME_.253
	.asciz	"popDebugGroup"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.254
_OBJC_SELECTOR_REFERENCES_.254:
	.quad	l_OBJC_METH_VAR_NAME_.253

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.255:              ; @OBJC_METH_VAR_NAME_.255
	.asciz	"renderCommandEncoder"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.256
_OBJC_SELECTOR_REFERENCES_.256:
	.quad	l_OBJC_METH_VAR_NAME_.255

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols
