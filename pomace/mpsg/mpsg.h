//
//  mpsg.h
//  mpsg
//
//  Created by Yury Korolev on 27.12.2022.
//

#import <Foundation/Foundation.h>
#import <MetalPerformanceShadersGraph/MetalPerformanceShadersGraph.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - MPSGraphDevice

NS_RETURNS_RETAINED
csel1(, MPSGraphDevice, deviceWithMTLDevice, id, MPSGraphDevice *)

rsel0(, id, metalDevice, id<MTLDevice>)

#pragma mark - MPSGraphExecutableExecutionDescriptor

rwsel(, id, scheduledHandler, setScheduledHandler, id)
rwsel(, id, completionHandler, setCompletionHandler, id)

#pragma mark - MPSGraphMemoryOps

NS_RETURNS_RETAINED
rsel3(, id, placeholderWithShape, MPSShape * _Nullable, dataType, MPSDataType, name, NSString * _Nullable, MPSGraphTensor *)

//-(MPSGraphTensor *) placeholderWithShape:(MPSShape * _Nullable) shape
//                                dataType:(MPSDataType) dataType
//                                    name:(NSString * _Nullable) name

#pragma mark - MPSGraph

NS_RETURNS_RETAINED
csel0(, MPSGraph, new, MPSGraph *)

#pragma mark - SELECTORS

SEL sel_graph;

SEL sel_type;
SEL mpsg_sel_waitUntilCompleted;
SEL sel_setWaitUntilCompleted;
SEL sel_inputTensors;
SEL sel_outputTensors;
SEL sel_controlDependencies;


SEL sel_dataType;
SEL sel_shape;

SEL sel_operation;


__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
   
    sel_graph = @selector(graph);
    
    sel_type = @selector(type);
    mpsg_sel_waitUntilCompleted = @selector(waitUntilCompleted);
    sel_setWaitUntilCompleted = @selector(setWaitUntilCompleted:);
    sel_inputTensors = @selector(inputTensors);
    sel_outputTensors = @selector(outputTensors);
    sel_controlDependencies = @selector(controlDependencies);
    
    sel_dataType = @selector(dataType);
    sel_shape = @selector(shape);
    sel_operation = @selector(operation);
  }
}

NS_ASSUME_NONNULL_END
