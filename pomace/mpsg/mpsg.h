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

NS_RETURNS_RETAINED
rsel3(, id, constantWithData, NSData *, shape, MPSShape * _Nullable, dataType, MPSDataType, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel2(, id, constantWithScalar, double, dataType, MPSDataType, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel3(, id, constantWithScalar, double, shape, MPSShape * _Nullable, dataType, MPSDataType, MPSGraphTensor *)





#pragma mark - MPSGraphConvolutionOps

NS_RETURNS_RETAINED
csel12(, MPSGraphConvolution2DOpDescriptor, descriptorWithStrideInX, NSUInteger, strideInY, NSUInteger, dilationRateInX, NSUInteger, dilationRateInY, NSUInteger, groups, NSUInteger, paddingLeft, NSUInteger, paddingRight, NSUInteger, paddingTop, NSUInteger, paddingBottom, NSUInteger, paddingStyle, MPSGraphPaddingStyle, dataLayout, MPSGraphTensorNamedDataLayout, weightsLayout, MPSGraphTensorNamedDataLayout, MPSGraphConvolution2DOpDescriptor *)


//-(MPSGraphTensor *) convolution2DWithSourceTensor:(MPSGraphTensor *) source
//                                    weightsTensor:(MPSGraphTensor *) weights
//                                       descriptor:(MPSGraphConvolution2DOpDescriptor *) descriptor
//                                             name:(NSString * _Nullable) name


NS_RETURNS_RETAINED
rsel4(, id, convolution2DWithSourceTensor, MPSGraphTensor *, weightsTensor, MPSGraphTensor*, descriptor, MPSGraphConvolution2DOpDescriptor *, name, NSString *, MPSGraphTensor *)

#pragma mark - MPSGraphArithmeticOps

NS_RETURNS_RETAINED
rsel3(, id, additionWithPrimaryTensor, MPSGraphTensor *, secondaryTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel3(, id, multiplicationWithPrimaryTensor, MPSGraphTensor *, secondaryTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel4(, id, clampWithTensor, MPSGraphTensor *, minValueTensor, MPSGraphTensor *, maxValueTensor, MPSGraphTensor *, name, NSString *,  MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel2(, id, roundWithTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel2(, id, squareRootWithTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel3(, id, subtractionWithPrimaryTensor, MPSGraphTensor *, secondaryTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel2(, id, tanhWithTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel2(, id, erfWithTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel2(, id, cosWithTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel2(, id, sinWithTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel3(, id, divisionWithPrimaryTensor, MPSGraphTensor *, secondaryTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)



#pragma mark - MPSGraphActivationOps

NS_RETURNS_RETAINED
rsel2(, id, reLUWithTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel2(, id, sigmoidWithTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel3(, id, softMaxWithTensor, MPSGraphTensor *, axis, NSInteger, name, NSString *, MPSGraphTensor *)

#pragma mark - MPSGraphMatrixMultiplicationOps

NS_RETURNS_RETAINED
rsel3(, id, matrixMultiplicationWithPrimaryTensor, MPSGraphTensor *, secondaryTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)


#pragma mark - MPSGraphResizeOps

NS_RETURNS_RETAINED
rsel7(, id, resizeTensor, MPSGraphTensor *, size, MPSShape *, mode, MPSGraphResizeMode, centerResult, BOOL, alignCorners, BOOL, layout, MPSGraphTensorNamedDataLayout, name, NSString *, MPSGraphTensor *)

#pragma mark - MPSGraphTensorShapeOps

NS_RETURNS_RETAINED
rsel3(, id, broadcastTensor, MPSGraphTensor *, toShape, MPSShape *, name, NSString *, MPSGraphTensor *)


NS_RETURNS_RETAINED
rsel3(, id, concatTensors, NSArray *, dimension, NSInteger, name, NSString *, MPSGraphTensor *)


NS_RETURNS_RETAINED
rsel3(, id, expandDimsOfTensor, MPSGraphTensor *, axis, NSInteger, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel3(, id, expandDimsOfTensor, MPSGraphTensor *, axes, NSArray *, name, NSString *, MPSGraphTensor *)


NS_RETURNS_RETAINED
rsel3(, id, reshapeTensor, MPSGraphTensor *, withShape, MPSShape *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel4(, id, transposeTensor, MPSGraphTensor *, dimension, NSUInteger, withDimension, NSUInteger, name, NSString *, MPSGraphTensor *)

rsel3(, id, castTensor, MPSGraphTensor *, toType, MPSDataType, name, NSString *, MPSGraphTensor *)

rsel5(, id, sliceTensor, MPSGraphTensor *, dimension, NSUInteger, start, NSInteger, length, NSInteger, name, NSString *, MPSGraphTensor *)


#pragma mark - MPSGraphNormalizationOps

NS_RETURNS_RETAINED
rsel3(, id, meanOfTensor, MPSGraphTensor *, axes, NSArray *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel3(, id, varianceOfTensor, MPSGraphTensor *, axes, NSArray *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel4(, id, varianceOfTensor, MPSGraphTensor *, meanTensor, MPSGraphTensor *, axes, NSArray *, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel7(, id, normalizationWithTensor, MPSGraphTensor *, meanTensor, MPSGraphTensor *, varianceTensor, MPSGraphTensor *, gammaTensor, MPSGraphTensor *, betaTensor, MPSGraphTensor *, epsilon, float, name, NSString *, MPSGraphTensor *)

#pragma mark - MPSGatherOps

NS_RETURNS_RETAINED
rsel5(, id, gatherWithUpdatesTensor, MPSGraphTensor *, indicesTensor, MPSGraphTensor *, axis, NSUInteger, batchDimensions, NSUInteger, name, NSString *, MPSGraphTensor *)

NS_RETURNS_RETAINED
rsel4(, id, gatherAlongAxis, NSInteger, withUpdatesTensor, MPSGraphTensor *, indicesTensor, MPSGraphTensor *, name, NSString *, MPSGraphTensor *)

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
SEL sel_options;
SEL sel_setOptions;



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
    
    sel_options = @selector(options);
    sel_setOptions = @selector(setOptions:);
  }
}

NS_ASSUME_NONNULL_END
