	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 13, 0	sdk_version 13, 1
	.globl	_wsel_setActiveFormat           ; -- Begin function wsel_setActiveFormat
	.p2align	2
_wsel_setActiveFormat:                  ; @wsel_setActiveFormat
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setActiveFormat:"
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setActiveVideoMinFrameDuration ; -- Begin function wsel_setActiveVideoMinFrameDuration
	.p2align	2
_wsel_setActiveVideoMinFrameDuration:   ; @wsel_setActiveVideoMinFrameDuration
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	q0, [x1]
	str	q0, [sp]
	ldr	x8, [x1, #16]
	str	x8, [sp, #16]
	mov	x2, sp
	bl	"_objc_msgSend$setActiveVideoMinFrameDuration:"
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setActiveVideoMaxFrameDuration ; -- Begin function wsel_setActiveVideoMaxFrameDuration
	.p2align	2
_wsel_setActiveVideoMaxFrameDuration:   ; @wsel_setActiveVideoMaxFrameDuration
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	q0, [x1]
	str	q0, [sp]
	ldr	x8, [x1, #16]
	str	x8, [sp, #16]
	mov	x2, sp
	bl	"_objc_msgSend$setActiveVideoMaxFrameDuration:"
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_videoSupportedFrameRateRanges ; -- Begin function rsel_videoSupportedFrameRateRanges
	.p2align	2
_rsel_videoSupportedFrameRateRanges:    ; @rsel_videoSupportedFrameRateRanges
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$videoSupportedFrameRateRanges
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_formatDescription         ; -- Begin function rsel_formatDescription
	.p2align	2
_rsel_formatDescription:                ; @rsel_formatDescription
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$formatDescription
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_autoFocusSystem           ; -- Begin function rsel_autoFocusSystem
	.p2align	2
_rsel_autoFocusSystem:                  ; @rsel_autoFocusSystem
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$autoFocusSystem
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_startAndReturnError       ; -- Begin function rsel_startAndReturnError
	.p2align	2
_rsel_startAndReturnError:              ; @rsel_startAndReturnError
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$startAndReturnError:"
	.cfi_endproc
                                        ; -- End function
	.globl	_AVAssetWriter_assetWriterWithURL_fileType_error ; -- Begin function AVAssetWriter_assetWriterWithURL_fileType_error
	.p2align	2
_AVAssetWriter_assetWriterWithURL_fileType_error: ; @AVAssetWriter_assetWriterWithURL_fileType_error
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x4, x2
	mov	x2, x0
Lloh0:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_@PAGE
Lloh1:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_@PAGEOFF]
	mov	x3, x1
	bl	"_objc_msgSend$assetWriterWithURL:fileType:error:"
	mov	x29, x29
	bl	_objc_retainAutoreleasedReturnValue
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.loh AdrpLdr	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_finishWritingWithCompletionHandler ; -- Begin function wsel_finishWritingWithCompletionHandler
	.p2align	2
_wsel_finishWritingWithCompletionHandler: ; @wsel_finishWritingWithCompletionHandler
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$finishWritingWithCompletionHandler:"
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_loadTracksWithMediaType_completionHandler ; -- Begin function wsel_loadTracksWithMediaType_completionHandler
	.p2align	2
_wsel_loadTracksWithMediaType_completionHandler: ; @wsel_loadTracksWithMediaType_completionHandler
	.cfi_startproc
; %bb.0:
	mov	x3, x2
	mov	x2, x1
	b	"_objc_msgSend$loadTracksWithMediaType:completionHandler:"
	.cfi_endproc
                                        ; -- End function
	.globl	_AVURLAsset_URLAssetWithURL_options ; -- Begin function AVURLAsset_URLAssetWithURL_options
	.p2align	2
_AVURLAsset_URLAssetWithURL_options:    ; @AVURLAsset_URLAssetWithURL_options
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x0
Lloh2:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.1@PAGE
Lloh3:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.1@PAGEOFF]
	mov	x3, x1
	bl	"_objc_msgSend$URLAssetWithURL:options:"
	mov	x29, x29
	bl	_objc_retainAutoreleasedReturnValue
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.loh AdrpLdr	Lloh2, Lloh3
	.cfi_endproc
                                        ; -- End function
	.globl	_AVAssetReader_assetReaderWithAsset_error ; -- Begin function AVAssetReader_assetReaderWithAsset_error
	.p2align	2
_AVAssetReader_assetReaderWithAsset_error: ; @AVAssetReader_assetReaderWithAsset_error
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x0
Lloh4:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.2@PAGE
Lloh5:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.2@PAGEOFF]
	mov	x3, x1
	bl	"_objc_msgSend$assetReaderWithAsset:error:"
	mov	x29, x29
	bl	_objc_retainAutoreleasedReturnValue
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.loh AdrpLdr	Lloh4, Lloh5
	.cfi_endproc
                                        ; -- End function
	.globl	_AVAssetReaderTrackOutput_assetReaderTrackOutputWithTrack_outputSettings ; -- Begin function AVAssetReaderTrackOutput_assetReaderTrackOutputWithTrack_outputSettings
	.p2align	2
_AVAssetReaderTrackOutput_assetReaderTrackOutputWithTrack_outputSettings: ; @AVAssetReaderTrackOutput_assetReaderTrackOutputWithTrack_outputSettings
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x0
Lloh6:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.3@PAGE
Lloh7:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.3@PAGEOFF]
	mov	x3, x1
	bl	"_objc_msgSend$assetReaderTrackOutputWithTrack:outputSettings:"
	mov	x29, x29
	bl	_objc_retainAutoreleasedReturnValue
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.loh AdrpLdr	Lloh6, Lloh7
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_supportsRandomAccess      ; -- Begin function rsel_supportsRandomAccess
	.p2align	2
