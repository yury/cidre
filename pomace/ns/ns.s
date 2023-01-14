	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 13, 0	sdk_version 13, 1
	.globl	_wsel_scheduleInRunLoop_forMode ; -- Begin function wsel_scheduleInRunLoop_forMode
	.p2align	2
_wsel_scheduleInRunLoop_forMode:        ; @wsel_scheduleInRunLoop_forMode
	.cfi_startproc
; %bb.0:
	mov	x3, x2
	mov	x2, x1
	b	"_objc_msgSend$scheduleInRunLoop:forMode:"
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_removeFromRunLoop_forMode ; -- Begin function wsel_removeFromRunLoop_forMode
	.p2align	2
_wsel_removeFromRunLoop_forMode:        ; @wsel_removeFromRunLoop_forMode
	.cfi_startproc
; %bb.0:
	mov	x3, x2
	mov	x2, x1
	b	"_objc_msgSend$removeFromRunLoop:forMode:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSPort_port                    ; -- Begin function NSPort_port
	.p2align	2
_NSPort_port:                           ; @NSPort_port
	.cfi_startproc
; %bb.0:
Lloh0:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_@PAGE
Lloh1:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_@PAGEOFF]
	b	_objc_msgSend$port
	.loh AdrpLdr	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_machPort                  ; -- Begin function rsel_machPort
	.p2align	2
_rsel_machPort:                         ; @rsel_machPort
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$machPort
	.cfi_endproc
                                        ; -- End function
	.globl	_make_mach_port_delegate        ; -- Begin function make_mach_port_delegate
	.p2align	2
_make_mach_port_delegate:               ; @make_mach_port_delegate
	.cfi_startproc
; %bb.0:
	stp	x20, x19, [sp, #-32]!           ; 16-byte Folded Spill
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	x19, x0
Lloh2:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.1@PAGE
Lloh3:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.1@PAGEOFF]
	bl	_objc_opt_new
	ldr	q0, [x19]
	stur	q0, [x0, #8]
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	ret
	.loh AdrpLdr	Lloh2, Lloh3
	.cfi_endproc
                                        ; -- End function
	.globl	_NSProcessInfo_processInfo      ; -- Begin function NSProcessInfo_processInfo
	.p2align	2
_NSProcessInfo_processInfo:             ; @NSProcessInfo_processInfo
	.cfi_startproc
; %bb.0:
Lloh4:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.2@PAGE
Lloh5:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.2@PAGEOFF]
	b	_objc_msgSend$processInfo
	.loh AdrpLdr	Lloh4, Lloh5
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_isLowPowerModeEnabled     ; -- Begin function rsel_isLowPowerModeEnabled
	.p2align	2
_rsel_isLowPowerModeEnabled:            ; @rsel_isLowPowerModeEnabled
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$isLowPowerModeEnabled
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_processorCount            ; -- Begin function rsel_processorCount
	.p2align	2
_rsel_processorCount:                   ; @rsel_processorCount
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$processorCount
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_activeProcessorCount      ; -- Begin function rsel_activeProcessorCount
	.p2align	2
_rsel_activeProcessorCount:             ; @rsel_activeProcessorCount
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$activeProcessorCount
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_isMacCatalystApp          ; -- Begin function rsel_isMacCatalystApp
	.p2align	2
_rsel_isMacCatalystApp:                 ; @rsel_isMacCatalystApp
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$isMacCatalystApp
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_isiOSAppOnMac             ; -- Begin function rsel_isiOSAppOnMac
	.p2align	2
_rsel_isiOSAppOnMac:                    ; @rsel_isiOSAppOnMac
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$isiOSAppOnMac
	.cfi_endproc
                                        ; -- End function
	.globl	_cidre_raise_exception          ; -- Begin function cidre_raise_exception
	.p2align	2
_cidre_raise_exception:                 ; @cidre_raise_exception
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh6:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.3@PAGE
Lloh7:
	ldr	x8, [x8, _OBJC_CLASSLIST_REFERENCES_$_.3@PAGEOFF]
Lloh8:
	adrp	x9, _NSGenericException@GOTPAGE
Lloh9:
	ldr	x9, [x9, _NSGenericException@GOTPAGEOFF]
Lloh10:
	ldr	x2, [x9]
	str	x0, [sp]
Lloh11:
	adrp	x3, l__unnamed_cfstring_@PAGE
Lloh12:
	add	x3, x3, l__unnamed_cfstring_@PAGEOFF
	mov	x0, x8
	bl	"_objc_msgSend$raise:format:"
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh11, Lloh12
	.loh AdrpLdrGotLdr	Lloh8, Lloh9, Lloh10
	.loh AdrpLdr	Lloh6, Lloh7
	.cfi_endproc
                                        ; -- End function
	.globl	_cidre_throw_exception          ; -- Begin function cidre_throw_exception
	.p2align	2
_cidre_throw_exception:                 ; @cidre_throw_exception
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	_objc_exception_throw
	.cfi_endproc
                                        ; -- End function
	.globl	_cidre_try_catch                ; -- Begin function cidre_try_catch
	.p2align	2
_cidre_try_catch:                       ; @cidre_try_catch
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, ___objc_personality_v0
	.cfi_lsda 16, Lexception0
; %bb.0:
	stp	x20, x19, [sp, #-32]!           ; 16-byte Folded Spill
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	x8, x0
Ltmp0:
	mov	x0, x1
	blr	x8
Ltmp1:
; %bb.1:
	mov	x0, #0
LBB13_2:
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	ret
LBB13_3:
Ltmp2:
	bl	_objc_begin_catch
	mov	x19, x0
	bl	_objc_end_catch
	mov	x0, x19
	b	LBB13_2
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table13:
Lexception0:
	.byte	255                             ; @LPStart Encoding = omit
	.byte	155                             ; @TType Encoding = indirect pcrel sdata4
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1                               ; Call site Encoding = uleb128
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0             ; >> Call Site 1 <<
	.uleb128 Ltmp1-Ltmp0                    ;   Call between Ltmp0 and Ltmp1
	.uleb128 Ltmp2-Lfunc_begin0             ;     jumps to Ltmp2
	.byte	1                               ;   On action: 1
	.uleb128 Ltmp1-Lfunc_begin0             ; >> Call Site 2 <<
	.uleb128 Lfunc_end0-Ltmp1               ;   Call between Ltmp1 and Lfunc_end0
	.byte	0                               ;     has no landing pad
	.byte	0                               ;   On action: cleanup
Lcst_end0:
	.byte	1                               ; >> Action Record 1 <<
                                        ;   Catch TypeInfo 1
	.byte	0                               ;   No further actions
	.p2align	2
                                        ; >> Catch TypeInfos <<
Ltmp3:                                  ; TypeInfo 1
	.long	_OBJC_EHTYPE_id@GOT-Ltmp3
Lttbase0:
	.p2align	2
                                        ; -- End function
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_NSURLSession_sharedSession     ; -- Begin function NSURLSession_sharedSession
	.p2align	2
_NSURLSession_sharedSession:            ; @NSURLSession_sharedSession
	.cfi_startproc
; %bb.0:
Lloh13:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.4@PAGE
Lloh14:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.4@PAGEOFF]
	b	_objc_msgSend$sharedSession
	.loh AdrpLdr	Lloh13, Lloh14
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_dataTaskWithURL           ; -- Begin function rsel_dataTaskWithURL
	.p2align	2
_rsel_dataTaskWithURL:                  ; @rsel_dataTaskWithURL
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$dataTaskWithURL:"
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_dataTaskWithRequest       ; -- Begin function rsel_dataTaskWithRequest
	.p2align	2
_rsel_dataTaskWithRequest:              ; @rsel_dataTaskWithRequest
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$dataTaskWithRequest:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_wsel_resume   ; -- Begin function NSURLSessionTask_wsel_resume
	.p2align	2
_NSURLSessionTask_wsel_resume:          ; @NSURLSessionTask_wsel_resume
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$resume
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_wsel_cancel   ; -- Begin function NSURLSessionTask_wsel_cancel
	.p2align	2
_NSURLSessionTask_wsel_cancel:          ; @NSURLSessionTask_wsel_cancel
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$cancel
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_wsel_suspend  ; -- Begin function NSURLSessionTask_wsel_suspend
	.p2align	2
