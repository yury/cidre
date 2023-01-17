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

#pragma mark - MTLDevice

NS_RETURNS_RETAINED
rsel0(, id, newCommandQueue, id <MTLCommandQueue> _Nullable);

NS_RETURNS_RETAINED
rsel1(, id, newCommandQueueWithMaxCommandBufferCount, NSUInteger, id <MTLCommandQueue> _Nullable )

NS_RETURNS_RETAINED
rsel3(, id, newTextureWithDescriptor, MTLTextureDescriptor *, iosurface, IOSurfaceRef, plane, NSUInteger, id <MTLTexture> _Nullable)

NS_RETURNS_RETAINED
rsel3(, id, newLibraryWithSource, NSString *, options, MTLCompileOptions * _Nullable, error, NSError * _Nullable * _Nullable, id <MTLLibrary> _Nullable)

wsel3(, id, newLibraryWithSource, NSString *, options, MTLCompileOptions * _Nullable, completionHandler, id)

NS_RETURNS_RETAINED
rsel2(, id, newComputePipelineStateWithFunction, id<MTLFunction>, error, NSError * _Nullable * _Nullable, id<MTLComputePipelineState> _Nullable)

NS_RETURNS_RETAINED
rsel2(, id, newRenderPipelineStateWithDescriptor, MTLRenderPipelineDescriptor *, error, NSError * _Nullable * _Nullable, id<MTLRenderPipelineState> _Nullable)

NS_RETURNS_RETAINED
rsel0(, id, newFence, id<MTLFence> _Nullable)

//- (nullable id <MTLEvent>)newEvent API_AVAILABLE(macos(10.14), ios(12.0));
NS_RETURNS_RETAINED
rsel0(, id, newEvent, id<MTLEvent> _Nullable)

//@property (readonly) NSUInteger maxBufferLength API_AVAILABLE(macos(10.14), ios(12.0));
rsel0(, id, maxBufferLength, NSUInteger)

//- (nullable id <MTLSharedEvent>)newSharedEvent API_AVAILABLE(macos(10.14), ios(12.0));
NS_RETURNS_RETAINED
rsel0(, id, newSharedEvent, id<MTLSharedEvent> _Nullable)

//- (MTLSizeAndAlign)heapTextureSizeAndAlignWithDescriptor:(MTLTextureDescriptor *)desc API_AVAILABLE(macos(10.13), ios(10.0));
rsel1(,id, heapTextureSizeAndAlignWithDescriptor, MTLTextureDescriptor *, MTLSizeAndAlign)
//- (MTLSizeAndAlign)heapBufferSizeAndAlignWithLength:(NSUInteger)length options:(MTLResourceOptions)options API_AVAILABLE(macos(10.13), ios(10.0));
rsel2(,id, heapBufferSizeAndAlignWithLength, NSUInteger, options, MTLResourceOptions, MTLSizeAndAlign)

#pragma mark - CompileOptions

NS_RETURNS_RETAINED
csel0(, MTLCompileOptions, new, MTLCompileOptions *)

//@property (readwrite, nonatomic) BOOL fastMathEnabled;
rwsel(, id, fastMathEnabled, setFastMathEnabled, BOOL)
//@property (readwrite, nonatomic) MTLLanguageVersion languageVersion
rwsel(, id, languageVersion, setLanguageVersion, MTLLanguageVersion)

//@property (readwrite, nonatomic) MTLTextureType textureType;
rwsel(, id, textureType, setTextureType, MTLTextureType)
//@property (readwrite, nonatomic) MTLPixelFormat pixelFormat;
rwsel(, id, pixelFormat, setPixelFormat, MTLPixelFormat)

#pragma mark - MTLCommandBuffer

//@property (readonly) id <MTLCommandQueue> commandQueue;
//NS_RETURNS_NOT_RETAINED
//rsel(, id, commandQueue, id<MTLCommandQueue>)

// - (void)addScheduledHandler:(MTLCommandBufferHandler)block;
wsel1(, id<MTLCommandBuffer>, addScheduledHandler, id)
// - (void)addCompletedHandler:(MTLCommandBufferHandler)block;
wsel1(, id<MTLCommandBuffer>, addCompletedHandler, id)

//- (nullable id <MTLBlitCommandEncoder>)blitCommandEncoder;

NS_RETURNS_RETAINED
rsel0(, id, blitCommandEncoder, id <MTLBlitCommandEncoder> _Nullable)

