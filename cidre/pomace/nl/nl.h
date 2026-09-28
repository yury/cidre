//
//  vn.h
//  vn
//
//  Created by Yury Korolev on 13.10.2022.
//

#import <NaturalLanguage/NaturalLanguage.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void nl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
    }
}

NS_ASSUME_NONNULL_END