_NSURLSessionTask_wsel_suspend:         ; @NSURLSessionTask_wsel_suspend
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$suspend
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_rsel_state    ; -- Begin function NSURLSessionTask_rsel_state
	.p2align	2
_NSURLSessionTask_rsel_state:           ; @NSURLSessionTask_rsel_state
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$state
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_rsel_error    ; -- Begin function NSURLSessionTask_rsel_error
	.p2align	2
_NSURLSessionTask_rsel_error:           ; @NSURLSessionTask_rsel_error
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$error
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_rsel_taskIdentifier ; -- Begin function NSURLSessionTask_rsel_taskIdentifier
	.p2align	2
_NSURLSessionTask_rsel_taskIdentifier:  ; @NSURLSessionTask_rsel_taskIdentifier
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$taskIdentifier
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_rsel_originalRequest ; -- Begin function NSURLSessionTask_rsel_originalRequest
	.p2align	2
_NSURLSessionTask_rsel_originalRequest: ; @NSURLSessionTask_rsel_originalRequest
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$originalRequest
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_rsel_currentRequest ; -- Begin function NSURLSessionTask_rsel_currentRequest
	.p2align	2
_NSURLSessionTask_rsel_currentRequest:  ; @NSURLSessionTask_rsel_currentRequest
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$currentRequest
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_rsel_response ; -- Begin function NSURLSessionTask_rsel_response
	.p2align	2
_NSURLSessionTask_rsel_response:        ; @NSURLSessionTask_rsel_response
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$response
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_rsel_priority ; -- Begin function NSURLSessionTask_rsel_priority
	.p2align	2
_NSURLSessionTask_rsel_priority:        ; @NSURLSessionTask_rsel_priority
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$priority
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionTask_wsel_setPriority ; -- Begin function NSURLSessionTask_wsel_setPriority
	.p2align	2
_NSURLSessionTask_wsel_setPriority:     ; @NSURLSessionTask_wsel_setPriority
	.cfi_startproc
; %bb.0:
	b	"_objc_msgSend$setPriority:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_requestWithURL    ; -- Begin function NSURLRequest_requestWithURL
	.p2align	2
_NSURLRequest_requestWithURL:           ; @NSURLRequest_requestWithURL
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh15:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.5@PAGE
Lloh16:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.5@PAGEOFF]
	b	"_objc_msgSend$requestWithURL:"
	.loh AdrpLdr	Lloh15, Lloh16
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_requestWithURL_cachePolicy_timeoutInterval ; -- Begin function NSURLRequest_requestWithURL_cachePolicy_timeoutInterval
	.p2align	2
_NSURLRequest_requestWithURL_cachePolicy_timeoutInterval: ; @NSURLRequest_requestWithURL_cachePolicy_timeoutInterval
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh17:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.5@PAGE
Lloh18:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.5@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$requestWithURL:cachePolicy:timeoutInterval:"
	.loh AdrpLdr	Lloh17, Lloh18
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_cachePolicy  ; -- Begin function NSURLRequest_rsel_cachePolicy
	.p2align	2
_NSURLRequest_rsel_cachePolicy:         ; @NSURLRequest_rsel_cachePolicy
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$cachePolicy
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_timeoutInterval ; -- Begin function NSURLRequest_rsel_timeoutInterval
	.p2align	2
_NSURLRequest_rsel_timeoutInterval:     ; @NSURLRequest_rsel_timeoutInterval
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$timeoutInterval
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_networkServiceType ; -- Begin function NSURLRequest_rsel_networkServiceType
	.p2align	2
_NSURLRequest_rsel_networkServiceType:  ; @NSURLRequest_rsel_networkServiceType
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$networkServiceType
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_allowsCellularAccess ; -- Begin function NSURLRequest_rsel_allowsCellularAccess
	.p2align	2
_NSURLRequest_rsel_allowsCellularAccess: ; @NSURLRequest_rsel_allowsCellularAccess
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$allowsCellularAccess
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_allowsExpensiveNetworkAccess ; -- Begin function NSURLRequest_rsel_allowsExpensiveNetworkAccess
	.p2align	2
_NSURLRequest_rsel_allowsExpensiveNetworkAccess: ; @NSURLRequest_rsel_allowsExpensiveNetworkAccess
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$allowsExpensiveNetworkAccess
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_allowsConstrainedNetworkAccess ; -- Begin function NSURLRequest_rsel_allowsConstrainedNetworkAccess
	.p2align	2
_NSURLRequest_rsel_allowsConstrainedNetworkAccess: ; @NSURLRequest_rsel_allowsConstrainedNetworkAccess
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$allowsConstrainedNetworkAccess
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_assumesHTTP3Capable ; -- Begin function NSURLRequest_rsel_assumesHTTP3Capable
	.p2align	2
_NSURLRequest_rsel_assumesHTTP3Capable: ; @NSURLRequest_rsel_assumesHTTP3Capable
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$assumesHTTP3Capable
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_attribution  ; -- Begin function NSURLRequest_rsel_attribution
	.p2align	2
_NSURLRequest_rsel_attribution:         ; @NSURLRequest_rsel_attribution
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$attribution
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_requiresDNSSECValidation ; -- Begin function NSURLRequest_rsel_requiresDNSSECValidation
	.p2align	2
_NSURLRequest_rsel_requiresDNSSECValidation: ; @NSURLRequest_rsel_requiresDNSSECValidation
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$requiresDNSSECValidation
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_HTTPMethod   ; -- Begin function NSURLRequest_rsel_HTTPMethod
	.p2align	2
_NSURLRequest_rsel_HTTPMethod:          ; @NSURLRequest_rsel_HTTPMethod
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$HTTPMethod
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_allHTTPHeaderFields ; -- Begin function NSURLRequest_rsel_allHTTPHeaderFields
	.p2align	2
_NSURLRequest_rsel_allHTTPHeaderFields: ; @NSURLRequest_rsel_allHTTPHeaderFields
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$allHTTPHeaderFields
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_valueForHTTPHeaderField ; -- Begin function NSURLRequest_rsel_valueForHTTPHeaderField
	.p2align	2
_NSURLRequest_rsel_valueForHTTPHeaderField: ; @NSURLRequest_rsel_valueForHTTPHeaderField
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$valueForHTTPHeaderField:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLRequest_rsel_HTTPBody     ; -- Begin function NSURLRequest_rsel_HTTPBody
	.p2align	2
_NSURLRequest_rsel_HTTPBody:            ; @NSURLRequest_rsel_HTTPBody
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$HTTPBody
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_requestWithURL ; -- Begin function NSMutableURLRequest_requestWithURL
	.p2align	2
_NSMutableURLRequest_requestWithURL:    ; @NSMutableURLRequest_requestWithURL
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh19:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.6@PAGE
Lloh20:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.6@PAGEOFF]
	b	"_objc_msgSend$requestWithURL:"
	.loh AdrpLdr	Lloh19, Lloh20
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_requestWithURL_cachePolicy_timeoutInterval ; -- Begin function NSMutableURLRequest_requestWithURL_cachePolicy_timeoutInterval
	.p2align	2
_NSMutableURLRequest_requestWithURL_cachePolicy_timeoutInterval: ; @NSMutableURLRequest_requestWithURL_cachePolicy_timeoutInterval
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh21:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.6@PAGE
Lloh22:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.6@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$requestWithURL:cachePolicy:timeoutInterval:"
	.loh AdrpLdr	Lloh21, Lloh22
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_rsel_cachePolicy ; -- Begin function NSMutableURLRequest_rsel_cachePolicy
	.p2align	2
_NSMutableURLRequest_rsel_cachePolicy:  ; @NSMutableURLRequest_rsel_cachePolicy
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$cachePolicy
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setCachePolicy ; -- Begin function NSMutableURLRequest_wsel_setCachePolicy
	.p2align	2
_NSMutableURLRequest_wsel_setCachePolicy: ; @NSMutableURLRequest_wsel_setCachePolicy
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setCachePolicy:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setTimeoutInterval ; -- Begin function NSMutableURLRequest_wsel_setTimeoutInterval
	.p2align	2
