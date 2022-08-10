//
//  mtl.h
//  mtl
//
//  Created by Yury Korolev on 27.02.2022.
//

#import <Foundation/Foundation.h>
#import <Metal/Metal.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - Common

wsel(, id, reset)

//- (void)enqueue;
//wsel(, id, enqueue)
//wsel(, id, commit)
//wsel(, id, waitUntilScheduled)
//- (void)waitUntilCompleted;
//wsel(, id, waitUntilCompleted)

#pragma mark - MTLDevice

// uint64_t registryID
rsel(, id, registryID, uint64_t);

//@property (readonly) MTLSize maxThreadsPerThreadgroup API_AVAILABLE(macos(10.11), ios(9.0));
rsel(, id, maxThreadsPerThreadgroup, MTLSize);
//@property (readonly, getter=isLowPower) BOOL lowPower API_AVAILABLE(macos(10.11), macCatalyst(13.0))
//rsel(, id, isLowPower, BOOL);

//@property (readonly) BOOL hasUnifiedMemory API_AVAILABLE(macos(10.15), ios(13.0));
rsel(, id, hasUnifiedMemory, BOOL);
//@property (readonly) MTLReadWriteTextureTier readWriteTextureSupport API_AVAILABLE(macos(10.13), ios(11.0));
rsel(, id, readWriteTextureSupport, MTLReadWriteTextureTier);
//@property (readonly) MTLArgumentBuffersTier argumentBuffersSupport API_AVAILABLE(macos(10.13), ios(11.0));
rsel(, id, argumentBuffersSupport, MTLArgumentBuffersTier);


//- (nullable id <MTLCommandQueue>)newCommandQueue;
NS_RETURNS_RETAINED
rsel(, id, newCommandQueue, id <MTLCommandQueue> _Nullable);

//- (nullable id <MTLCommandQueue>)newCommandQueueWithMaxCommandBufferCount:(NSUInteger)maxCommandBufferCount;
NS_RETURNS_RETAINED
rsel_a(, id, newCommandQueueWithMaxCommandBufferCount, NSUInteger, id <MTLCommandQueue> _Nullable )

// - (nullable id <MTLTexture>)newTextureWithDescriptor:(MTLTextureDescriptor *)descriptor;
NS_RETURNS_RETAINED
rsel_a(, id, newTextureWithDescriptor, MTLTextureDescriptor *, id <MTLTexture> _Nullable )

//- (nullable id <MTLTexture>)newTextureWithDescriptor:(MTLTextureDescriptor *)descriptor iosurface:(IOSurfaceRef)iosurface plane:(NSUInteger)plane API_AVAILABLE(macos(10.11), ios(11.0));
NS_RETURNS_RETAINED
rsel_abc(, id, newTextureWithDescriptor, MTLTextureDescriptor *, iosurface, IOSurfaceRef, plane, NSUInteger, id <MTLTexture> _Nullable)


//- (nullable id <MTLLibrary>)newDefaultLibrary;
NS_RETURNS_RETAINED
rsel(, id, newDefaultLibrary, id <MTLLibrary> _Nullable)

//- (nullable id <MTLLibrary>)newLibraryWithSource:(NSString *)source options:(nullable MTLCompileOptions *)options error:(__autoreleasing NSError **)error;
NS_RETURNS_RETAINED
rsel_abc(, id, newLibraryWithSource, NSString *, options, MTLCompileOptions * _Nullable, error, NSError * _Nullable * _Nullable, id <MTLLibrary> _Nullable)

//- (void)newLibraryWithSource:(NSString *)source options:(nullable MTLCompileOptions *)options completionHandler:(MTLNewLibraryCompletionHandler)completionHandler
sel_ab_ch_ab(, id, newLibraryWithSource, NSString *, options, MTLCompileOptions * _Nullable, completionHandler, id <MTLLibrary> __nullable, NSError * __nullable)

// - (nullable id <MTLComputePipelineState>)newComputePipelineStateWithFunction:(id <MTLFunction>)computeFunction error:(__autoreleasing NSError **)error;
NS_RETURNS_RETAINED
rsel_ab(, id, newComputePipelineStateWithFunction, id<MTLFunction>, error, NSError * _Nullable * _Nullable, id<MTLComputePipelineState> _Nullable)

//- (nullable id <MTLBuffer>)newBufferWithLength:(NSUInteger)length options:(MTLResourceOptions)options;
NS_RETURNS_RETAINED
rsel_ab(, id, newBufferWithLength, NSUInteger, options, MTLResourceOptions, id<MTLBuffer> _Nullable)

