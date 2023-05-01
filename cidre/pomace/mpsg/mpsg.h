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

Class MPS_GRAPH_DEVICE;
Class MPS_GRAPH;
Class MPS_GRAPH_CONVOLUTION_2D_OP_DESCRIPTOR;

__attribute__((constructor))
static void mtl_initializer(void)
{
  static int initialized = 0;
  if (!initialized) {
    
    MPS_GRAPH_DEVICE = [MPSGraphDevice class];
    MPS_GRAPH = [MPSGraph class];
    MPS_GRAPH_CONVOLUTION_2D_OP_DESCRIPTOR = [MPSGraphConvolution2DOpDescriptor class];
    
    initialized = 1;
  }
}

NS_ASSUME_NONNULL_END