_NSMutableURLRequest_wsel_setTimeoutInterval: ; @NSMutableURLRequest_wsel_setTimeoutInterval
	.cfi_startproc
; %bb.0:
	b	"_objc_msgSend$setTimeoutInterval:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setNetworkServiceType ; -- Begin function NSMutableURLRequest_wsel_setNetworkServiceType
	.p2align	2
_NSMutableURLRequest_wsel_setNetworkServiceType: ; @NSMutableURLRequest_wsel_setNetworkServiceType
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setNetworkServiceType:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setAllowsCellularAccess ; -- Begin function NSMutableURLRequest_wsel_setAllowsCellularAccess
	.p2align	2
_NSMutableURLRequest_wsel_setAllowsCellularAccess: ; @NSMutableURLRequest_wsel_setAllowsCellularAccess
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setAllowsCellularAccess:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setAllowsExpensiveNetworkAccess ; -- Begin function NSMutableURLRequest_wsel_setAllowsExpensiveNetworkAccess
	.p2align	2
_NSMutableURLRequest_wsel_setAllowsExpensiveNetworkAccess: ; @NSMutableURLRequest_wsel_setAllowsExpensiveNetworkAccess
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setAllowsExpensiveNetworkAccess:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setAllowsConstrainedNetworkAccess ; -- Begin function NSMutableURLRequest_wsel_setAllowsConstrainedNetworkAccess
	.p2align	2
_NSMutableURLRequest_wsel_setAllowsConstrainedNetworkAccess: ; @NSMutableURLRequest_wsel_setAllowsConstrainedNetworkAccess
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setAllowsConstrainedNetworkAccess:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setAssumesHTTP3Capable ; -- Begin function NSMutableURLRequest_wsel_setAssumesHTTP3Capable
	.p2align	2
_NSMutableURLRequest_wsel_setAssumesHTTP3Capable: ; @NSMutableURLRequest_wsel_setAssumesHTTP3Capable
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setAssumesHTTP3Capable:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setAttribution ; -- Begin function NSMutableURLRequest_wsel_setAttribution
	.p2align	2
_NSMutableURLRequest_wsel_setAttribution: ; @NSMutableURLRequest_wsel_setAttribution
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setAttribution:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setRequiresDNSSECValidation ; -- Begin function NSMutableURLRequest_wsel_setRequiresDNSSECValidation
	.p2align	2
_NSMutableURLRequest_wsel_setRequiresDNSSECValidation: ; @NSMutableURLRequest_wsel_setRequiresDNSSECValidation
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setRequiresDNSSECValidation:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setHTTPMethod ; -- Begin function NSMutableURLRequest_wsel_setHTTPMethod
	.p2align	2
_NSMutableURLRequest_wsel_setHTTPMethod: ; @NSMutableURLRequest_wsel_setHTTPMethod
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setHTTPMethod:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setAllHTTPHeaderFields ; -- Begin function NSMutableURLRequest_wsel_setAllHTTPHeaderFields
	.p2align	2
_NSMutableURLRequest_wsel_setAllHTTPHeaderFields: ; @NSMutableURLRequest_wsel_setAllHTTPHeaderFields
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setAllHTTPHeaderFields:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSMutableURLRequest_wsel_setHTTPBody ; -- Begin function NSMutableURLRequest_wsel_setHTTPBody
	.p2align	2
_NSMutableURLRequest_wsel_setHTTPBody:  ; @NSMutableURLRequest_wsel_setHTTPBody
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setHTTPBody:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLResponse_initWithURL_MIMEType_expectedContentLength_textEncodingName ; -- Begin function NSURLResponse_initWithURL_MIMEType_expectedContentLength_textEncodingName
	.p2align	2
_NSURLResponse_initWithURL_MIMEType_expectedContentLength_textEncodingName: ; @NSURLResponse_initWithURL_MIMEType_expectedContentLength_textEncodingName
	.cfi_startproc
; %bb.0:
	stp	x22, x21, [sp, #-48]!           ; 16-byte Folded Spill
	stp	x20, x19, [sp, #16]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	mov	x19, x3
	mov	x20, x2
	mov	x21, x1
	mov	x22, x0
Lloh23:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.7@PAGE
Lloh24:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.7@PAGEOFF]
	bl	_objc_alloc
	mov	x2, x22
	mov	x3, x21
	mov	x4, x20
	mov	x5, x19
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #16]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp], #48             ; 16-byte Folded Reload
	b	"_objc_msgSend$initWithURL:MIMEType:expectedContentLength:textEncodingName:"
	.loh AdrpLdr	Lloh23, Lloh24
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionWebSocketMessage_initWithData ; -- Begin function NSURLSessionWebSocketMessage_initWithData
	.p2align	2
_NSURLSessionWebSocketMessage_initWithData: ; @NSURLSessionWebSocketMessage_initWithData
	.cfi_startproc
; %bb.0:
	stp	x20, x19, [sp, #-32]!           ; 16-byte Folded Spill
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	x19, x0
Lloh25:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.8@PAGE
Lloh26:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.8@PAGEOFF]
	bl	_objc_alloc
	mov	x2, x19
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	b	"_objc_msgSend$initWithData:"
	.loh AdrpLdr	Lloh25, Lloh26
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionWebSocketMessage_initWithString ; -- Begin function NSURLSessionWebSocketMessage_initWithString
	.p2align	2
_NSURLSessionWebSocketMessage_initWithString: ; @NSURLSessionWebSocketMessage_initWithString
	.cfi_startproc
; %bb.0:
	stp	x20, x19, [sp, #-32]!           ; 16-byte Folded Spill
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	x19, x0
Lloh27:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.8@PAGE
Lloh28:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.8@PAGEOFF]
	bl	_objc_alloc
	mov	x2, x19
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	b	"_objc_msgSend$initWithString:"
	.loh AdrpLdr	Lloh27, Lloh28
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionWebSocketMessage_rsel_type ; -- Begin function NSURLSessionWebSocketMessage_rsel_type
	.p2align	2
_NSURLSessionWebSocketMessage_rsel_type: ; @NSURLSessionWebSocketMessage_rsel_type
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$type
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionWebSocketMessage_rsel_data ; -- Begin function NSURLSessionWebSocketMessage_rsel_data
	.p2align	2
_NSURLSessionWebSocketMessage_rsel_data: ; @NSURLSessionWebSocketMessage_rsel_data
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$data
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLSessionWebSocketMessage_rsel_string ; -- Begin function NSURLSessionWebSocketMessage_rsel_string
	.p2align	2
_NSURLSessionWebSocketMessage_rsel_string: ; @NSURLSessionWebSocketMessage_rsel_string
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$string
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLCache_sharedURLCache      ; -- Begin function NSURLCache_sharedURLCache
	.p2align	2
_NSURLCache_sharedURLCache:             ; @NSURLCache_sharedURLCache
	.cfi_startproc
; %bb.0:
Lloh29:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.9@PAGE
Lloh30:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.9@PAGEOFF]
	b	_objc_msgSend$sharedURLCache
	.loh AdrpLdr	Lloh29, Lloh30
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURLCache_initWithMemoryCapacity_diskCapacity_directoryURL ; -- Begin function NSURLCache_initWithMemoryCapacity_diskCapacity_directoryURL
	.p2align	2
_NSURLCache_initWithMemoryCapacity_diskCapacity_directoryURL: ; @NSURLCache_initWithMemoryCapacity_diskCapacity_directoryURL
	.cfi_startproc
