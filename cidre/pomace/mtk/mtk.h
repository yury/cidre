//
//  mtk.h
//  mtk
//
//  Created by Yury Korolev on 10/28/24.
//

#import <MetalKit/MetalKit.h>

NS_ASSUME_NONNULL_BEGIN

Class MTK_TEXTURE_LOADER;

__attribute__((constructor))
static void mtk_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        MTK_TEXTURE_LOADER = [MTKTextureLoader class];
    }
}


NS_ASSUME_NONNULL_END
