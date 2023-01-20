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
rsel3(, id, newLibraryWithSource, NSString *, options, MTLCompileOptions * _Nullable, error, NSError * _Nullable * _Nullable, id <MTLLibrary> _Nullable)

wsel3(, id, newLibraryWithSource, NSString *, options, MTLCompileOptions * _Nullable, completionHandler, id)

NS_RETURNS_RETAINED
rsel2(, id, newRenderPipelineStateWithDescriptor, MTLRenderPipelineDescriptor *, error, NSError * _Nullable * _Nullable, id<MTLRenderPipelineState> _Nullable)

#pragma mark - MTLCommandBuffer

wsel1(, id<MTLCommandBuffer>, addScheduledHandler, id)
wsel1(, id<MTLCommandBuffer>, addCompletedHandler, id)

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


NS_RETURNS_RETAINED
rsel1(, id, newDepthStencilStateWithDescriptor, MTLDepthStencilDescriptor * _Nonnull,
      id<MTLDepthStencilState> _Nullable)

// MTLFunction

NS_RETURNS_RETAINED
rsel1(, id, newArgumentEncoderWithBufferIndex, NSUInteger,id <MTLArgumentEncoder>)


#pragma mark - MTLArgumentsEncoder


// FunctionDescriptor

//+ (nonnull MTLFunctionDescriptor *)functionDescriptor;
NS_RETURNS_NOT_RETAINED
csel0(, MTLFunctionDescriptor, functionDescriptor, MTLFunctionDescriptor *)

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

// rwsel(, id, tileWidth, setTileWidth, NSUInteger)
// rwsel(, id, tileHeight, setTileHeight, NSUInteger)

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
SEL sel_renderCommandEncoderWithDescriptor;
SEL sel_drawPrimitives_vertexStart_vertexCount;
SEL sel_drawPrimitives_vertexStart_vertexCount_instanceCount;
SEL sel_setVertexBuffer_offset_atIndex;
SEL sel_setFragmentBuffer_offset_atIndex;
SEL sel_computeCommandEncoderWithDescriptor;

Class MTL_COMPUTE_PASS_DESCRIPTOR;
Class MTL_HEAP_DESCRIPTOR;
Class MTL_COMPILE_OPTIONS;

Class MTL_COMPUTE_PIPELINE_DESCRIPTOR;

__attribute__((constructor))
static void mtl_initializer()
{
    static int initialized = 0;
    if (!initialized) {
      
      MTL_COMPUTE_PASS_DESCRIPTOR = [MTLComputePassDescriptor class];
      MTL_HEAP_DESCRIPTOR = [MTLHeapDescriptor class];
      MTL_COMPILE_OPTIONS = [MTLCompileOptions class];
      
      MTL_COMPUTE_PIPELINE_DESCRIPTOR = [MTLComputePipelineDescriptor class];
      
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