//- (nullable id <MTLComputeCommandEncoder>)computeCommandEncoder;
NS_RETURNS_RETAINED
rsel0(, id, computeCommandEncoder, id <MTLComputeCommandEncoder> _Nullable)

// TextureDescriptor

// + (MTLTextureDescriptor*)texture2DDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat width:(NSUInteger)width height:(NSUInteger)height mipmapped:(BOOL)mipmapped;

NS_RETURNS_RETAINED
csel4(, MTLTextureDescriptor, texture2DDescriptorWithPixelFormat, MTLPixelFormat, width, NSUInteger, height, NSUInteger, mipmapped, BOOL,  MTLTextureDescriptor * _Nonnull)

//+ (MTLTextureDescriptor*)textureCubeDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat size:(NSUInteger)size mipmapped:(BOOL)mipmapped;

NS_RETURNS_RETAINED
csel3(, MTLTextureDescriptor, textureCubeDescriptorWithPixelFormat, MTLPixelFormat, size, NSUInteger, mipmapped, BOOL, MTLTextureDescriptor * _Nonnull)

//+ (MTLTextureDescriptor*)textureBufferDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat
//                                                          width:(NSUInteger)width
//                                                resourceOptions:(MTLResourceOptions)resourceOptions
//                                                          usage:(MTLTextureUsage)usage

//NS_RETURNS_RETAINED
csel4(, MTLTextureDescriptor, textureBufferDescriptorWithPixelFormat, MTLPixelFormat, width, NSUInteger, resourceOptions, MTLResourceOptions, usage, MTLTextureUsage, MTLTextureDescriptor * _Nullable)


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

rsel0(, id, parentTexture, id<MTLTexture> _Nullable)

//- (nullable id<MTLTexture>)newTextureViewWithPixelFormat:(MTLPixelFormat)pixelFormat;
NS_RETURNS_RETAINED
rsel1(, id, newTextureViewWithPixelFormat, MTLPixelFormat, id <MTLTexture> _Nullable )

//@property (nullable, readonly) IOSurfaceRef iosurface API_AVAILABLE(macos(10.11), ios(11.0));
CF_RETURNS_NOT_RETAINED
rsel0(, id, iosurface, IOSurfaceRef _Nullable )

//@property (readonly) NSUInteger iosurfacePlane API_AVAILABLE(macos(10.11), ios(11.0));
rsel0(, id, iosurfacePlane, NSUInteger)

// MTLFunctionConstantValues

NS_RETURNS_RETAINED
csel0(, MTLFunctionConstantValues, new, MTLFunctionConstantValues *)
//- (void)setConstantValue:(const void *)value type:(MTLDataType)type atIndex:(NSUInteger)index;
wsel3(, id, setConstantValue, const void *, type, MTLDataType, atIndex, NSUInteger)
//- (void)setConstantValues:(const void *)values type:(MTLDataType)type withRange:(NSRange)range;
wsel3(, id, setConstantValues, const void *, type, MTLDataType, withRange, NSRange)
//- (void)setConstantValue:(const void *)value type:(MTLDataType)type withName:(NSString *)name;
wsel3(, id, setConstantValue, const void *, type, MTLDataType, withName, NSString *)



#pragma mark - MTLHeapDescriptor

rwsel(MTLHeapType_MTLHeapDescriptor_, MTLHeapDescriptor *, type, setType, MTLHeapType)
rsel0(MTLHeapType_MTLHeap_, id<MTLHeap>, type, MTLHeapType)


rsel0(MTLHeapType_MTLHeap_, id<MTLHeap>, currentAllocatedSize, NSUInteger)

rsel1(, id, maxAvailableSizeWithAlignment, NSUInteger, NSUInteger)

NS_RETURNS_RETAINED
rsel1(, id, newDepthStencilStateWithDescriptor, MTLDepthStencilDescriptor * _Nonnull,
      id<MTLDepthStencilState> _Nullable)


// MTLHeap end

// MTLLibrary

NS_RETURNS_RETAINED
rsel1(, id, newFunctionWithName, NSString *, id<MTLFunction> _Nullable)

NS_RETURNS_RETAINED
rsel3(, id, newFunctionWithName, NSString *, constantValues, MTLFunctionConstantValues *, error, NSError * _Nullable * _Nullable, id<MTLFunction> _Nullable)

//@property (readonly) NSArray <NSString *> *functionNames;
NS_RETURNS_NOT_RETAINED
rsel0(, id, functionNames, NSArray <NSString *> *)


// MTLFunction

//- (id <MTLArgumentEncoder>)newArgumentEncoderWithBufferIndex:(NSUInteger)bufferIndex API_AVAILABLE(macos(10.13), ios(11.0));
NS_RETURNS_RETAINED
rsel1(, id, newArgumentEncoderWithBufferIndex, NSUInteger,id <MTLArgumentEncoder>)


#pragma mark - MTLArgumentsEncoder
//@property (readonly) NSUInteger encodedLength;
//rsel(, id, encodedLength, NSUInteger)

//@property (readonly) NSUInteger alignment;
//rsel(, id, alignment, NSUInteger)

//@property (readonly) MTLFunctionOptions options API_AVAILABLE(macos(11.0), ios(14.0));
rsel0(, id<MTLFunction>, options, MTLFunctionOptions)


// FunctionDescriptor

//+ (nonnull MTLFunctionDescriptor *)functionDescriptor;
NS_RETURNS_NOT_RETAINED
csel0(, MTLFunctionDescriptor, functionDescriptor, MTLFunctionDescriptor *)

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
rsel0(, id, vertexArguments, NSArray <MTLArgument *> * _Nullable)
//@property (nullable, readonly) NSArray <MTLArgument *> *fragmentArguments;
NS_RETURNS_NOT_RETAINED
rsel0(, id, fragmentArguments, NSArray <MTLArgument *> * _Nullable)
// @property (nullable, readonly) NSArray <MTLArgument *> *tileArguments;
NS_RETURNS_NOT_RETAINED
rsel0(, id, tileArguments, NSArray <MTLArgument *> * _Nullable)

#pragma mark - MTLRenderPipelineDescriptor

NS_RETURNS_RETAINED
csel0(, MTLRenderPipelineDescriptor, new, MTLRenderPipelineDescriptor *)

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
rsel0(, id, threadgroupSizeMatchesTileSize, BOOL)
//@property (readonly) NSUInteger imageblockSampleLength API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));
rsel0(, id, imageblockSampleLength, NSUInteger)

#pragma mark - MTLComputePipelineDescriptor

NS_RETURNS_RETAINED
csel0(, MTLComputePipelineDescriptor, new, MTLComputePipelineDescriptor *)

//@property (readwrite, nonatomic) BOOL threadGroupSizeIsMultipleOfThreadExecutionWidth;
rwsel(, id, threadGroupSizeIsMultipleOfThreadExecutionWidth, setThreadGroupSizeIsMultipleOfThreadExecutionWidth, BOOL)

//@property (nullable, readwrite, nonatomic, strong) id <MTLFunction> computeFunction;
rwsel(, id, computeFunction, setComputeFunction, id <MTLFunction> _Nullable)

#pragma mark - MTLComputePipelineState

//@property (readonly) NSUInteger maxTotalThreadsPerThreadgroup;
//@property (readonly) NSUInteger threadExecutionWidth;
rsel0(, id, threadExecutionWidth, NSUInteger)
// @property (readonly) NSUInteger staticThreadgroupMemoryLength API_AVAILABLE(macos(10.13), ios(11.0));
rsel0(, id, staticThreadgroupMemoryLength, NSUInteger)

#pragma mark - MTLCommandEncoder

wsel0(, id, endEncoding)
wsel1(, id, insertDebugSignpost, NSString *)

#pragma mark - MTLRenderCommandEncoder

wsel3(, id, setVertexBytes, const void *, length, NSUInteger, atIndex, NSUInteger)
//- (void)setVertexBuffer:(nullable id <MTLBuffer>)buffer offset:(NSUInteger)offset atIndex:(NSUInteger)index;
wsel3(, id, setVertexBuffer, id<MTLBuffer> _Nullable, offset, NSUInteger, atIndex, NSUInteger)
//- (void)setFragmentBuffer:(nullable id <MTLBuffer>)buffer offset:(NSUInteger)offset atIndex:(NSUInteger)index
wsel3(, id, setFragmentBuffer, id <MTLBuffer> _Nullable, offset, NSUInteger, atIndex, NSUInteger)
//- (void)useResource:(id <MTLResource>)resource usage:(MTLResourceUsage)usage API_AVAILABLE(macos(10.13), ios(11.0));
wsel2(, id, useResource, id <MTLResource>, usage, MTLResourceUsage)
//- (void)useResources:(const id <MTLResource> __nonnull[__nonnull])resources count:(NSUInteger)count usage:(MTLResourceUsage)usage API_AVAILABLE(macos(10.13), ios(11.0));
wsel3(, id, useResources, const id <MTLResource> _Nonnull * _Nonnull, count, NSUInteger, usage, MTLResourceUsage)

//- (void)drawPrimitives:(MTLPrimitiveType)primitiveType vertexStart:(NSUInteger)vertexStart vertexCount:(NSUInteger)vertexCount instanceCount:(NSUInteger)instanceCount;
//wsel_abcd(ic_, id, drawPrimitives, MTLPrimitiveType, vertexStart, NSUInteger, vertexCount, NSUInteger, instanceCount, NSUInteger)
//- (void)drawPrimitives:(MTLPrimitiveType)primitiveType vertexStart:(NSUInteger)vertexStart vertexCount:(NSUInteger)vertexCount;
//wsel_abc(ic_, id, drawPrimitives, MTLPrimitiveType, vertexStart, NSUInteger, vertexCount, NSUInteger)



#pragma mark - MTLParallelRenderCommandEncoder

//- (nullable id <MTLRenderCommandEncoder>)renderCommandEncoder;
NS_RETURNS_NOT_RETAINED
rsel0(, id, renderCommandEncoder, id<MTLRenderCommandEncoder> _Nullable)

#pragma mark - MTLBlitCommandEncoder

//- (void)fillBuffer:(id<MTLBuffer>)buffer range:(NSRange)range value:(uint8_t)value;
wsel3(, id, fillBuffer, id<MTLBuffer>, range, NSRange, value, uint8_t)

// - (void)copyFromTexture:(id<MTLTexture>)sourceTexture sourceSlice:(NSUInteger)sourceSlice sourceLevel:(NSUInteger)sourceLevel sourceOrigin:(MTLOrigin)sourceOrigin sourceSize:(MTLSize)sourceSize toTexture:(id<MTLTexture>)destinationTexture destinationSlice:(NSUInteger)destinationSlice destinationLevel:(NSUInteger)destinationLevel destinationOrigin:(MTLOrigin)destinationOrigin;
wsel9(, id, copyFromTexture, id<MTLTexture>, sourceSlice, NSUInteger, sourceLevel, NSUInteger, sourceOrigin, MTLOrigin, sourceSize, MTLSize, toTexture, id<MTLTexture>, destinationSlice, NSUInteger, destinationLevel, NSUInteger, destinationOrigin, MTLOrigin)

//- (void)copyFromTexture:(id<MTLTexture>)sourceTexture toTexture:(id<MTLTexture>)destinationTexture API_AVAILABLE(macos(10.15), ios(13.0));

wsel2(, id, copyFromTexture, id<MTLTexture>, toTexture, id<MTLTexture>)

//- (void)optimizeContentsForGPUAccess:(id<MTLTexture>)texture API_AVAILABLE(macos(10.14), ios(12.0));
wsel1(, id, optimizeContentsForGPUAccess, id<MTLTexture>)

//- (void) resetCommandsInBuffer:(id<MTLIndirectCommandBuffer>)buffer withRange:(NSRange)range API_AVAILABLE(macos(10.14), ios(12.0));
wsel2(, id, resetCommandsInBuffer, id<MTLIndirectCommandBuffer>, withRange, NSRange)


#pragma mark - MTLCommandQueue

#pragma mark - MTLRenderPassDescriptor

NS_RETURNS_RETAINED
csel0(, MTLRenderPassDescriptor, new, MTLRenderPassDescriptor *)

NS_RETURNS_NOT_RETAINED
csel0(, MTLRenderPassDescriptor, renderPassDescriptor, MTLRenderPassDescriptor *)

//@property (readonly) MTLRenderPassColorAttachmentDescriptorArray *colorAttachments;
NS_RETURNS_NOT_RETAINED
rsel0(, id, colorAttachments, MTLRenderPassColorAttachmentDescriptorArray *)

//@property (copy, nonatomic, null_resettable) MTLRenderPassDepthAttachmentDescriptor *depthAttachment;
NS_RETURNS_NOT_RETAINED
rsel0(, id, depthAttachment, MTLRenderPassDepthAttachmentDescriptor *)
wsel1(, id, setDepthAttachment, MTLRenderPassDepthAttachmentDescriptor *)


//@property (copy, nonatomic, null_resettable) MTLRenderPassStencilAttachmentDescriptor *stencilAttachment;
NS_RETURNS_NOT_RETAINED
rsel0(, id, stencilAttachment, MTLRenderPassStencilAttachmentDescriptor *)
wsel1(, id, setStencilAttachment, MTLRenderPassStencilAttachmentDescriptor *)

//- (MTLRenderPassColorAttachmentDescriptor *)objectAtIndexedSubscript:(NSUInteger)attachmentIndex;
NS_RETURNS_NOT_RETAINED
rsel1(MTLRenderPassColorAttachmentDescriptorArray_, MTLRenderPassColorAttachmentDescriptorArray *, objectAtIndexedSubscript, NSUInteger, MTLRenderPassColorAttachmentDescriptor *)

//- (void)setObject:(nullable MTLRenderPassColorAttachmentDescriptor *)attachment atIndexedSubscript:(NSUInteger)attachmentIndex;
wsel2(MTLRenderPassColorAttachmentDescriptorArray_, MTLRenderPassColorAttachmentDescriptorArray *, setObject, MTLRenderPassColorAttachmentDescriptor * _Nullable, atIndexedSubscript, NSUInteger)

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

//@property (readonly) MTLComputePassSampleBufferAttachmentDescriptorArray * sampleBufferAttachments;
NS_RETURNS_NOT_RETAINED
rsel0(, MTLComputePassDescriptor *, sampleBufferAttachments, MTLComputePassSampleBufferAttachmentDescriptorArray *)

#pragma mark - MTLRenderPipelineColorAttachmentDescriptorArray


NS_RETURNS_NOT_RETAINED
rsel1(MTLRenderPipelineColorAttachmentDescriptorArray_, MTLRenderPipelineColorAttachmentDescriptorArray *, objectAtIndexedSubscript, NSUInteger, MTLRenderPipelineColorAttachmentDescriptor *)

wsel2(MTLRenderPipelineColorAttachmentDescriptorArray_, MTLRenderPipelineColorAttachmentDescriptorArray *, setObject, MTLRenderPipelineColorAttachmentDescriptor * _Nullable, atIndexedSubscript, NSUInteger)

#pragma mark - MTLBlitPassDescriptor

NS_RETURNS_NOT_RETAINED
csel0(, MTLBlitPassDescriptor, blitPassDescriptor, MTLBlitPassDescriptor *)

//@property (readonly) MTLBlitPassSampleBufferAttachmentDescriptorArray * sampleBufferAttachments;
NS_RETURNS_NOT_RETAINED
rsel0(MTLBlitPassDescriptor_, MTLBlitPassDescriptor *, sampleBufferAttachments, MTLBlitPassSampleBufferAttachmentDescriptorArray *)


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
SEL sel_setFragmentTexture_atIndex;
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
SEL sel_renderCommandEncoderWithDescriptor;
SEL sel_drawPrimitives_vertexStart_vertexCount;
SEL sel_drawPrimitives_vertexStart_vertexCount_instanceCount;
SEL sel_setVertexBuffer_offset_atIndex;
SEL sel_setFragmentBuffer_offset_atIndex;
SEL sel_computeCommandEncoderWithDescriptor;

Class MTL_COMPUTE_PASS_DESCRIPTOR;
Class MTL_HEAP_DESCRIPTOR;

__attribute__((constructor))
static void mtl_initializer()
{
    static int initialized = 0;
    if (!initialized) {
      
      MTL_COMPUTE_PASS_DESCRIPTOR = [MTLComputePassDescriptor class];
      MTL_HEAP_DESCRIPTOR = [MTLHeapDescriptor class];
      
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
      sel_setFragmentTexture_atIndex = @selector(setFragmentTexture:atIndex:);
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
      sel_renderCommandEncoderWithDescriptor = @selector(renderCommandEncoderWithDescriptor:);
      sel_drawPrimitives_vertexStart_vertexCount = @selector(drawPrimitives:vertexStart:vertexCount:);
      sel_drawPrimitives_vertexStart_vertexCount_instanceCount = @selector(drawPrimitives:vertexStart:vertexCount:instanceCount:);
      sel_setVertexBuffer_offset_atIndex = @selector(setVertexBuffer:offset:atIndex:);
      sel_setFragmentBuffer_offset_atIndex = @selector(setFragmentBuffer:offset:atIndex:);
      sel_computeCommandEncoderWithDescriptor = @selector(computeCommandEncoderWithDescriptor:);


      initialized = 1;
    }
}


NS_ASSUME_NONNULL_END
