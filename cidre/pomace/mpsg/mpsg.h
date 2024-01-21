//
//  mpsg.h
//  mpsg
//
//  Created by Yury Korolev on 27.12.2022.
//

#import <Foundation/Foundation.h>
#if TARGET_OS_SIMULATOR
#else
#import <MetalPerformanceShadersGraph/MetalPerformanceShadersGraph.h>
#endif
//#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

Class MPS_GRAPH_DEVICE;
Class MPS_GRAPH;
Class MPS_GRAPH_CONVOLUTION_2D_OP_DESCRIPTOR;
Class MPS_GRAPH_SINGLE_GATE_RNN_DESCRIPTOR;
Class MPS_GRAPH_LSTM_DESCRIPTOR;
Class MPS_GRAPH_GRU_DESCRIPTOR;

__attribute__((constructor))
static void mpsg_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        
#if TARGET_OS_SIMULATOR
#else
        
        MPS_GRAPH_DEVICE = [MPSGraphDevice class];
        MPS_GRAPH = [MPSGraph class];
        MPS_GRAPH_CONVOLUTION_2D_OP_DESCRIPTOR = [MPSGraphConvolution2DOpDescriptor class];
        MPS_GRAPH_SINGLE_GATE_RNN_DESCRIPTOR = [MPSGraphSingleGateRNNDescriptor class];
        MPS_GRAPH_GRU_DESCRIPTOR = [MPSGraphGRUDescriptor class];
        MPS_GRAPH_LSTM_DESCRIPTOR = [MPSGraphLSTMDescriptor class];
#endif
        
        initialized = 1;
    }
}

NS_ASSUME_NONNULL_END
