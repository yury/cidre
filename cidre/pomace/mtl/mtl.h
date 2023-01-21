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

NS_RETURNS_RETAINED
csel4(, MTLTextureDescriptor, texture2DDescriptorWithPixelFormat, MTLPixelFormat, width, NSUInteger, height, NSUInteger, mipmapped, BOOL,  MTLTextureDescriptor * _Nonnull)

//+ (MTLTextureDescriptor*)textureCubeDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat size:(NSUInteger)size mipmapped:(BOOL)mipmapped;

NS_RETURNS_RETAINED
csel3(, MTLTextureDescriptor, textureCubeDescriptorWithPixelFormat, MTLPixelFormat, size, NSUInteger, mipmapped, BOOL, MTLTextureDescriptor * _Nonnull)

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
Class MTL_RENDER_PASS_DESCRIPTOR;
Class MTL_BLIT_PASS_DESCRIPTOR;
Class MTL_RENDER_PIPELINE_DESCRIPTOR;
Class MTL_FUNCTION_DESCRIPTOR;

__attribute__((constructor))
static void mtl_initializer()
{
    static int initialized = 0;
    if (!initialized) {
      
      MTL_COMPUTE_PASS_DESCRIPTOR = [MTLComputePassDescriptor class];
      MTL_HEAP_DESCRIPTOR = [MTLHeapDescriptor class];
      MTL_COMPILE_OPTIONS = [MTLCompileOptions class];
      
      MTL_COMPUTE_PIPELINE_DESCRIPTOR = [MTLComputePipelineDescriptor class];
      MTL_RENDER_PASS_DESCRIPTOR = [MTLRenderPassDescriptor class];
      MTL_BLIT_PASS_DESCRIPTOR = [MTLBlitPassDescriptor class];
      MTL_RENDER_PIPELINE_DESCRIPTOR = [MTLRenderPipelineDescriptor class];
      MTL_FUNCTION_DESCRIPTOR = [MTLFunctionDescriptor class];
      
//      sel_waitUntilCompleted = @selector(waitUntilCompleted);
//      sel_waitUntilScheduled = @selector(waitUntilScheduled);
//      sel_blitCommandEncoder = @selector(blitCommandEncoder);
//      sel_computeCommandEncoder = @selector(computeCommandEncoder);
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
