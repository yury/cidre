//
//  mlc.h
//  mlc
//
//  Created by Yury Korolev on 27.02.2022.
//

#import <Foundation/Foundation.h>
#import <MLCompute/MLCompute.h>

#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

Class MLC_DEVICE;
Class MLC_GRAPH;

__attribute__((constructor))
static void mlc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
      
      MLC_DEVICE = [MLCDevice class];
      MLC_GRAPH = [MLCGraph class];

      initialized = 1;
    }
}


NS_ASSUME_NONNULL_END