NS_RETURNS_RETAINED
rsel(, id, newFence, id<MTLFence> _Nullable)

//- (nullable id <MTLEvent>)newEvent API_AVAILABLE(macos(10.14), ios(12.0));
NS_RETURNS_RETAINED
rsel(, id, newEvent, id<MTLEvent> _Nullable)

//@property (readonly) NSUInteger maxBufferLength API_AVAILABLE(macos(10.14), ios(12.0));
rsel(, id, maxBufferLength, NSUInteger)

//- (nullable id <MTLSharedEvent>)newSharedEvent API_AVAILABLE(macos(10.14), ios(12.0));
NS_RETURNS_RETAINED
rsel(, id, newSharedEvent, id<MTLSharedEvent> _Nullable)

//- (MTLSizeAndAlign)heapTextureSizeAndAlignWithDescriptor:(MTLTextureDescriptor *)desc API_AVAILABLE(macos(10.13), ios(10.0));
rsel_a(,id, heapTextureSizeAndAlignWithDescriptor, MTLTextureDescriptor *, MTLSizeAndAlign)
//- (MTLSizeAndAlign)heapBufferSizeAndAlignWithLength:(NSUInteger)length options:(MTLResourceOptions)options API_AVAILABLE(macos(10.13), ios(10.0));
rsel_ab(,id, heapBufferSizeAndAlignWithLength, NSUInteger, options, MTLResourceOptions, MTLSizeAndAlign)

#pragma mark - CompileOptions

NS_RETURNS_RETAINED csel(, MTLCompileOptions, new, MTLCompileOptions *)

//@property (readwrite, nonatomic) BOOL fastMathEnabled;
rwsel(, id, fastMathEnabled, setFastMathEnabled, BOOL)
//@property (readwrite, nonatomic) MTLLanguageVersion languageVersion
rwsel(, id, languageVersion, setLanguageVersion, MTLLanguageVersion)

//@property (readwrite, nonatomic) MTLResourceOptions resourceOptions;
rwsel(, id, resourceOptions, setResourceOptions, MTLResourceOptions)
rwsel(, id, storageMode, setStorageMode, MTLStorageMode)
rwsel(, id, cpuCacheMode, setCpuCacheMode, MTLCPUCacheMode)
rwsel(, id, hazardTrackingMode, setHazardTrackingMode, MTLHazardTrackingMode)

//@property (readwrite, nonatomic) MTLTextureType textureType;
rwsel(, id, textureType, setTextureType, MTLTextureType)
//@property (readwrite, nonatomic) MTLPixelFormat pixelFormat;
rwsel(, id, pixelFormat, setPixelFormat, MTLPixelFormat)

#pragma mark - MTLCommandBuffer

//@property (readonly) id <MTLCommandQueue> commandQueue;
//NS_RETURNS_NOT_RETAINED
//rsel(, id, commandQueue, id<MTLCommandQueue>)

// - (void)addScheduledHandler:(MTLCommandBufferHandler)block;
sel_ch_a(, id<MTLCommandBuffer>, addScheduledHandler, id <MTLCommandBuffer>)
// - (void)addCompletedHandler:(MTLCommandBufferHandler)block;
sel_ch_a(, id<MTLCommandBuffer>, addCompletedHandler, id <MTLCommandBuffer>)

//- (nullable id <MTLBlitCommandEncoder>)blitCommandEncoder;

NS_RETURNS_RETAINED
rsel(, id, blitCommandEncoder, id <MTLBlitCommandEncoder> _Nullable)

//- (nullable id <MTLComputeCommandEncoder>)computeCommandEncoder;
NS_RETURNS_RETAINED
rsel(, id, computeCommandEncoder, id <MTLComputeCommandEncoder> _Nullable)

// TextureDescriptor

// + (MTLTextureDescriptor*)texture2DDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat width:(NSUInteger)width height:(NSUInteger)height mipmapped:(BOOL)mipmapped;

//NS_RETURNS_RETAINED
csel_abcd(, MTLTextureDescriptor, texture2DDescriptorWithPixelFormat, MTLPixelFormat, width, NSUInteger, height, NSUInteger, mipmapped, BOOL,  MTLTextureDescriptor * _Nonnull)

//+ (MTLTextureDescriptor*)textureCubeDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat size:(NSUInteger)size mipmapped:(BOOL)mipmapped;

//NS_RETURNS_RETAINED
csel_abc(, MTLTextureDescriptor, textureCubeDescriptorWithPixelFormat, MTLPixelFormat, size, NSUInteger, mipmapped, BOOL, MTLTextureDescriptor * _Nonnull)

