//
//  gc.h
//  gc
//
//  Created by Yury Korolev on 1/9/24.
//

#import <Foundation/Foundation.h>
#import <GameController/GameController.h>

NS_ASSUME_NONNULL_BEGIN

Class GC_COLOR;
Class GC_CONTROLLER;

__attribute__((constructor))
static void gc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        GC_COLOR = [GCColor class];
        GC_CONTROLLER = [GCController class];
        
        initialized = 1;
    }
}


NS_ASSUME_NONNULL_END