_rsel_supportsRandomAccess:             ; @rsel_supportsRandomAccess
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$supportsRandomAccess
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_resetForReadingTimeRanges ; -- Begin function wsel_resetForReadingTimeRanges
	.p2align	2
_wsel_resetForReadingTimeRanges:        ; @wsel_resetForReadingTimeRanges
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$resetForReadingTimeRanges:"
	.cfi_endproc
                                        ; -- End function
	.globl	_rsel_alwaysCopiesSampleData    ; -- Begin function rsel_alwaysCopiesSampleData
	.p2align	2
_rsel_alwaysCopiesSampleData:           ; @rsel_alwaysCopiesSampleData
	.cfi_startproc
; %bb.0:
	b	_objc_msgSend$alwaysCopiesSampleData
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_setAlwaysCopiesSampleData ; -- Begin function wsel_setAlwaysCopiesSampleData
	.p2align	2
_wsel_setAlwaysCopiesSampleData:        ; @wsel_setAlwaysCopiesSampleData
	.cfi_startproc
; %bb.0:
	mov	x2, x1
	b	"_objc_msgSend$setAlwaysCopiesSampleData:"
	.cfi_endproc
                                        ; -- End function
	.globl	_AVCaptureDeviceInput_deviceInputWithDevice_error ; -- Begin function AVCaptureDeviceInput_deviceInputWithDevice_error
	.p2align	2
_AVCaptureDeviceInput_deviceInputWithDevice_error: ; @AVCaptureDeviceInput_deviceInputWithDevice_error
	.cfi_startproc
; %bb.0:
	mov	x2, x0
Lloh8:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.4@PAGE
Lloh9:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.4@PAGEOFF]
	mov	x3, x1
	b	"_objc_msgSend$deviceInputWithDevice:error:"
	.loh AdrpLdr	Lloh8, Lloh9
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
Lloh10:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.5@PAGE
Lloh11:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.5@PAGEOFF]
	bl	_objc_opt_class
Lloh12:
	adrp	x8, _AV_CAPTURE_DEVICE@GOTPAGE
Lloh13:
	ldr	x8, [x8, _AV_CAPTURE_DEVICE@GOTPAGEOFF]
Lloh14:
	str	x0, [x8]
Lloh15:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.6@PAGE
Lloh16:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.6@PAGEOFF]
	bl	_objc_opt_class
Lloh17:
	adrp	x8, _AV_CAPTURE_METADATA_OUTPUT@GOTPAGE
Lloh18:
	ldr	x8, [x8, _AV_CAPTURE_METADATA_OUTPUT@GOTPAGEOFF]
Lloh19:
	str	x0, [x8]
Lloh20:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.7@PAGE
Lloh21:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.7@PAGEOFF]
	bl	_objc_opt_class
Lloh22:
	adrp	x8, _AV_CAPTURE_SESSION@GOTPAGE
Lloh23:
	ldr	x8, [x8, _AV_CAPTURE_SESSION@GOTPAGEOFF]
Lloh24:
	str	x0, [x8]
Lloh25:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.8@PAGE
Lloh26:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.8@PAGEOFF]
	bl	_objc_opt_class
Lloh27:
	adrp	x8, _AV_CAPTURE_DEVICE_DISCOVERY_SESSION@GOTPAGE
Lloh28:
	ldr	x8, [x8, _AV_CAPTURE_DEVICE_DISCOVERY_SESSION@GOTPAGEOFF]
Lloh29:
	str	x0, [x8]
Lloh30:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.9@PAGE
Lloh31:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.9@PAGEOFF]
	bl	_objc_opt_class
Lloh32:
	adrp	x8, _AV_CAPTURE_VIDEO_DATA_OUTPUT@GOTPAGE
Lloh33:
	ldr	x8, [x8, _AV_CAPTURE_VIDEO_DATA_OUTPUT@GOTPAGEOFF]
Lloh34:
	str	x0, [x8]
Lloh35:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.4@PAGE
Lloh36:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.4@PAGEOFF]
	bl	_objc_opt_class
Lloh37:
	adrp	x8, _AV_CAPTURE_DEVICE_INPUT@GOTPAGE
Lloh38:
	ldr	x8, [x8, _AV_CAPTURE_DEVICE_INPUT@GOTPAGEOFF]
Lloh39:
	str	x0, [x8]
Lloh40:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.10@PAGE
Lloh41:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.10@PAGEOFF]
	bl	_objc_opt_class
Lloh42:
	adrp	x8, _AV_AUDIO_ENGINE@GOTPAGE
Lloh43:
	ldr	x8, [x8, _AV_AUDIO_ENGINE@GOTPAGEOFF]
Lloh44:
	str	x0, [x8]
Lloh45:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGE
Lloh46:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.11@PAGEOFF]
	bl	_objc_opt_class
Lloh47:
	adrp	x8, _AV_AUDIO_TIME@GOTPAGE
Lloh48:
	ldr	x8, [x8, _AV_AUDIO_TIME@GOTPAGEOFF]
Lloh49:
	str	x0, [x8]
Lloh50:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.12@PAGE
Lloh51:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.12@PAGEOFF]
	bl	_objc_opt_class
Lloh52:
	adrp	x8, _AV_AUDIO_UNIT_EFFECT@GOTPAGE
Lloh53:
	ldr	x8, [x8, _AV_AUDIO_UNIT_EFFECT@GOTPAGEOFF]
Lloh54:
	str	x0, [x8]
Lloh55:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.13@PAGE
Lloh56:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.13@PAGEOFF]
	bl	_objc_opt_class
Lloh57:
	adrp	x8, _AV_AUDIO_UNIT_EQ@GOTPAGE
Lloh58:
	ldr	x8, [x8, _AV_AUDIO_UNIT_EQ@GOTPAGEOFF]
Lloh59:
	str	x0, [x8]
Lloh60:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.14@PAGE
Lloh61:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.14@PAGEOFF]
	bl	_objc_opt_class
Lloh62:
	adrp	x8, _AV_AUDIO_UNIT_TIME_EFFECT@GOTPAGE
Lloh63:
	ldr	x8, [x8, _AV_AUDIO_UNIT_TIME_EFFECT@GOTPAGEOFF]
Lloh64:
	str	x0, [x8]
Lloh65:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.3@PAGE
Lloh66:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.3@PAGEOFF]
	bl	_objc_opt_class
Lloh67:
	adrp	x8, _AV_ASSET_READER_TRACK_OUTPUT@GOTPAGE
Lloh68:
	ldr	x8, [x8, _AV_ASSET_READER_TRACK_OUTPUT@GOTPAGEOFF]
Lloh69:
	str	x0, [x8]
Lloh70:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.2@PAGE
Lloh71:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.2@PAGEOFF]
	bl	_objc_opt_class
Lloh72:
	adrp	x8, _AV_ASSET_READER@GOTPAGE
Lloh73:
	ldr	x8, [x8, _AV_ASSET_READER@GOTPAGEOFF]
Lloh74:
	str	x0, [x8]
Lloh75:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.15@PAGE
Lloh76:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.15@PAGEOFF]
	bl	_objc_opt_class
Lloh77:
	adrp	x8, _AV_AUDIO_FORMAT@GOTPAGE
Lloh78:
	ldr	x8, [x8, _AV_AUDIO_FORMAT@GOTPAGEOFF]
Lloh79:
	str	x0, [x8]
Lloh80:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.16@PAGE
Lloh81:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.16@PAGEOFF]
	bl	_objc_opt_class
Lloh82:
	adrp	x8, _AV_AUDIO_PCM_BUFFER@GOTPAGE
Lloh83:
	ldr	x8, [x8, _AV_AUDIO_PCM_BUFFER@GOTPAGEOFF]
Lloh84:
	str	x0, [x8]
Lloh85:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.17@PAGE
Lloh86:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.17@PAGEOFF]
	bl	_objc_opt_class
Lloh87:
	adrp	x8, _AV_AUDIO_COMPRESSED_BUFFER@GOTPAGE
Lloh88:
	ldr	x8, [x8, _AV_AUDIO_COMPRESSED_BUFFER@GOTPAGEOFF]
Lloh89:
	str	x0, [x8]
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.loh AdrpLdrGotStr	Lloh87, Lloh88, Lloh89
	.loh AdrpLdr	Lloh85, Lloh86
	.loh AdrpLdrGotStr	Lloh82, Lloh83, Lloh84
	.loh AdrpLdr	Lloh80, Lloh81
	.loh AdrpLdrGotStr	Lloh77, Lloh78, Lloh79
	.loh AdrpLdr	Lloh75, Lloh76
	.loh AdrpLdrGotStr	Lloh72, Lloh73, Lloh74
	.loh AdrpLdr	Lloh70, Lloh71
	.loh AdrpLdrGotStr	Lloh67, Lloh68, Lloh69
	.loh AdrpLdr	Lloh65, Lloh66
	.loh AdrpLdrGotStr	Lloh62, Lloh63, Lloh64
	.loh AdrpLdr	Lloh60, Lloh61
	.loh AdrpLdrGotStr	Lloh57, Lloh58, Lloh59
	.loh AdrpLdr	Lloh55, Lloh56
	.loh AdrpLdrGotStr	Lloh52, Lloh53, Lloh54
	.loh AdrpLdr	Lloh50, Lloh51
	.loh AdrpLdrGotStr	Lloh47, Lloh48, Lloh49
	.loh AdrpLdr	Lloh45, Lloh46
	.loh AdrpLdrGotStr	Lloh42, Lloh43, Lloh44
	.loh AdrpLdr	Lloh40, Lloh41
	.loh AdrpLdrGotStr	Lloh37, Lloh38, Lloh39
	.loh AdrpLdr	Lloh35, Lloh36
	.loh AdrpLdrGotStr	Lloh32, Lloh33, Lloh34
	.loh AdrpLdr	Lloh30, Lloh31
	.loh AdrpLdrGotStr	Lloh27, Lloh28, Lloh29
	.loh AdrpLdr	Lloh25, Lloh26
	.loh AdrpLdrGotStr	Lloh22, Lloh23, Lloh24
	.loh AdrpLdr	Lloh20, Lloh21
	.loh AdrpLdrGotStr	Lloh17, Lloh18, Lloh19
	.loh AdrpLdr	Lloh15, Lloh16
	.loh AdrpLdrGotStr	Lloh12, Lloh13, Lloh14
	.loh AdrpLdr	Lloh10, Lloh11
	.cfi_endproc
                                        ; -- End function
	.globl	_foo                            ; -- Begin function foo
	.p2align	2