//+ (MTLTextureDescriptor*)textureBufferDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat
//                                                          width:(NSUInteger)width
//                                                resourceOptions:(MTLResourceOptions)resourceOptions
//                                                          usage:(MTLTextureUsage)usage

//NS_RETURNS_RETAINED
csel_abcd(, MTLTextureDescriptor, textureBufferDescriptorWithPixelFormat, MTLPixelFormat, width, NSUInteger, resourceOptions, MTLResourceOptions, usage, MTLTextureUsage, MTLTextureDescriptor * _Nullable)

//@property (readwrite, nonatomic) NSUInteger width;
//rwsel(, id, width, setWidth, NSUInteger)
//@property (readwrite, nonatomic) NSUInteger height;
//rwsel(, id, height, setHeight, NSUInteger)
//@property (readwrite, nonatomic) NSUInteger depth;
rwsel(, id, depth, setDepth, NSUInteger)
//@property (readwrite, nonatomic) NSUInteger mipmapLevelCount;
rwsel(, id, mipmapLevelCount, setMipmapLevelCount, NSUInteger)
//@property (readonly) NSUInteger sampleCount;
rwsel(MTLTextureDescriptor_, MTLTextureDescriptor *, sampleCount, setSampleCount, NSUInteger)
//@property (readwrite, nonatomic) NSUInteger arrayLength;
rwsel(, id, arrayLength, setArrayLength, NSUInteger)

//@property (readwrite, nonatomic) MTLTextureUsage usage API_AVAILABLE(macos(10.11), ios(9.0));
rwsel(, id, usage, setUsage, MTLTextureUsage)

//@property (readwrite, nonatomic) BOOL allowGPUOptimizedContents API_AVAILABLE(macos(10.14), ios(12.0));
rwsel(, id, allowGPUOptimizedContents, setAllowGPUOptimizedContents, BOOL)

//@property (readwrite, nonatomic) MTLTextureCompressionType compressionType API_AVAILABLE(macos(12.0), ios(15.0));

// rwsel(MTLTextureDescriptor_, MTLTextureDescriptor *, compressionType, setCompressionType, MTLTextureCompressionType)

//@property (readwrite, nonatomic) MTLTextureSwizzleChannels swizzle API_AVAILABLE(macos(10.15), ios(13.0));
rwsel(, id, swizzle, setSwizzle, MTLTextureSwizzleChannels)

// MTLTexture

//@property (nullable, readonly) id <MTLTexture> parentTexture API_AVAILABLE(macos(10.11), ios(9.0));

rsel(, id, parentTexture, id<MTLTexture> _Nullable)

//- (nullable id<MTLTexture>)newTextureViewWithPixelFormat:(MTLPixelFormat)pixelFormat;
NS_RETURNS_RETAINED
rsel_a(, id, newTextureViewWithPixelFormat, MTLPixelFormat, id <MTLTexture> _Nullable )

//@property (nullable, readonly) IOSurfaceRef iosurface API_AVAILABLE(macos(10.11), ios(11.0));
CF_RETURNS_NOT_RETAINED
rsel(, id, iosurface, IOSurfaceRef _Nullable )

//@property (readonly) NSUInteger iosurfacePlane API_AVAILABLE(macos(10.11), ios(11.0));
rsel(, id, iosurfacePlane, NSUInteger)

// MTLFunctionConstantValues

NS_RETURNS_RETAINED csel(, MTLFunctionConstantValues, new, MTLFunctionConstantValues *)
//- (void)setConstantValue:(const void *)value type:(MTLDataType)type atIndex:(NSUInteger)index;
wsel_abc(, id, setConstantValue, const void *, type, MTLDataType, atIndex, NSUInteger)
//- (void)setConstantValues:(const void *)values type:(MTLDataType)type withRange:(NSRange)range;
wsel_abc(, id, setConstantValues, const void *, type, MTLDataType, withRange, NSRange)
//- (void)setConstantValue:(const void *)value type:(MTLDataType)type withName:(NSString *)name;
wsel_abc(, id, setConstantValue, const void *, type, MTLDataType, withName, NSString *)



// MTLHeap

NS_RETURNS_RETAINED csel(, MTLHeapDescriptor, new, MTLHeapDescriptor *)


rwsel(MTLHeapType_MTLHeapDescriptor_, MTLHeapDescriptor *, type, setType, MTLHeapType)
rsel(MTLHeapType_MTLHeap_, id<MTLHeap>, type, MTLHeapType)


rsel(MTLHeapType_MTLHeap_, id<MTLHeap>, currentAllocatedSize, NSUInteger)