; %bb.0:
	stp	x22, x21, [sp, #-48]!           ; 16-byte Folded Spill
	stp	x20, x19, [sp, #16]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	mov	x19, x2
	mov	x20, x1
	mov	x21, x0
Lloh31:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.9@PAGE
Lloh32:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.9@PAGEOFF]
	bl	_objc_alloc
	mov	x2, x21
	mov	x3, x20
	mov	x4, x19
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #16]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp], #48             ; 16-byte Folded Reload
	b	"_objc_msgSend$initWithMemoryCapacity:diskCapacity:directoryURL:"
	.loh AdrpLdr	Lloh31, Lloh32
	.cfi_endproc
                                        ; -- End function
	.globl	_NSData_dataWithContentsOfFile_options_error ; -- Begin function NSData_dataWithContentsOfFile_options_error
	.p2align	2
_NSData_dataWithContentsOfFile_options_error: ; @NSData_dataWithContentsOfFile_options_error
	.cfi_startproc
; %bb.0:
	mov	x4, x2
	mov	x2, x0
Lloh33:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.10@PAGE
Lloh34:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.10@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$dataWithContentsOfFile:options:error:"
	.loh AdrpLdr	Lloh33, Lloh34
	.cfi_endproc
                                        ; -- End function
	.globl	_NSData_dataWithContentsOfURL_options_error ; -- Begin function NSData_dataWithContentsOfURL_options_error
	.p2align	2
_NSData_dataWithContentsOfURL_options_error: ; @NSData_dataWithContentsOfURL_options_error
	.cfi_startproc
; %bb.0:
	mov	x4, x2
	mov	x2, x0
Lloh35:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.10@PAGE
Lloh36:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.10@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$dataWithContentsOfURL:options:error:"
	.loh AdrpLdr	Lloh35, Lloh36
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_writeToFile_atomically    ; -- Begin function rsel_writeToFile_atomically
	.p2align	2
_rsel_writeToFile_atomically:           ; @rsel_writeToFile_atomically
	.cfi_startproc
; %bb.0:
	mov	x3, x2
	mov	x2, x1
	b	"_objc_msgSend$writeToFile:atomically:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSNumber_numberWithUnsignedLongLong ; -- Begin function NSNumber_numberWithUnsignedLongLong
	.p2align	2
_NSNumber_numberWithUnsignedLongLong:   ; @NSNumber_numberWithUnsignedLongLong
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh37:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh38:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	b	"_objc_msgSend$numberWithUnsignedLongLong:"
	.loh AdrpLdr	Lloh37, Lloh38
	.cfi_endproc
                                        ; -- End function
	.globl	_NSNumber_numberWithFloat       ; -- Begin function NSNumber_numberWithFloat
	.p2align	2
_NSNumber_numberWithFloat:              ; @NSNumber_numberWithFloat
	.cfi_startproc
; %bb.0:
Lloh39:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh40:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	b	"_objc_msgSend$numberWithFloat:"
	.loh AdrpLdr	Lloh39, Lloh40
	.cfi_endproc
                                        ; -- End function
	.globl	_NSNumber_numberWithDouble      ; -- Begin function NSNumber_numberWithDouble
	.p2align	2
_NSNumber_numberWithDouble:             ; @NSNumber_numberWithDouble
	.cfi_startproc
; %bb.0:
Lloh41:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh42:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	b	"_objc_msgSend$numberWithDouble:"
	.loh AdrpLdr	Lloh41, Lloh42
	.cfi_endproc
                                        ; -- End function
	.globl	_NSNumber_numberWithBool        ; -- Begin function NSNumber_numberWithBool
	.p2align	2
_NSNumber_numberWithBool:               ; @NSNumber_numberWithBool
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh43:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh44:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	b	"_objc_msgSend$numberWithBool:"
	.loh AdrpLdr	Lloh43, Lloh44
	.cfi_endproc
                                        ; -- End function
	.globl	_NSNumber_numberWithInteger     ; -- Begin function NSNumber_numberWithInteger
	.p2align	2
_NSNumber_numberWithInteger:            ; @NSNumber_numberWithInteger
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh45:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh46:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	b	"_objc_msgSend$numberWithInteger:"
	.loh AdrpLdr	Lloh45, Lloh46
	.cfi_endproc
                                        ; -- End function
	.globl	_NSNumber_numberWithUnsignedInteger ; -- Begin function NSNumber_numberWithUnsignedInteger
	.p2align	2
_NSNumber_numberWithUnsignedInteger:    ; @NSNumber_numberWithUnsignedInteger
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh47:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh48:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	b	"_objc_msgSend$numberWithUnsignedInteger:"
	.loh AdrpLdr	Lloh47, Lloh48
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_isEqualToNumber           ; -- Begin function rsel_isEqualToNumber
	.p2align	2
_rsel_isEqualToNumber:                  ; @rsel_isEqualToNumber
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$isEqualToNumber:"
	.cfi_endproc
                                        ; -- End function
	.globl	_NSArray_withObjs               ; -- Begin function NSArray_withObjs
	.p2align	2
_NSArray_withObjs:                      ; @NSArray_withObjs
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh49:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.12@PAGE
Lloh50:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.12@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$arrayWithObjects:count:"
	.loh AdrpLdr	Lloh49, Lloh50
	.cfi_endproc
                                        ; -- End function
	.globl	_NSArray_array                  ; -- Begin function NSArray_array
	.p2align	2
_NSArray_array:                         ; @NSArray_array
	.cfi_startproc
; %bb.0:
Lloh51:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.12@PAGE
Lloh52:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.12@PAGEOFF]
	b	_objc_msgSend$array
	.loh AdrpLdr	Lloh51, Lloh52
	.cfi_endproc
                                        ; -- End function
	.globl	_NSSet_withObjs                 ; -- Begin function NSSet_withObjs
	.p2align	2
_NSSet_withObjs:                        ; @NSSet_withObjs
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh53:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.13@PAGE
Lloh54:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.13@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$setWithObjects:count:"
	.loh AdrpLdr	Lloh53, Lloh54
	.cfi_endproc
                                        ; -- End function
	.globl	_foo                            ; -- Begin function foo
	.p2align	2
_foo:                                   ; @foo
	.cfi_startproc
; %bb.0:
Lloh55:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh56:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	mov	x2, #9223372036854775806
	b	"_objc_msgSend$numberWithInteger:"
	.loh AdrpLdr	Lloh55, Lloh56
	.cfi_endproc
                                        ; -- End function
	.globl	_foo1                           ; -- Begin function foo1
	.p2align	2
_foo1:                                  ; @foo1
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh57:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh58:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	bl	_objc_alloc
	mov	x2, #9223372036854775806
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	b	"_objc_msgSend$initWithInteger:"
	.loh AdrpLdr	Lloh57, Lloh58
	.cfi_endproc
                                        ; -- End function
	.globl	_NSSet_set                      ; -- Begin function NSSet_set
	.p2align	2
_NSSet_set:                             ; @NSSet_set
	.cfi_startproc
; %bb.0:
Lloh59:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.13@PAGE
Lloh60:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.13@PAGEOFF]
	b	_objc_msgSend$set
	.loh AdrpLdr	Lloh59, Lloh60
	.cfi_endproc
                                        ; -- End function
	.globl	_NSRegularExpression_regularExpressionWithPattern_options_error ; -- Begin function NSRegularExpression_regularExpressionWithPattern_options_error
	.p2align	2
_NSRegularExpression_regularExpressionWithPattern_options_error: ; @NSRegularExpression_regularExpressionWithPattern_options_error
	.cfi_startproc
; %bb.0:
	mov	x4, x2
	mov	x2, x0
Lloh61:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.14@PAGE
Lloh62:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.14@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$regularExpressionWithPattern:options:error:"
	.loh AdrpLdr	Lloh61, Lloh62
	.cfi_endproc
                                        ; -- End function
	.globl	_NSString_initWithBytes_length_encoding ; -- Begin function NSString_initWithBytes_length_encoding
	.p2align	2
_NSString_initWithBytes_length_encoding: ; @NSString_initWithBytes_length_encoding
	.cfi_startproc
