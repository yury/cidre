//
//  gc.h
//  gc
//
//  Created by Yury Korolev on 1/9/24.
//

#import <GameController/GameController.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void gc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

    }
}

NS_ASSUME_NONNULL_END
