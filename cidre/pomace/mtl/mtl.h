//
//  mtl.h
//  mtl
//
//  Created by Yury Korolev on 27.02.2022.
//

#import <Metal/Metal.h>

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
Class MTL_INDIRECT_COMMAND_BUFFER_DESCRIPTOR;


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

Class MTL_TILE_RENDER_PIPELINE_DESCRIPTOR;

Class MTL_CAPTURE_DESCRIPTOR;
Class MTL_CAPTURE_MANAGER;

Class MTL_FUNCTION_STITCHING_GRAPH;
Class MTL_FUNCTION_STITCHING_INPUT_NODE;
Class MTL_FUNCTION_STITCHING_FUNCTION_NODE;
Class MTL_FUNCTION_STITCHED_LIBRARY_DESCRIPTOR;
Class MTL_FUNCTION_STITCHING_ATTRIBUTE_ALWAYS_INLINE;
Class MTL_RESIDENCY_SET_DESCRIPTOR;

Class MTL_SHARED_EVENT_LISTENER;

Class MTL4_ARGUMENT_TABLE_DESCRIPTOR;
Class MTL4_COMMAND_ALLOCATOR_DESCRIPTOR;
Class MTL4_COUNTER_HEAP_DESCRIPTOR;
Class MTL4_COMMIT_OPTIONS;
Class MTL4_COMMAND_BUFFER_OPTIONS;
Class MTL4_MACHINE_LEARNING_PIPELINE_DESCRIPTOR;
Class MTL4_RENDER_PASS_DESCRIPTOR;
Class MTL4_PIPELINE_OPTIONS;





__attribute__((constructor))
static void mtl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

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
        MTL_INDIRECT_COMMAND_BUFFER_DESCRIPTOR = [MTLIndirectCommandBufferDescriptor class];
        
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
        
        MTL_TILE_RENDER_PIPELINE_DESCRIPTOR = [MTLTileRenderPipelineDescriptor class];
        
        MTL_CAPTURE_DESCRIPTOR = [MTLCaptureDescriptor class];
        MTL_CAPTURE_MANAGER = [MTLCaptureManager class];
        
        MTL_FUNCTION_STITCHING_GRAPH = [MTLFunctionStitchingGraph class];
        MTL_FUNCTION_STITCHING_INPUT_NODE = [MTLFunctionStitchingInputNode class];
        MTL_FUNCTION_STITCHING_FUNCTION_NODE = [MTLFunctionStitchingFunctionNode class];
        MTL_FUNCTION_STITCHED_LIBRARY_DESCRIPTOR = [MTLStitchedLibraryDescriptor class];
        MTL_FUNCTION_STITCHING_ATTRIBUTE_ALWAYS_INLINE = [MTLFunctionStitchingAttributeAlwaysInline class];
        
        MTL_SHARED_EVENT_LISTENER = [MTLSharedEventListener class];
        
        MTL_RESIDENCY_SET_DESCRIPTOR = NSClassFromString(@"MTLResidencySetDescriptor");
        
        // Metal 4
        
        MTL4_ARGUMENT_TABLE_DESCRIPTOR = NSClassFromString(@"MTL4ArgumentTableDescriptor");
        MTL4_COMMAND_ALLOCATOR_DESCRIPTOR = NSClassFromString(@"MTL4CommandAllocatorDescriptor");
        MTL4_COUNTER_HEAP_DESCRIPTOR = NSClassFromString(@"MTL4CounterHeapDescriptor");
        MTL4_COMMIT_OPTIONS = NSClassFromString(@"MTL4CommitOptions");
        MTL4_COMMAND_BUFFER_OPTIONS = NSClassFromString(@"MTL4CommandBufferOptions");
        MTL4_MACHINE_LEARNING_PIPELINE_DESCRIPTOR = NSClassFromString(@"MTL4MachineLearningPipelineDescriptor");
        MTL4_RENDER_PASS_DESCRIPTOR = NSClassFromString(@"MTL4RenderPassDescriptor");
        MTL4_PIPELINE_OPTIONS = NSClassFromString(@"MTL4PipelineOptions");
        
        
    }
    
}


NS_ASSUME_NONNULL_END
