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

Class MTL_COMPUTE_PASS_DESCRIPTOR;
Class MTL_HEAP_DESCRIPTOR;
Class MTL_COMPILE_OPTIONS;

Class MTL_COMPUTE_PIPELINE_DESCRIPTOR;
Class MTL_RENDER_PASS_DESCRIPTOR;
Class MTL_BLIT_PASS_DESCRIPTOR;
Class MTL_RENDER_PIPELINE_DESCRIPTOR;
Class MTL_FUNCTION_DESCRIPTOR;
Class MTL_FUNCTION_CONSTANT_VALUES;
Class MTL_TEXTURE_DESCRIPTOR;
Class MTL_STENCIL_DESCRIPTOR;
Class MTL_DEPTH_STENCIL_DESCRIPTOR;

Class MTL_VISIBLE_FUNCTION_TABLE_DESCRIPTOR;
Class MTL_INTERSECTION_FUNCTION_TABLE_DESCRIPTOR;
Class MTL_COUNTER_SAMPLE_BUFFER_DESCRIPTOR;

Class MTL_ACCELERATION_STRUCTURE_GEOMETRY_DESCRIPTOR;
Class MTL_ACCELERATION_STRUCTURE_TRIANGLE_GEOMETRY_DESCRIPTOR;
Class MTL_ACCELERATION_STRUCTURE_MOTION_BOUNDING_BOX_GEOMETRY_DESCRIPTOR;

Class MTL_VERTEX_DESCRIPTOR;
Class MTL_VERTEX_BUFFER_LAYOUT_DESCRIPTOR;
Class MTL_VERTEX_ATTRIBUTE_DESCRIPTOR;

Class MTL_PIPELINE_BUFFER_DESCRIPTOR;


__attribute__((constructor))
static void mtl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
      MTL_HEAP_DESCRIPTOR = [MTLHeapDescriptor class];
      MTL_COMPILE_OPTIONS = [MTLCompileOptions class];
      
      MTL_COMPUTE_PASS_DESCRIPTOR = [MTLComputePassDescriptor class];
      MTL_COMPUTE_PIPELINE_DESCRIPTOR = [MTLComputePipelineDescriptor class];
      MTL_RENDER_PASS_DESCRIPTOR = [MTLRenderPassDescriptor class];
      MTL_BLIT_PASS_DESCRIPTOR = [MTLBlitPassDescriptor class];
      MTL_RENDER_PIPELINE_DESCRIPTOR = [MTLRenderPipelineDescriptor class];
      MTL_FUNCTION_DESCRIPTOR = [MTLFunctionDescriptor class];
      MTL_FUNCTION_CONSTANT_VALUES = [MTLFunctionConstantValues class];
      MTL_TEXTURE_DESCRIPTOR = [MTLTextureDescriptor class];
      MTL_STENCIL_DESCRIPTOR = [MTLStencilDescriptor class];
      MTL_DEPTH_STENCIL_DESCRIPTOR = [MTLDepthStencilDescriptor class];
      
      MTL_VISIBLE_FUNCTION_TABLE_DESCRIPTOR = [MTLVisibleFunctionTableDescriptor class];
      MTL_INTERSECTION_FUNCTION_TABLE_DESCRIPTOR = [MTLIntersectionFunctionTableDescriptor class];
      MTL_COUNTER_SAMPLE_BUFFER_DESCRIPTOR = [MTLCounterSampleBufferDescriptor class];
      
      MTL_ACCELERATION_STRUCTURE_GEOMETRY_DESCRIPTOR = [MTLAccelerationStructureGeometryDescriptor class];
      MTL_ACCELERATION_STRUCTURE_TRIANGLE_GEOMETRY_DESCRIPTOR = [MTLAccelerationStructureTriangleGeometryDescriptor class];
      MTL_ACCELERATION_STRUCTURE_MOTION_BOUNDING_BOX_GEOMETRY_DESCRIPTOR = [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor self];
      
      MTL_VERTEX_DESCRIPTOR = [MTLVertexDescriptor class];
      
      MTL_VERTEX_BUFFER_LAYOUT_DESCRIPTOR = [MTLVertexBufferLayoutDescriptor class];
      MTL_VERTEX_ATTRIBUTE_DESCRIPTOR = [MTLVertexAttributeDescriptor class];
      
      MTL_PIPELINE_BUFFER_DESCRIPTOR = [MTLPipelineBufferDescriptor class];

      initialized = 1;
    }
  
}


NS_ASSUME_NONNULL_END
