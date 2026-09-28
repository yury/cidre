//
//  mtk.h
//  mtk
//
//  Created by Yury Korolev on 10/28/24.
//

#import <MetalKit/MetalKit.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void mtk_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
    }
}

NS_ASSUME_NONNULL_END