rsel_a(, id, maxAvailableSizeWithAlignment, NSUInteger, NSUInteger)


NS_RETURNS_RETAINED
rsel_abc(, id, newBufferWithBytes, const void *, length,
        NSUInteger, options, MTLResourceOptions,
        id<MTLBuffer> _Nullable)

NS_RETURNS_RETAINED
rsel_a(, id, newDepthStencilStateWithDescriptor, MTLDepthStencilDescriptor * _Nonnull,
      id<MTLDepthStencilState> _Nullable)


// MTLHeap end

// MTLLibrary

NS_RETURNS_RETAINED
rsel_a(, id, newFunctionWithName, NSString *, id<MTLFunction> _Nullable)

NS_RETURNS_RETAINED
rsel_abc(, id, newFunctionWithName, NSString *, constantValues, MTLFunctionConstantValues *, error, NSError * _Nullable * _Nullable, id<MTLFunction> _Nullable)

//@property (readonly) NSArray <NSString *> *functionNames;
NS_RETURNS_NOT_RETAINED
rsel(, id, functionNames, NSArray <NSString *> *)


// MTLFunction

//- (id <MTLArgumentEncoder>)newArgumentEncoderWithBufferIndex:(NSUInteger)bufferIndex API_AVAILABLE(macos(10.13), ios(11.0));
NS_RETURNS_RETAINED
rsel_a(, id, newArgumentEncoderWithBufferIndex, NSUInteger,id <MTLArgumentEncoder>)


#pragma mark - MTLArgumentsEncoder
//@property (readonly) NSUInteger encodedLength;
//rsel(, id, encodedLength, NSUInteger)

//@property (readonly) NSUInteger alignment;
//rsel(, id, alignment, NSUInteger)

//@property (readonly) MTLFunctionOptions options API_AVAILABLE(macos(11.0), ios(14.0));
rsel(, id<MTLFunction>, options, MTLFunctionOptions)


// FunctionDescriptor

//+ (nonnull MTLFunctionDescriptor *)functionDescriptor;
NS_RETURNS_NOT_RETAINED csel(, MTLFunctionDescriptor, functionDescriptor, MTLFunctionDescriptor *)

#pragma mark - MTLRenderPipelineColorAttachmentDescriptor

// @property (nonatomic, getter = isBlendingEnabled) BOOL blendingEnabled;
rwsel(, id, isBlendingEnabled, setBlendingEnabled, BOOL)
//@property (nonatomic) MTLBlendFactor sourceRGBBlendFactor;
rwsel(, id, sourceRGBBlendFactor, setSourceRGBBlendFactor, MTLBlendFactor)
//@property (nonatomic) MTLBlendFactor destinationRGBBlendFactor;
rwsel(, id, destinationRGBBlendFactor, setDestinationRGBBlendFactor, MTLBlendFactor)
// @property (nonatomic) MTLBlendOperation rgbBlendOperation;
rwsel(, id, rgbBlendOperation, setRgbBlendOperation, MTLBlendOperation)
// @property (nonatomic) MTLBlendFactor sourceAlphaBlendFactor;
rwsel(, id, sourceAlphaBlendFactor, setSourceAlphaBlendFactor, MTLBlendFactor)
// @property (nonatomic) MTLBlendFactor destinationAlphaBlendFactor;
rwsel(, id, destinationAlphaBlendFactor, setDestinationAlphaBlendFactor, MTLBlendFactor)
// @property (nonatomic) MTLBlendOperation alphaBlendOperation;
rwsel(, id, alphaBlendOperation, setAlphaBlendOperation, MTLBlendOperation)
// @property (nonatomic) MTLColorWriteMask writeMask;
rwsel(render_pipeline_, MTLRenderPipelineColorAttachmentDescriptor *, writeMask, setWriteMask, MTLColorWriteMask)


#pragma mark - MTLRenderPipelineReflection

//@property (nullable, readonly) NSArray <MTLArgument *> *vertexArguments;
NS_RETURNS_NOT_RETAINED
rsel(, id, vertexArguments, NSArray <MTLArgument *> * _Nullable)
//@property (nullable, readonly) NSArray <MTLArgument *> *fragmentArguments;
NS_RETURNS_NOT_RETAINED
rsel(, id, fragmentArguments, NSArray <MTLArgument *> * _Nullable)
// @property (nullable, readonly) NSArray <MTLArgument *> *tileArguments;
NS_RETURNS_NOT_RETAINED
rsel(, id, tileArguments, NSArray <MTLArgument *> * _Nullable)

#pragma mark - MTLRenderPipelineDescriptor

NS_RETURNS_RETAINED
csel(, MTLRenderPipelineDescriptor, new, MTLRenderPipelineDescriptor *)

// @property (nullable, readwrite, nonatomic, strong) id <MTLFunction> vertexFunction;
NS_RETURNS_NOT_RETAINED
rwsel(, id, vertexFunction, setVertexFunction, id <MTLFunction> _Nullable)
// @property (nullable, readwrite, nonatomic, strong) id <MTLFunction> fragmentFunction;
NS_RETURNS_NOT_RETAINED
rwsel(, id, fragmentFunction, setFragmentFunction, id <MTLFunction> _Nullable)
// @property (nullable, copy, nonatomic) MTLVertexDescriptor *vertexDescriptor;

//@property (readwrite, nonatomic) NSUInteger rasterSampleCount;
rwsel(, id, rasterSampleCount, setRasterSampleCount, NSUInteger)


#pragma mark - MTLRenderPipelineState

//@property (readonly) NSUInteger maxTotalThreadsPerThreadgroup API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));
rwsel(, id, maxTotalThreadsPerThreadgroup, setMaxTotalThreadsPerThreadgroup, NSUInteger)
//@property (readonly) BOOL threadgroupSizeMatchesTileSize API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));
rsel(, id, threadgroupSizeMatchesTileSize, BOOL)
//@property (readonly) NSUInteger imageblockSampleLength API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));
rsel(, id, imageblockSampleLength, NSUInteger)

#pragma mark - MTLComputePipelineDescriptor

NS_RETURNS_RETAINED
csel(, MTLComputePipelineDescriptor, new, MTLComputePipelineDescriptor *)

//@property (readwrite, nonatomic) BOOL threadGroupSizeIsMultipleOfThreadExecutionWidth;
rwsel(, id, threadGroupSizeIsMultipleOfThreadExecutionWidth, setThreadGroupSizeIsMultipleOfThreadExecutionWidth, BOOL)

//@property (nullable, readwrite, nonatomic, strong) id <MTLFunction> computeFunction;
rwsel(, id, computeFunction, setComputeFunction, id <MTLFunction> _Nullable)

#pragma mark - MTLComputePipelineState

//@property (readonly) NSUInteger maxTotalThreadsPerThreadgroup;
//@property (readonly) NSUInteger threadExecutionWidth;
rsel(, id, threadExecutionWidth, NSUInteger)
// @property (readonly) NSUInteger staticThreadgroupMemoryLength API_AVAILABLE(macos(10.13), ios(11.0));
rsel(, id, staticThreadgroupMemoryLength, NSUInteger)

#pragma mark - MTLCommandEncoder

wsel(, id, endEncoding)
wsel_a(, id, insertDebugSignpost, NSString *)
wsel_a(, id, pushDebugGroup, NSString *)
wsel(, id, popDebugGroup)

#pragma mark - MTLRenderCommandEncoder

//- (void)setRenderPipelineState:(id <MTLRenderPipelineState>)pipelineState;
wsel_a(, id, setRenderPipelineState, id <MTLRenderPipelineState>)
//- (void)setVertexBytes:(const void *)bytes length:(NSUInteger)length atIndex:(NSUInteger)index API_AVAILABLE(macos(10.11), ios(8.3));
wsel_abc(, id, setVertexBytes, const void *, length, NSUInteger, atIndex, NSUInteger)
//- (void)setVertexBuffer:(nullable id <MTLBuffer>)buffer offset:(NSUInteger)offset atIndex:(NSUInteger)index;
wsel_abc(, id, setVertexBuffer, id<MTLBuffer> _Nullable, offset, NSUInteger, atIndex, NSUInteger)
//- (void)setFragmentBuffer:(nullable id <MTLBuffer>)buffer offset:(NSUInteger)offset atIndex:(NSUInteger)index
wsel_abc(, id, setFragmentBuffer, id <MTLBuffer> _Nullable, offset, NSUInteger, atIndex, NSUInteger)
//- (void)useResource:(id <MTLResource>)resource usage:(MTLResourceUsage)usage API_AVAILABLE(macos(10.13), ios(11.0));
wsel_ab(, id, useResource, id <MTLResource>, usage, MTLResourceUsage)
//- (void)useResources:(const id <MTLResource> __nonnull[__nonnull])resources count:(NSUInteger)count usage:(MTLResourceUsage)usage API_AVAILABLE(macos(10.13), ios(11.0));
wsel_abc(, id, useResources, const id <MTLResource> _Nonnull * _Nonnull, count, NSUInteger, usage, MTLResourceUsage)
//- (void)useHeap:(id <MTLHeap>)heap API_AVAILABLE(macos(10.13), ios(11.0));
wsel_a(, id, useHeap, id <MTLHeap> _Nonnull)

//- (void)drawPrimitives:(MTLPrimitiveType)primitiveType vertexStart:(NSUInteger)vertexStart vertexCount:(NSUInteger)vertexCount instanceCount:(NSUInteger)instanceCount;
wsel_abcd(ic_, id, drawPrimitives, MTLPrimitiveType, vertexStart, NSUInteger, vertexCount, NSUInteger, instanceCount, NSUInteger)
//- (void)drawPrimitives:(MTLPrimitiveType)primitiveType vertexStart:(NSUInteger)vertexStart vertexCount:(NSUInteger)vertexCount;
wsel_abc(, id, drawPrimitives, MTLPrimitiveType, vertexStart, NSUInteger, vertexCount, NSUInteger)



#pragma mark - MTLParallelRenderCommandEncoder

//- (nullable id <MTLRenderCommandEncoder>)renderCommandEncoder;
NS_RETURNS_NOT_RETAINED
rsel(, id, renderCommandEncoder, id<MTLRenderCommandEncoder> _Nullable)

#pragma mark - MTLBlitCommandEncoder

//- (void)fillBuffer:(id<MTLBuffer>)buffer range:(NSRange)range value:(uint8_t)value;
wsel_abc(, id, fillBuffer, id<MTLBuffer>, range, NSRange, value, uint8_t)

// - (void)copyFromTexture:(id<MTLTexture>)sourceTexture sourceSlice:(NSUInteger)sourceSlice sourceLevel:(NSUInteger)sourceLevel sourceOrigin:(MTLOrigin)sourceOrigin sourceSize:(MTLSize)sourceSize toTexture:(id<MTLTexture>)destinationTexture destinationSlice:(NSUInteger)destinationSlice destinationLevel:(NSUInteger)destinationLevel destinationOrigin:(MTLOrigin)destinationOrigin;
wsel_abcdefghi(, id, copyFromTexture, id<MTLTexture>, sourceSlice, NSUInteger, sourceLevel, NSUInteger, sourceOrigin, MTLOrigin, sourceSize, MTLSize, toTexture, id<MTLTexture>, destinationSlice, NSUInteger, destinationLevel, NSUInteger, destinationOrigin, MTLOrigin)

//- (void)copyFromTexture:(id<MTLTexture>)sourceTexture toTexture:(id<MTLTexture>)destinationTexture API_AVAILABLE(macos(10.15), ios(13.0));

wsel_ab(, id, copyFromTexture, id<MTLTexture>, toTexture, id<MTLTexture>)

//- (void)optimizeContentsForGPUAccess:(id<MTLTexture>)texture API_AVAILABLE(macos(10.14), ios(12.0));
wsel_a(, id, optimizeContentsForGPUAccess, id<MTLTexture>)

//- (void) resetCommandsInBuffer:(id<MTLIndirectCommandBuffer>)buffer withRange:(NSRange)range API_AVAILABLE(macos(10.14), ios(12.0));
wsel_ab(, id, resetCommandsInBuffer, id<MTLIndirectCommandBuffer>, withRange, NSRange)



#pragma mark - MTLCommandQueue

#pragma mark - MTLRenderPassDescriptor

NS_RETURNS_RETAINED
csel(, MTLRenderPassDescriptor, new, MTLRenderPassDescriptor *)

NS_RETURNS_NOT_RETAINED
csel(, MTLRenderPassDescriptor, renderPassDescriptor, MTLRenderPassDescriptor *)

//@property (readonly) MTLRenderPassColorAttachmentDescriptorArray *colorAttachments;
NS_RETURNS_NOT_RETAINED
rsel(, id, colorAttachments, MTLRenderPassColorAttachmentDescriptorArray *)

//@property (copy, nonatomic, null_resettable) MTLRenderPassDepthAttachmentDescriptor *depthAttachment;
NS_RETURNS_NOT_RETAINED
rsel(, id, depthAttachment, MTLRenderPassDepthAttachmentDescriptor *)
wsel_a(, id, setDepthAttachment, MTLRenderPassDepthAttachmentDescriptor *)


//@property (copy, nonatomic, null_resettable) MTLRenderPassStencilAttachmentDescriptor *stencilAttachment;
NS_RETURNS_NOT_RETAINED
rsel(, id, stencilAttachment, MTLRenderPassStencilAttachmentDescriptor *)
wsel_a(, id, setStencilAttachment, MTLRenderPassStencilAttachmentDescriptor *)

//- (MTLRenderPassColorAttachmentDescriptor *)objectAtIndexedSubscript:(NSUInteger)attachmentIndex;
NS_RETURNS_NOT_RETAINED
rsel_a(MTLRenderPassColorAttachmentDescriptorArray_, MTLRenderPassColorAttachmentDescriptorArray *, objectAtIndexedSubscript, NSUInteger, MTLRenderPassColorAttachmentDescriptor *)

//- (void)setObject:(nullable MTLRenderPassColorAttachmentDescriptor *)attachment atIndexedSubscript:(NSUInteger)attachmentIndex;
wsel_ab(MTLRenderPassColorAttachmentDescriptorArray_, MTLRenderPassColorAttachmentDescriptorArray *, setObject, MTLRenderPassColorAttachmentDescriptor * _Nullable, atIndexedSubscript, NSUInteger)

// @property (nonatomic) NSUInteger tileWidth API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));

// @property (nonatomic) NSUInteger tileHeight API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));

rwsel(, id, tileWidth, setTileWidth, NSUInteger)
rwsel(, id, tileHeight, setTileHeight, NSUInteger)

rwsel(, id, defaultRasterSampleCount, setDefaultRasterSampleCount, NSUInteger)
rwsel(, id, renderTargetWidth, setRenderTargetWidth, NSUInteger)
rwsel(, id, renderTargetHeight, setRenderTargetHeight, NSUInteger)



#pragma mark - MTLRenderPassAttachmentDescriptor

//@property (nullable, nonatomic, strong) id <MTLTexture> texture;

rwsel(, id, texture, setTexture, id<MTLTexture> _Nullable)
//@property (nonatomic) NSUInteger level;
rwsel(, id, level, setLevel, NSUInteger)
rwsel(, id, slice, setSlice, NSUInteger)
rwsel(, id, depthPlane, setDepthPlane, NSUInteger)

rwsel(, id, resolveTexture, setResolveTexture, id<MTLTexture> _Nullable)
rwsel(, id, resolveLevel, setResolveLevel, NSUInteger)
rwsel(, id, resolveSlice, setResolveSlice, NSUInteger)
rwsel(, id, resolveDepthPlane, setResolveDepthPlane, NSUInteger)

//@property (nonatomic) MTLLoadAction loadAction;
rwsel(, id, loadAction, setLoadAction, MTLLoadAction)
rwsel(, id, storeAction, setStoreAction, MTLStoreAction)
rwsel(, id, storeActionOptions, setStoreActionOptions, MTLStoreActionOptions)

#pragma mark - MTLRenderPassColorAttachmentDescriptor

// @property (nonatomic) MTLClearColor clearColor;
rwsel(, id, clearColor, setClearColor, MTLClearColor)

#pragma mark - MTLRenderPassDepthAttachmentDescriptor

// @property (nonatomic) double clearDepth;
rwsel(, id, clearDepth, setClearDepth, double)
//@property (nonatomic) MTLMultisampleDepthResolveFilter depthResolveFilter API_AVAILABLE(macos(10.14), ios(9.0));
rwsel(, id, depthResolveFilter, setDepthResolveFilter, MTLMultisampleDepthResolveFilter)

#pragma mark - MTLComputePassDescriptor

csel(, MTLComputePassDescriptor, computePassDescriptor, MTLComputePassDescriptor *)
//@property (nonatomic) MTLDispatchType dispatchType;
rwsel(, id, dispatchType, setDispatchType, MTLDispatchType)

//@property (readonly) MTLComputePassSampleBufferAttachmentDescriptorArray * sampleBufferAttachments;
NS_RETURNS_NOT_RETAINED
rsel(, MTLComputePassDescriptor *, sampleBufferAttachments, MTLComputePassSampleBufferAttachmentDescriptorArray *)

#pragma mark - MTLComputePassSampleBufferAttachmentDescriptorArray


NS_RETURNS_NOT_RETAINED
rsel_a(MTLComputePassSampleBufferAttachmentDescriptorArray_, MTLComputePassSampleBufferAttachmentDescriptorArray *, objectAtIndexedSubscript, NSUInteger, MTLComputePassSampleBufferAttachmentDescriptor *)

//- (void)setObject:(nullable MTLRenderPassColorAttachmentDescriptor *)attachment atIndexedSubscript:(NSUInteger)attachmentIndex;
wsel_ab(MTLComputePassSampleBufferAttachmentDescriptorArray_, MTLComputePassSampleBufferAttachmentDescriptorArray *, setObject, MTLComputePassSampleBufferAttachmentDescriptor * _Nullable, atIndexedSubscript, NSUInteger)

#pragma mark - MTLComputePassSampleBufferAttachmentDescriptor

rwsel(, id, sampleBuffer, setSampleBuffer, id<MTLCounterSampleBuffer> _Nullable)

#pragma mark - MTLBlitPassDescriptor

NS_RETURNS_NOT_RETAINED
csel(, MTLBlitPassDescriptor, blitPassDescriptor, MTLBlitPassDescriptor *)

//@property (readonly) MTLBlitPassSampleBufferAttachmentDescriptorArray * sampleBufferAttachments;
NS_RETURNS_NOT_RETAINED
rsel(MTLBlitPassDescriptor_, MTLBlitPassDescriptor *, sampleBufferAttachments, MTLBlitPassSampleBufferAttachmentDescriptorArray *)


SEL sel_device;
SEL sel_commandBuffer;
SEL sel_commandBufferWithUnretainedReferences;
SEL sel_commandQueue;
SEL sel_commit;
SEL sel_endEncoding;
SEL sel_waitUntilCompleted;
SEL sel_waitUntilScheduled;
SEL sel_blitCommandEncoder;
SEL sel_computeCommandEncoder;
SEL sel_updateFence_a;
SEL sel_waitForFence_a;
SEL sel_newCommandQueue;
SEL sel_newFence;
SEL sel_useResource_usage;
SEL sel_useResources_count_usage;
SEL sel_enqueue;
//- (void)setTexture:(nullable id <MTLTexture>)texture atIndex:(NSUInteger)index;
SEL sel_setTexture_atIndex;
//- (void)setArgumentBuffer:(nullable id <MTLBuffer>)argumentBuffer offset:(NSUInteger)offset;
SEL sel_setArgumentBuffer_offset;
SEL sel_aligment;
SEL sel_encodedLength;
SEL sel_setComputePipelineState;
SEL sel_dispatchThreads_threadsPerThreadgroup;
SEL sel_dispatchThreadgroups_threadsPerThreadgroup;
//- (void)setTextures:(const id <MTLTexture> __nullable [__nonnull])textures withRange:(NSRange)range
SEL sel_setTextures_withRange;
SEL sel_setImageblockWidth_height;
SEL sel_contents;
SEL sel_gpuAddress;
SEL sel_gpuResourceID;


__attribute__((constructor))
static void mtl_initializer()
{
    static int initialized = 0;
    if (!initialized) {
      
      sel_device = @selector(device);
      sel_commandBuffer = @selector(commandBuffer);
      sel_commandBufferWithUnretainedReferences = @selector(commandBufferWithUnretainedReferences);
      
      sel_commandQueue = @selector(commandQueue);
      sel_commit = @selector(commit);
      sel_endEncoding = @selector(endEncoding);
      sel_waitUntilCompleted = @selector(waitUntilCompleted);
      sel_waitUntilScheduled = @selector(waitUntilScheduled);
      sel_blitCommandEncoder = @selector(blitCommandEncoder);
      sel_computeCommandEncoder = @selector(computeCommandEncoder);
      sel_updateFence_a = @selector(updateFence:);
      sel_waitForFence_a = @selector(waitForFence:);
      sel_newCommandQueue = @selector(newCommandQueue);
      sel_newFence = @selector(newFence);
      sel_useResource_usage = @selector(useResource:usage:);
      sel_useResources_count_usage = @selector(useResources:count:usage:);
      sel_enqueue = @selector(enqueue);
      sel_setTexture_atIndex = @selector(setTexture:atIndex:);
      sel_setArgumentBuffer_offset = @selector(setArgumentBuffer:offset:);
      sel_aligment = @selector(alignment);
      sel_encodedLength = @selector(encodedLength);
      sel_setComputePipelineState = @selector(setComputePipelineState:);
      sel_dispatchThreads_threadsPerThreadgroup = @selector( dispatchThreads:threadsPerThreadgroup:);
      sel_dispatchThreadgroups_threadsPerThreadgroup = @selector(dispatchThreadgroups:threadsPerThreadgroup:);
      sel_setTextures_withRange = @selector(setTextures:withRange:);
      sel_setImageblockWidth_height = @selector(setImageblockWidth:height:);
      sel_contents = @selector(contents);
      sel_gpuAddress = @selector(gpuAddress);
      sel_gpuResourceID = @selector(gpuResourceID);


      initialized = 1;
    }
}


NS_ASSUME_NONNULL_END