; %bb.0:
	stp	x22, x21, [sp, #-48]!           ; 16-byte Folded Spill
	stp	x20, x19, [sp, #16]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	mov	x19, x2
	mov	x20, x1
	mov	x21, x0
Lloh63:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.15@PAGE
Lloh64:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.15@PAGEOFF]
	bl	_objc_alloc
	mov	x2, x21
	mov	x3, x20
	mov	x4, x19
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #16]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp], #48             ; 16-byte Folded Reload
	b	"_objc_msgSend$initWithBytes:length:encoding:"
	.loh AdrpLdr	Lloh63, Lloh64
	.cfi_endproc
                                        ; -- End function
	.globl	_NSString_initWithBytesNoCopy_length_encoding_freeWhenDone ; -- Begin function NSString_initWithBytesNoCopy_length_encoding_freeWhenDone
	.p2align	2
_NSString_initWithBytesNoCopy_length_encoding_freeWhenDone: ; @NSString_initWithBytesNoCopy_length_encoding_freeWhenDone
	.cfi_startproc
; %bb.0:
	stp	x22, x21, [sp, #-48]!           ; 16-byte Folded Spill
	stp	x20, x19, [sp, #16]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	mov	x19, x3
	mov	x20, x2
	mov	x21, x1
	mov	x22, x0
Lloh65:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.15@PAGE
Lloh66:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.15@PAGEOFF]
	bl	_objc_alloc
	mov	x2, x22
	mov	x3, x21
	mov	x4, x20
	mov	x5, x19
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #16]             ; 16-byte Folded Reload
	ldp	x22, x21, [sp], #48             ; 16-byte Folded Reload
	b	"_objc_msgSend$initWithBytesNoCopy:length:encoding:freeWhenDone:"
	.loh AdrpLdr	Lloh65, Lloh66
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURL_fileURLWithPath_isDirectory_relativeToURL ; -- Begin function NSURL_fileURLWithPath_isDirectory_relativeToURL
	.p2align	2
_NSURL_fileURLWithPath_isDirectory_relativeToURL: ; @NSURL_fileURLWithPath_isDirectory_relativeToURL
	.cfi_startproc
; %bb.0:
	mov	x4, x2
	mov	x2, x0
Lloh67:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.16@PAGE
Lloh68:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.16@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$fileURLWithPath:isDirectory:relativeToURL:"
	.loh AdrpLdr	Lloh67, Lloh68
	.cfi_endproc
                                        ; -- End function
	.globl	_NSURL_URLWithString_relativeToURL ; -- Begin function NSURL_URLWithString_relativeToURL
	.p2align	2
_NSURL_URLWithString_relativeToURL:     ; @NSURL_URLWithString_relativeToURL
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh69:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.16@PAGE
Lloh70:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.16@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$URLWithString:relativeToURL:"
	.loh AdrpLdr	Lloh69, Lloh70
	.cfi_endproc
                                        ; -- End function
	.globl	_NSDictionary_dictionary        ; -- Begin function NSDictionary_dictionary
	.p2align	2
_NSDictionary_dictionary:               ; @NSDictionary_dictionary
	.cfi_startproc
; %bb.0:
Lloh71:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.17@PAGE
Lloh72:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.17@PAGEOFF]
	b	_objc_msgSend$dictionary
	.loh AdrpLdr	Lloh71, Lloh72
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function common_initializer
_common_initializer:                    ; @common_initializer
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh73:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_@PAGE
Lloh74:
	ldr	x8, [x8, _OBJC_SELECTOR_REFERENCES_@PAGEOFF]
Lloh75:
	adrp	x9, _ns_isEqual@GOTPAGE
Lloh76:
	ldr	x9, [x9, _ns_isEqual@GOTPAGEOFF]
Lloh77:
	str	x8, [x9]
Lloh78:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.19@PAGE
Lloh79:
	ldr	x8, [x8, _OBJC_SELECTOR_REFERENCES_.19@PAGEOFF]
Lloh80:
	adrp	x9, _ns_resultType@GOTPAGE
Lloh81:
	ldr	x9, [x9, _ns_resultType@GOTPAGEOFF]
Lloh82:
	str	x8, [x9]
Lloh83:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.21@PAGE
Lloh84:
	ldr	x8, [x8, _OBJC_SELECTOR_REFERENCES_.21@PAGEOFF]
Lloh85:
	adrp	x9, _ns_range@GOTPAGE
Lloh86:
	ldr	x9, [x9, _ns_range@GOTPAGEOFF]
Lloh87:
	str	x8, [x9]
Lloh88:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.23@PAGE
Lloh89:
	ldr	x8, [x8, _OBJC_SELECTOR_REFERENCES_.23@PAGEOFF]
Lloh90:
	adrp	x9, _ns_lengthOfBytesUsingEncoding@GOTPAGE
Lloh91:
	ldr	x9, [x9, _ns_lengthOfBytesUsingEncoding@GOTPAGEOFF]
Lloh92:
	str	x8, [x9]
Lloh93:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh94:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	bl	_objc_opt_class
Lloh95:
	adrp	x8, _NS_NUMBER@GOTPAGE
Lloh96:
	ldr	x8, [x8, _NS_NUMBER@GOTPAGEOFF]
Lloh97:
	str	x0, [x8]
Lloh98:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.15@PAGE
Lloh99:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.15@PAGEOFF]
	bl	_objc_opt_class
Lloh100:
	adrp	x8, _NS_STRING@GOTPAGE
Lloh101:
	ldr	x8, [x8, _NS_STRING@GOTPAGEOFF]
Lloh102:
	str	x0, [x8]
Lloh103:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.24@PAGE
Lloh104:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.24@PAGEOFF]
	bl	_objc_opt_class
Lloh105:
	adrp	x8, _NS_MUTABLE_STRING@GOTPAGE
Lloh106:
	ldr	x8, [x8, _NS_MUTABLE_STRING@GOTPAGEOFF]
Lloh107:
	str	x0, [x8]
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.loh AdrpLdrGotStr	Lloh105, Lloh106, Lloh107
	.loh AdrpLdr	Lloh103, Lloh104
	.loh AdrpLdrGotStr	Lloh100, Lloh101, Lloh102
	.loh AdrpLdr	Lloh98, Lloh99
	.loh AdrpLdrGotStr	Lloh95, Lloh96, Lloh97
	.loh AdrpLdr	Lloh93, Lloh94
	.loh AdrpLdrGotStr	Lloh90, Lloh91, Lloh92
	.loh AdrpLdr	Lloh88, Lloh89
	.loh AdrpLdrGotStr	Lloh85, Lloh86, Lloh87
	.loh AdrpLdr	Lloh83, Lloh84
	.loh AdrpLdrGotStr	Lloh80, Lloh81, Lloh82
	.loh AdrpLdr	Lloh78, Lloh79
	.loh AdrpLdrGotStr	Lloh75, Lloh76, Lloh77
	.loh AdrpLdr	Lloh73, Lloh74
	.cfi_endproc
                                        ; -- End function
	.p2align	2                               ; -- Begin function -[CidreMachPortDelegate handleMachMessage:]
"-[CidreMachPortDelegate handleMachMessage:]": ; @"\01-[CidreMachPortDelegate handleMachMessage:]"
	.cfi_startproc
; %bb.0:
	mov	x1, x2
	ldp	x0, x2, [x0, #8]
	br	x2
	.cfi_endproc
                                        ; -- End function
	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_"
_OBJC_CLASSLIST_REFERENCES_$_:
	.quad	_OBJC_CLASS_$_NSPort

	.section	__DATA,__objc_data
	.globl	_OBJC_CLASS_$_CidreMachPortDelegate ; @"OBJC_CLASS_$_CidreMachPortDelegate"
	.p2align	3
_OBJC_CLASS_$_CidreMachPortDelegate:
	.quad	_OBJC_METACLASS_$_CidreMachPortDelegate
	.quad	_OBJC_CLASS_$_NSObject
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_CLASS_RO_$_CidreMachPortDelegate

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.1"
_OBJC_CLASSLIST_REFERENCES_$_.1:
	.quad	_OBJC_CLASS_$_CidreMachPortDelegate

	.section	__DATA,__objc_ivar
	.globl	_OBJC_IVAR_$_CidreMachPortDelegate._vtable ; @"OBJC_IVAR_$_CidreMachPortDelegate._vtable"
	.p2align	2
_OBJC_IVAR_$_CidreMachPortDelegate._vtable:
	.long	8                               ; 0x8

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.2"
_OBJC_CLASSLIST_REFERENCES_$_.2:
	.quad	_OBJC_CLASS_$_NSProcessInfo

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.3"
_OBJC_CLASSLIST_REFERENCES_$_.3:
	.quad	_OBJC_CLASS_$_NSException

	.section	__TEXT,__cstring,cstring_literals
l_.str:                                 ; @.str
	.asciz	"%@"

	.section	__DATA,__cfstring
	.p2align	3                               ; @_unnamed_cfstring_
l__unnamed_cfstring_:
	.quad	___CFConstantStringClassReference
	.long	1992                            ; 0x7c8
	.space	4
	.quad	l_.str
	.quad	2                               ; 0x2

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.4"
_OBJC_CLASSLIST_REFERENCES_$_.4:
	.quad	_OBJC_CLASS_$_NSURLSession

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.5"
_OBJC_CLASSLIST_REFERENCES_$_.5:
	.quad	_OBJC_CLASS_$_NSURLRequest

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.6"
_OBJC_CLASSLIST_REFERENCES_$_.6:
	.quad	_OBJC_CLASS_$_NSMutableURLRequest

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.7"
_OBJC_CLASSLIST_REFERENCES_$_.7:
	.quad	_OBJC_CLASS_$_NSURLResponse

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.8"
_OBJC_CLASSLIST_REFERENCES_$_.8:
	.quad	_OBJC_CLASS_$_NSURLSessionWebSocketMessage

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.9"
_OBJC_CLASSLIST_REFERENCES_$_.9:
	.quad	_OBJC_CLASS_$_NSURLCache

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.10"
_OBJC_CLASSLIST_REFERENCES_$_.10:
	.quad	_OBJC_CLASS_$_NSData

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.11"
_OBJC_CLASSLIST_REFERENCES_$_.11:
	.quad	_OBJC_CLASS_$_NSNumber

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.12"
_OBJC_CLASSLIST_REFERENCES_$_.12:
	.quad	_OBJC_CLASS_$_NSArray

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.13"
_OBJC_CLASSLIST_REFERENCES_$_.13:
	.quad	_OBJC_CLASS_$_NSSet

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.14"
_OBJC_CLASSLIST_REFERENCES_$_.14:
	.quad	_OBJC_CLASS_$_NSRegularExpression

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.15"
_OBJC_CLASSLIST_REFERENCES_$_.15:
	.quad	_OBJC_CLASS_$_NSString

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.16"
_OBJC_CLASSLIST_REFERENCES_$_.16:
	.quad	_OBJC_CLASS_$_NSURL

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.17"
_OBJC_CLASSLIST_REFERENCES_$_.17:
	.quad	_OBJC_CLASS_$_NSDictionary

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_:                  ; @OBJC_METH_VAR_NAME_
	.asciz	"isEqual:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_
_OBJC_SELECTOR_REFERENCES_:
	.quad	l_OBJC_METH_VAR_NAME_

	.comm	_ns_isEqual,8,3                 ; @ns_isEqual
	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.18:               ; @OBJC_METH_VAR_NAME_.18
	.asciz	"resultType"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.19
_OBJC_SELECTOR_REFERENCES_.19:
	.quad	l_OBJC_METH_VAR_NAME_.18

	.comm	_ns_resultType,8,3              ; @ns_resultType
	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.20:               ; @OBJC_METH_VAR_NAME_.20
	.asciz	"range"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.21
_OBJC_SELECTOR_REFERENCES_.21:
	.quad	l_OBJC_METH_VAR_NAME_.20

	.comm	_ns_range,8,3                   ; @ns_range
	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.22:               ; @OBJC_METH_VAR_NAME_.22
	.asciz	"lengthOfBytesUsingEncoding:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.23
_OBJC_SELECTOR_REFERENCES_.23:
	.quad	l_OBJC_METH_VAR_NAME_.22

	.comm	_ns_lengthOfBytesUsingEncoding,8,3 ; @ns_lengthOfBytesUsingEncoding
	.comm	_NS_NUMBER,8,3                  ; @NS_NUMBER
	.comm	_NS_STRING,8,3                  ; @NS_STRING
	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.24"
_OBJC_CLASSLIST_REFERENCES_$_.24:
	.quad	_OBJC_CLASS_$_NSMutableString

	.comm	_NS_MUTABLE_STRING,8,3          ; @NS_MUTABLE_STRING
	.section	__TEXT,__objc_classname,cstring_literals
l_OBJC_CLASS_NAME_:                     ; @OBJC_CLASS_NAME_
	.asciz	"CidreMachPortDelegate"

l_OBJC_CLASS_NAME_.25:                  ; @OBJC_CLASS_NAME_.25
	.asciz	"NSMachPortDelegate"

l_OBJC_CLASS_NAME_.26:                  ; @OBJC_CLASS_NAME_.26
	.asciz	"NSPortDelegate"

l_OBJC_CLASS_NAME_.27:                  ; @OBJC_CLASS_NAME_.27
	.asciz	"NSObject"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_:                  ; @OBJC_METH_VAR_TYPE_
	.asciz	"B24@0:8@16"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.28:               ; @OBJC_METH_VAR_NAME_.28
	.asciz	"class"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.29:               ; @OBJC_METH_VAR_TYPE_.29
	.asciz	"#16@0:8"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.30:               ; @OBJC_METH_VAR_NAME_.30
	.asciz	"self"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.31:               ; @OBJC_METH_VAR_TYPE_.31
	.asciz	"@16@0:8"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.32:               ; @OBJC_METH_VAR_NAME_.32
	.asciz	"performSelector:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.33:               ; @OBJC_METH_VAR_TYPE_.33
	.asciz	"@24@0:8:16"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.34:               ; @OBJC_METH_VAR_NAME_.34
	.asciz	"performSelector:withObject:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.35:               ; @OBJC_METH_VAR_TYPE_.35
	.asciz	"@32@0:8:16@24"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.36:               ; @OBJC_METH_VAR_NAME_.36
	.asciz	"performSelector:withObject:withObject:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.37:               ; @OBJC_METH_VAR_TYPE_.37
	.asciz	"@40@0:8:16@24@32"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.38:               ; @OBJC_METH_VAR_NAME_.38
	.asciz	"isProxy"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.39:               ; @OBJC_METH_VAR_TYPE_.39
	.asciz	"B16@0:8"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.40:               ; @OBJC_METH_VAR_NAME_.40
	.asciz	"isKindOfClass:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.41:               ; @OBJC_METH_VAR_TYPE_.41
	.asciz	"B24@0:8#16"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.42:               ; @OBJC_METH_VAR_NAME_.42
	.asciz	"isMemberOfClass:"

l_OBJC_METH_VAR_NAME_.43:               ; @OBJC_METH_VAR_NAME_.43
	.asciz	"conformsToProtocol:"

l_OBJC_METH_VAR_NAME_.44:               ; @OBJC_METH_VAR_NAME_.44
	.asciz	"respondsToSelector:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.45:               ; @OBJC_METH_VAR_TYPE_.45
	.asciz	"B24@0:8:16"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.46:               ; @OBJC_METH_VAR_NAME_.46
	.asciz	"retain"

l_OBJC_METH_VAR_NAME_.47:               ; @OBJC_METH_VAR_NAME_.47
	.asciz	"release"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.48:               ; @OBJC_METH_VAR_TYPE_.48
	.asciz	"Vv16@0:8"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.49:               ; @OBJC_METH_VAR_NAME_.49
	.asciz	"autorelease"

l_OBJC_METH_VAR_NAME_.50:               ; @OBJC_METH_VAR_NAME_.50
	.asciz	"retainCount"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.51:               ; @OBJC_METH_VAR_TYPE_.51
	.asciz	"Q16@0:8"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.52:               ; @OBJC_METH_VAR_NAME_.52
	.asciz	"zone"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.53:               ; @OBJC_METH_VAR_TYPE_.53
	.asciz	"^{_NSZone=}16@0:8"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.54:               ; @OBJC_METH_VAR_NAME_.54
	.asciz	"hash"

l_OBJC_METH_VAR_NAME_.55:               ; @OBJC_METH_VAR_NAME_.55
	.asciz	"superclass"

l_OBJC_METH_VAR_NAME_.56:               ; @OBJC_METH_VAR_NAME_.56
	.asciz	"description"

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROTOCOL_INSTANCE_METHODS_NSObject"
__OBJC_$_PROTOCOL_INSTANCE_METHODS_NSObject:
	.long	24                              ; 0x18
	.long	19                              ; 0x13
	.quad	l_OBJC_METH_VAR_NAME_
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.28
	.quad	l_OBJC_METH_VAR_TYPE_.29
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.30
	.quad	l_OBJC_METH_VAR_TYPE_.31
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.32
	.quad	l_OBJC_METH_VAR_TYPE_.33
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.34
	.quad	l_OBJC_METH_VAR_TYPE_.35
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.36
	.quad	l_OBJC_METH_VAR_TYPE_.37
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.38
	.quad	l_OBJC_METH_VAR_TYPE_.39
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.40
	.quad	l_OBJC_METH_VAR_TYPE_.41
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.42
	.quad	l_OBJC_METH_VAR_TYPE_.41
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.43
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.44
	.quad	l_OBJC_METH_VAR_TYPE_.45
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.46
	.quad	l_OBJC_METH_VAR_TYPE_.31
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.47
	.quad	l_OBJC_METH_VAR_TYPE_.48
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.49
	.quad	l_OBJC_METH_VAR_TYPE_.31
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.50
	.quad	l_OBJC_METH_VAR_TYPE_.51
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.52
	.quad	l_OBJC_METH_VAR_TYPE_.53
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.54
	.quad	l_OBJC_METH_VAR_TYPE_.51
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.55
	.quad	l_OBJC_METH_VAR_TYPE_.29
	.quad	0
	.quad	l_OBJC_METH_VAR_NAME_.56
	.quad	l_OBJC_METH_VAR_TYPE_.31
	.quad	0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.57:               ; @OBJC_METH_VAR_NAME_.57
	.asciz	"debugDescription"

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSObject"
__OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSObject:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.57
	.quad	l_OBJC_METH_VAR_TYPE_.31
	.quad	0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_PROP_NAME_ATTR_:                 ; @OBJC_PROP_NAME_ATTR_
	.asciz	"hash"

l_OBJC_PROP_NAME_ATTR_.58:              ; @OBJC_PROP_NAME_ATTR_.58
	.asciz	"TQ,R"

l_OBJC_PROP_NAME_ATTR_.59:              ; @OBJC_PROP_NAME_ATTR_.59
	.asciz	"superclass"

l_OBJC_PROP_NAME_ATTR_.60:              ; @OBJC_PROP_NAME_ATTR_.60
	.asciz	"T#,R"

l_OBJC_PROP_NAME_ATTR_.61:              ; @OBJC_PROP_NAME_ATTR_.61
	.asciz	"description"

l_OBJC_PROP_NAME_ATTR_.62:              ; @OBJC_PROP_NAME_ATTR_.62
	.asciz	"T@\"NSString\",R,C"

l_OBJC_PROP_NAME_ATTR_.63:              ; @OBJC_PROP_NAME_ATTR_.63
	.asciz	"debugDescription"

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROP_LIST_NSObject"
__OBJC_$_PROP_LIST_NSObject:
	.long	16                              ; 0x10
	.long	4                               ; 0x4
	.quad	l_OBJC_PROP_NAME_ATTR_
	.quad	l_OBJC_PROP_NAME_ATTR_.58
	.quad	l_OBJC_PROP_NAME_ATTR_.59
	.quad	l_OBJC_PROP_NAME_ATTR_.60
	.quad	l_OBJC_PROP_NAME_ATTR_.61
	.quad	l_OBJC_PROP_NAME_ATTR_.62
	.quad	l_OBJC_PROP_NAME_ATTR_.63
	.quad	l_OBJC_PROP_NAME_ATTR_.62

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.64:               ; @OBJC_METH_VAR_TYPE_.64
	.asciz	"B24@0:8@\"Protocol\"16"

l_OBJC_METH_VAR_TYPE_.65:               ; @OBJC_METH_VAR_TYPE_.65
	.asciz	"@\"NSString\"16@0:8"

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROTOCOL_METHOD_TYPES_NSObject"
__OBJC_$_PROTOCOL_METHOD_TYPES_NSObject:
	.quad	l_OBJC_METH_VAR_TYPE_
	.quad	l_OBJC_METH_VAR_TYPE_.29
	.quad	l_OBJC_METH_VAR_TYPE_.31
	.quad	l_OBJC_METH_VAR_TYPE_.33
	.quad	l_OBJC_METH_VAR_TYPE_.35
	.quad	l_OBJC_METH_VAR_TYPE_.37
	.quad	l_OBJC_METH_VAR_TYPE_.39
	.quad	l_OBJC_METH_VAR_TYPE_.41
	.quad	l_OBJC_METH_VAR_TYPE_.41
	.quad	l_OBJC_METH_VAR_TYPE_.64
	.quad	l_OBJC_METH_VAR_TYPE_.45
	.quad	l_OBJC_METH_VAR_TYPE_.31
	.quad	l_OBJC_METH_VAR_TYPE_.48
	.quad	l_OBJC_METH_VAR_TYPE_.31
	.quad	l_OBJC_METH_VAR_TYPE_.51
	.quad	l_OBJC_METH_VAR_TYPE_.53
	.quad	l_OBJC_METH_VAR_TYPE_.51
	.quad	l_OBJC_METH_VAR_TYPE_.29
	.quad	l_OBJC_METH_VAR_TYPE_.65
	.quad	l_OBJC_METH_VAR_TYPE_.65

	.private_extern	__OBJC_PROTOCOL_$_NSObject ; @"_OBJC_PROTOCOL_$_NSObject"
	.section	__DATA,__data
	.globl	__OBJC_PROTOCOL_$_NSObject
	.weak_definition	__OBJC_PROTOCOL_$_NSObject
	.p2align	3
__OBJC_PROTOCOL_$_NSObject:
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.27
	.quad	0
	.quad	__OBJC_$_PROTOCOL_INSTANCE_METHODS_NSObject
	.quad	0
	.quad	__OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSObject
	.quad	0
	.quad	__OBJC_$_PROP_LIST_NSObject
	.long	96                              ; 0x60
	.long	0                               ; 0x0
	.quad	__OBJC_$_PROTOCOL_METHOD_TYPES_NSObject
	.quad	0
	.quad	0

	.private_extern	__OBJC_LABEL_PROTOCOL_$_NSObject ; @"_OBJC_LABEL_PROTOCOL_$_NSObject"
	.section	__DATA,__objc_protolist,coalesced,no_dead_strip
	.globl	__OBJC_LABEL_PROTOCOL_$_NSObject
	.weak_definition	__OBJC_LABEL_PROTOCOL_$_NSObject
	.p2align	3
__OBJC_LABEL_PROTOCOL_$_NSObject:
	.quad	__OBJC_PROTOCOL_$_NSObject

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROTOCOL_REFS_NSPortDelegate"
__OBJC_$_PROTOCOL_REFS_NSPortDelegate:
	.quad	1                               ; 0x1
	.quad	__OBJC_PROTOCOL_$_NSObject
	.quad	0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.66:               ; @OBJC_METH_VAR_NAME_.66
	.asciz	"handlePortMessage:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.67:               ; @OBJC_METH_VAR_TYPE_.67
	.asciz	"v24@0:8@16"

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSPortDelegate"
__OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSPortDelegate:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.66
	.quad	l_OBJC_METH_VAR_TYPE_.67
	.quad	0

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.68:               ; @OBJC_METH_VAR_TYPE_.68
	.asciz	"v24@0:8@\"NSPortMessage\"16"

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROTOCOL_METHOD_TYPES_NSPortDelegate"
__OBJC_$_PROTOCOL_METHOD_TYPES_NSPortDelegate:
	.quad	l_OBJC_METH_VAR_TYPE_.68

	.private_extern	__OBJC_PROTOCOL_$_NSPortDelegate ; @"_OBJC_PROTOCOL_$_NSPortDelegate"
	.section	__DATA,__data
	.globl	__OBJC_PROTOCOL_$_NSPortDelegate
	.weak_definition	__OBJC_PROTOCOL_$_NSPortDelegate
	.p2align	3
__OBJC_PROTOCOL_$_NSPortDelegate:
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.26
	.quad	__OBJC_$_PROTOCOL_REFS_NSPortDelegate
	.quad	0
	.quad	0
	.quad	__OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSPortDelegate
	.quad	0
	.quad	0
	.long	96                              ; 0x60
	.long	0                               ; 0x0
	.quad	__OBJC_$_PROTOCOL_METHOD_TYPES_NSPortDelegate
	.quad	0
	.quad	0

	.private_extern	__OBJC_LABEL_PROTOCOL_$_NSPortDelegate ; @"_OBJC_LABEL_PROTOCOL_$_NSPortDelegate"
	.section	__DATA,__objc_protolist,coalesced,no_dead_strip
	.globl	__OBJC_LABEL_PROTOCOL_$_NSPortDelegate
	.weak_definition	__OBJC_LABEL_PROTOCOL_$_NSPortDelegate
	.p2align	3
__OBJC_LABEL_PROTOCOL_$_NSPortDelegate:
	.quad	__OBJC_PROTOCOL_$_NSPortDelegate

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROTOCOL_REFS_NSMachPortDelegate"
__OBJC_$_PROTOCOL_REFS_NSMachPortDelegate:
	.quad	1                               ; 0x1
	.quad	__OBJC_PROTOCOL_$_NSPortDelegate
	.quad	0

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.69:               ; @OBJC_METH_VAR_NAME_.69
	.asciz	"handleMachMessage:"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.70:               ; @OBJC_METH_VAR_TYPE_.70
	.asciz	"v24@0:8^v16"

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSMachPortDelegate"
__OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSMachPortDelegate:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.69
	.quad	l_OBJC_METH_VAR_TYPE_.70
	.quad	0

	.p2align	3                               ; @"_OBJC_$_PROTOCOL_METHOD_TYPES_NSMachPortDelegate"
__OBJC_$_PROTOCOL_METHOD_TYPES_NSMachPortDelegate:
	.quad	l_OBJC_METH_VAR_TYPE_.70

	.private_extern	__OBJC_PROTOCOL_$_NSMachPortDelegate ; @"_OBJC_PROTOCOL_$_NSMachPortDelegate"
	.section	__DATA,__data
	.globl	__OBJC_PROTOCOL_$_NSMachPortDelegate
	.weak_definition	__OBJC_PROTOCOL_$_NSMachPortDelegate
	.p2align	3
__OBJC_PROTOCOL_$_NSMachPortDelegate:
	.quad	0
	.quad	l_OBJC_CLASS_NAME_.25
	.quad	__OBJC_$_PROTOCOL_REFS_NSMachPortDelegate
	.quad	0
	.quad	0
	.quad	__OBJC_$_PROTOCOL_INSTANCE_METHODS_OPT_NSMachPortDelegate
	.quad	0
	.quad	0
	.long	96                              ; 0x60
	.long	0                               ; 0x0
	.quad	__OBJC_$_PROTOCOL_METHOD_TYPES_NSMachPortDelegate
	.quad	0
	.quad	0

	.private_extern	__OBJC_LABEL_PROTOCOL_$_NSMachPortDelegate ; @"_OBJC_LABEL_PROTOCOL_$_NSMachPortDelegate"
	.section	__DATA,__objc_protolist,coalesced,no_dead_strip
	.globl	__OBJC_LABEL_PROTOCOL_$_NSMachPortDelegate
	.weak_definition	__OBJC_LABEL_PROTOCOL_$_NSMachPortDelegate
	.p2align	3
__OBJC_LABEL_PROTOCOL_$_NSMachPortDelegate:
	.quad	__OBJC_PROTOCOL_$_NSMachPortDelegate

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_CLASS_PROTOCOLS_$_CidreMachPortDelegate"
__OBJC_CLASS_PROTOCOLS_$_CidreMachPortDelegate:
	.quad	1                               ; 0x1
	.quad	__OBJC_PROTOCOL_$_NSMachPortDelegate
	.quad	0

	.p2align	3                               ; @"_OBJC_METACLASS_RO_$_CidreMachPortDelegate"
__OBJC_METACLASS_RO_$_CidreMachPortDelegate:
	.long	1                               ; 0x1
	.long	40                              ; 0x28
	.long	40                              ; 0x28
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_
	.quad	0
	.quad	__OBJC_CLASS_PROTOCOLS_$_CidreMachPortDelegate
	.quad	0
	.quad	0
	.quad	0

	.section	__DATA,__objc_data
	.globl	_OBJC_METACLASS_$_CidreMachPortDelegate ; @"OBJC_METACLASS_$_CidreMachPortDelegate"
	.p2align	3
_OBJC_METACLASS_$_CidreMachPortDelegate:
	.quad	_OBJC_METACLASS_$_NSObject
	.quad	_OBJC_METACLASS_$_NSObject
	.quad	__objc_empty_cache
	.quad	0
	.quad	__OBJC_METACLASS_RO_$_CidreMachPortDelegate

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_INSTANCE_METHODS_CidreMachPortDelegate"
__OBJC_$_INSTANCE_METHODS_CidreMachPortDelegate:
	.long	24                              ; 0x18
	.long	1                               ; 0x1
	.quad	l_OBJC_METH_VAR_NAME_.69
	.quad	l_OBJC_METH_VAR_TYPE_.70
	.quad	"-[CidreMachPortDelegate handleMachMessage:]"

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.71:               ; @OBJC_METH_VAR_NAME_.71
	.asciz	"_vtable"

	.section	__TEXT,__objc_methtype,cstring_literals
l_OBJC_METH_VAR_TYPE_.72:               ; @OBJC_METH_VAR_TYPE_.72
	.asciz	"[2^v]"

	.section	__DATA,__objc_const
	.p2align	3                               ; @"_OBJC_$_INSTANCE_VARIABLES_CidreMachPortDelegate"
__OBJC_$_INSTANCE_VARIABLES_CidreMachPortDelegate:
	.long	32                              ; 0x20
	.long	1                               ; 0x1
	.quad	_OBJC_IVAR_$_CidreMachPortDelegate._vtable
	.quad	l_OBJC_METH_VAR_NAME_.71
	.quad	l_OBJC_METH_VAR_TYPE_.72
	.long	3                               ; 0x3
	.long	16                              ; 0x10

	.p2align	3                               ; @"_OBJC_$_PROP_LIST_CidreMachPortDelegate"
__OBJC_$_PROP_LIST_CidreMachPortDelegate:
	.long	16                              ; 0x10
	.long	4                               ; 0x4
	.quad	l_OBJC_PROP_NAME_ATTR_
	.quad	l_OBJC_PROP_NAME_ATTR_.58
	.quad	l_OBJC_PROP_NAME_ATTR_.59
	.quad	l_OBJC_PROP_NAME_ATTR_.60
	.quad	l_OBJC_PROP_NAME_ATTR_.61
	.quad	l_OBJC_PROP_NAME_ATTR_.62
	.quad	l_OBJC_PROP_NAME_ATTR_.63
	.quad	l_OBJC_PROP_NAME_ATTR_.62

	.p2align	3                               ; @"_OBJC_CLASS_RO_$_CidreMachPortDelegate"
__OBJC_CLASS_RO_$_CidreMachPortDelegate:
	.long	0                               ; 0x0
	.long	8                               ; 0x8
	.long	24                              ; 0x18
	.space	4
	.quad	0
	.quad	l_OBJC_CLASS_NAME_
	.quad	__OBJC_$_INSTANCE_METHODS_CidreMachPortDelegate
	.quad	__OBJC_CLASS_PROTOCOLS_$_CidreMachPortDelegate
	.quad	__OBJC_$_INSTANCE_VARIABLES_CidreMachPortDelegate
	.quad	0
	.quad	__OBJC_$_PROP_LIST_CidreMachPortDelegate

	.section	__DATA,__objc_classlist,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_LABEL_CLASS_$"
l_OBJC_LABEL_CLASS_$:
	.quad	_OBJC_CLASS_$_CidreMachPortDelegate

	.section	__DATA,__mod_init_func,mod_init_funcs
	.p2align	3
	.quad	_common_initializer
	.no_dead_strip	__OBJC_LABEL_PROTOCOL_$_NSMachPortDelegate
	.no_dead_strip	__OBJC_LABEL_PROTOCOL_$_NSObject
	.no_dead_strip	__OBJC_LABEL_PROTOCOL_$_NSPortDelegate
	.no_dead_strip	__OBJC_PROTOCOL_$_NSMachPortDelegate
	.no_dead_strip	__OBJC_PROTOCOL_$_NSObject
	.no_dead_strip	__OBJC_PROTOCOL_$_NSPortDelegate
	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols
