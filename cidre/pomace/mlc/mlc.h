//
//  mlc.h
//  mlc
//
//  Created by Yury Korolev on 27.02.2022.
//

#import <Foundation/Foundation.h>

// MLC is not supported on simulators
#if TARGET_OS_SIMULATOR
#else
#import <MLCompute/MLCompute.h>
#endif


NS_ASSUME_NONNULL_BEGIN

Class MLC_DEVICE;
Class MLC_GRAPH;
Class MLC_ACTIVATION_DESCRIPTOR;
//Class MLC_LAYER;
Class MLC_ACTIVATION_LAYER;
Class MLC_OPTIMIZER_DESCRIPTOR;
Class MLC_ADAM_OPTIMIZER;
Class MLC_ADAMW_OPTIMIZER;
Class MLC_ARITHMETIC_LAYER;
Class MLC_BATCH_NORMALIZATION_LAYER;
Class MLC_TENSOR_DESCRIPTOR;
Class MLC_TENSOR_PARAMETER;
Class MLC_TENSOR_DATA;
Class MLC_TENSOR;
Class MLC_MATMUL_DESCRIPTOR;
Class MLC_MATMUL_LAYER;
Class MLC_INFERENCE_GRAPH;
Class MLC_TRAINING_GRAPH;

__attribute__((constructor))
static void mlc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
#if TARGET_OS_SIMULATOR
#else
        MLC_DEVICE = NSClassFromString(@"MLCDevice");
        MLC_GRAPH = NSClassFromString(@"MLCGraph");
        MLC_ACTIVATION_DESCRIPTOR = NSClassFromString(@"MLCActivationDescriptor");
        //      MLC_LAYER = [MLCLayer class];
        MLC_ACTIVATION_LAYER = NSClassFromString(@"MLCActivationLayer");
        MLC_OPTIMIZER_DESCRIPTOR = NSClassFromString(@"MLCOptimizerDescriptor");
        MLC_ADAM_OPTIMIZER = NSClassFromString(@"MLCAdamOptimizer");
        MLC_ADAMW_OPTIMIZER = NSClassFromString(@"MLCAdamWOptimizer");
        MLC_ARITHMETIC_LAYER = NSClassFromString(@"MLCArithmeticLayer");
        MLC_BATCH_NORMALIZATION_LAYER = NSClassFromString(@"MLCBatchNormalizationLayer");
        MLC_TENSOR_DESCRIPTOR = NSClassFromString(@"MLCTensorDescriptor");
        MLC_TENSOR_PARAMETER = NSClassFromString(@"MLCTensorParameter");
        MLC_TENSOR_DATA = NSClassFromString(@"MLCTensorData");
        MLC_TENSOR = NSClassFromString(@"MLCTensor");
        MLC_GRAPH = NSClassFromString(@"MLCGraph");
        MLC_MATMUL_DESCRIPTOR = NSClassFromString(@"MLCMatMulDescriptor");
        MLC_MATMUL_LAYER = NSClassFromString(@"MLCMatMulLayer");
        MLC_INFERENCE_GRAPH = NSClassFromString(@"MLCInferenceGraph");
        MLC_TRAINING_GRAPH = NSClassFromString(@"MLCTrainingGraph");
#endif
    }
}


NS_ASSUME_NONNULL_END