_foo:                                   ; @foo
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]             ; 16-byte Folded Spill
	stp	x29, x30, [sp, #32]             ; 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
Lloh90:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_.10@PAGE
Lloh91:
	ldr	x0, [x8, _OBJC_CLASSLIST_REFERENCES_$_.10@PAGEOFF]
	bl	_objc_alloc_init
	mov	x19, x0
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	"_objc_msgSend$startAndReturnError:"
	mov	x0, x19
	bl	_objc_release
	ldp	x29, x30, [sp, #32]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #48
	ret
	.loh AdrpLdr	Lloh90, Lloh91
	.cfi_endproc
                                        ; -- End function
	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_"
_OBJC_CLASSLIST_REFERENCES_$_:
	.quad	_OBJC_CLASS_$_AVAssetWriter

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.1"
_OBJC_CLASSLIST_REFERENCES_$_.1:
	.quad	_OBJC_CLASS_$_AVURLAsset

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.2"
_OBJC_CLASSLIST_REFERENCES_$_.2:
	.quad	_OBJC_CLASS_$_AVAssetReader

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.3"
_OBJC_CLASSLIST_REFERENCES_$_.3:
	.quad	_OBJC_CLASS_$_AVAssetReaderTrackOutput

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.4"
_OBJC_CLASSLIST_REFERENCES_$_.4:
	.quad	_OBJC_CLASS_$_AVCaptureDeviceInput

	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.5"
_OBJC_CLASSLIST_REFERENCES_$_.5:
	.quad	_OBJC_CLASS_$_AVCaptureDevice

	.comm	_AV_CAPTURE_DEVICE,8,3          ; @AV_CAPTURE_DEVICE
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.6"
_OBJC_CLASSLIST_REFERENCES_$_.6:
	.quad	_OBJC_CLASS_$_AVCaptureMetadataOutput

	.comm	_AV_CAPTURE_METADATA_OUTPUT,8,3 ; @AV_CAPTURE_METADATA_OUTPUT
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.7"
_OBJC_CLASSLIST_REFERENCES_$_.7:
	.quad	_OBJC_CLASS_$_AVCaptureSession

	.comm	_AV_CAPTURE_SESSION,8,3         ; @AV_CAPTURE_SESSION
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.8"
_OBJC_CLASSLIST_REFERENCES_$_.8:
	.quad	_OBJC_CLASS_$_AVCaptureDeviceDiscoverySession

	.comm	_AV_CAPTURE_DEVICE_DISCOVERY_SESSION,8,3 ; @AV_CAPTURE_DEVICE_DISCOVERY_SESSION
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.9"
_OBJC_CLASSLIST_REFERENCES_$_.9:
	.quad	_OBJC_CLASS_$_AVCaptureVideoDataOutput

	.comm	_AV_CAPTURE_VIDEO_DATA_OUTPUT,8,3 ; @AV_CAPTURE_VIDEO_DATA_OUTPUT
	.comm	_AV_CAPTURE_DEVICE_INPUT,8,3    ; @AV_CAPTURE_DEVICE_INPUT
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.10"
_OBJC_CLASSLIST_REFERENCES_$_.10:
	.quad	_OBJC_CLASS_$_AVAudioEngine

	.comm	_AV_AUDIO_ENGINE,8,3            ; @AV_AUDIO_ENGINE
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.11"
_OBJC_CLASSLIST_REFERENCES_$_.11:
	.quad	_OBJC_CLASS_$_AVAudioTime

	.comm	_AV_AUDIO_TIME,8,3              ; @AV_AUDIO_TIME
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.12"
_OBJC_CLASSLIST_REFERENCES_$_.12:
	.quad	_OBJC_CLASS_$_AVAudioUnitEffect

	.comm	_AV_AUDIO_UNIT_EFFECT,8,3       ; @AV_AUDIO_UNIT_EFFECT
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.13"
_OBJC_CLASSLIST_REFERENCES_$_.13:
	.quad	_OBJC_CLASS_$_AVAudioUnitEQ

	.comm	_AV_AUDIO_UNIT_EQ,8,3           ; @AV_AUDIO_UNIT_EQ
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.14"
_OBJC_CLASSLIST_REFERENCES_$_.14:
	.quad	_OBJC_CLASS_$_AVAudioUnitTimeEffect

	.comm	_AV_AUDIO_UNIT_TIME_EFFECT,8,3  ; @AV_AUDIO_UNIT_TIME_EFFECT
	.comm	_AV_ASSET_READER_TRACK_OUTPUT,8,3 ; @AV_ASSET_READER_TRACK_OUTPUT
	.comm	_AV_ASSET_READER,8,3            ; @AV_ASSET_READER
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.15"
_OBJC_CLASSLIST_REFERENCES_$_.15:
	.quad	_OBJC_CLASS_$_AVAudioFormat

	.comm	_AV_AUDIO_FORMAT,8,3            ; @AV_AUDIO_FORMAT
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.16"
_OBJC_CLASSLIST_REFERENCES_$_.16:
	.quad	_OBJC_CLASS_$_AVAudioPCMBuffer

	.comm	_AV_AUDIO_PCM_BUFFER,8,3        ; @AV_AUDIO_PCM_BUFFER
	.p2align	3                               ; @"OBJC_CLASSLIST_REFERENCES_$_.17"
_OBJC_CLASSLIST_REFERENCES_$_.17:
	.quad	_OBJC_CLASS_$_AVAudioCompressedBuffer

	.comm	_AV_AUDIO_COMPRESSED_BUFFER,8,3 ; @AV_AUDIO_COMPRESSED_BUFFER
	.comm	_AV_CAPTURE_MULTI_CAM_SESSION,8,3 ; @AV_CAPTURE_MULTI_CAM_SESSION
	.section	__DATA,__mod_init_func,mod_init_funcs
	.p2align	3
	.quad	_common_initializer
	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols
