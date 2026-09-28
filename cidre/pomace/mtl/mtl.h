//
//  mtl.h
//  mtl
//
//  Created by Yury Korolev on 27.02.2022.
//

#import <Metal/Metal.h>

NS_ASSUME_NONNULL_BEGIN

Class MTL_VISIBLE_FUNCTION_TABLE_DESCRIPTOR;
Class MTL_INTERSECTION_FUNCTION_TABLE_DESCRIPTOR;

Class MTL_ACCELERATION_STRUCTURE_GEOMETRY_DESCRIPTOR;
Class MTL_ACCELERATION_STRUCTURE_TRIANGLE_GEOMETRY_DESCRIPTOR;
Class MTL_ACCELERATION_STRUCTURE_MOTION_BOUNDING_BOX_GEOMETRY_DESCRIPTOR;

Class MTL_RESIDENCY_SET_DESCRIPTOR;

Class MTL4_ARGUMENT_TABLE_DESCRIPTOR;
Class MTL4_COMMAND_ALLOCATOR_DESCRIPTOR;
Class MTL4_COUNTER_HEAP_DESCRIPTOR;
Class MTL4_COMMIT_OPTIONS;
Class MTL4_COMMAND_BUFFER_OPTIONS;
Class MTL4_MACHINE_LEARNING_PIPELINE_DESCRIPTOR;
Class MTL4_RENDER_PASS_DESCRIPTOR;
Class MTL4_PIPELINE_OPTIONS;
Class MTL4_COMPILER_DESCRIPTOR;
Class MTL4_LIBRARY_FUNCTION_DESCRIPTOR;
Class MTL_TENSOR_EXTENTS;
Class MTL_TENSOR_DESCRIPTOR;

__attribute__((constructor))
static void mtl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

        MTL_VISIBLE_FUNCTION_TABLE_DESCRIPTOR = [MTLVisibleFunctionTableDescriptor class];
        MTL_INTERSECTION_FUNCTION_TABLE_DESCRIPTOR = [MTLIntersectionFunctionTableDescriptor class];
        
        MTL_ACCELERATION_STRUCTURE_GEOMETRY_DESCRIPTOR = [MTLAccelerationStructureGeometryDescriptor class];
        MTL_ACCELERATION_STRUCTURE_TRIANGLE_GEOMETRY_DESCRIPTOR = [MTLAccelerationStructureTriangleGeometryDescriptor class];
        MTL_ACCELERATION_STRUCTURE_MOTION_BOUNDING_BOX_GEOMETRY_DESCRIPTOR = [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor self];

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
        MTL4_COMPILER_DESCRIPTOR =  NSClassFromString(@"MTL4CompilerDescriptor");
        MTL4_LIBRARY_FUNCTION_DESCRIPTOR = NSClassFromString(@"MTL4LibraryFunctionDescriptor");
        MTL_TENSOR_EXTENTS = NSClassFromString(@"MTLTensorExtents");
        MTL_TENSOR_DESCRIPTOR = NSClassFromString(@"MTLTensorDescriptor");

    }
    
}

NS_ASSUME_NONNULL_END
