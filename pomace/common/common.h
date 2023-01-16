//
//  common.h
//  common
//
//  Created by Yury Korolev on 29.04.2022.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN


SEL sel_setDelegate;


__attribute__((constructor))
static void common_initializer()
{
    static int initialized = 0;
    if (!initialized) {
      
      sel_setDelegate = @selector(setDelegate:);

      initialized = 1;
    }
}

NS_ASSUME_NONNULL_END

