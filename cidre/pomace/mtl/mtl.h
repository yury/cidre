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

NS_RETURNS_RETAINED
csel3(, MTLTextureDescriptor, textureCubeDescriptorWithPixelFormat, MTLPixelFormat, size, NSUInteger, mipmapped, BOOL, MTLTextureDescriptor * _Nonnull)

//NS_RETURNS_RETAINED
csel4(, MTLTextureDescriptor, textureBufferDescriptorWithPixelFormat, MTLPixelFormat, width, NSUInteger, resourceOptions, MTLResourceOptions, usage, MTLTextureUsage, MTLTextureDescriptor * _Nullable)

//// MTLFunction
//
//NS_RETURNS_RETAINED
//rsel1(, id, newArgumentEncoderWithBufferIndex, NSUInteger,id <MTLArgumentEncoder>)

Class MTL_COMPUTE_PASS_DESCRIPTOR;
Class MTL_HEAP_DESCRIPTOR;
Class MTL_COMPILE_OPTIONS;

Class MTL_COMPUTE_PIPELINE_DESCRIPTOR;
Class MTL_RENDER_PASS_DESCRIPTOR;
Class MTL_BLIT_PASS_DESCRIPTOR;
Class MTL_RENDER_PIPELINE_DESCRIPTOR;
Class MTL_FUNCTION_DESCRIPTOR;
Class MTL_FUNCTION_CONSTANT_VALUES;

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
      MTL_FUNCTION_CONSTANT_VALUES = [MTLFunctionConstantValues class];

      initialized = 1;
    }
}


NS_ASSUME_NONNULL_END
