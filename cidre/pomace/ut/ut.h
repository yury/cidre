//
//  ut.h
//  ut
//
//  Created by Yury Korolev on 12/11/23.
//

#import <Foundation/Foundation.h>
#import <UniformTypeIdentifiers/UniformTypeIdentifiers.h>

NS_ASSUME_NONNULL_BEGIN

Class UT_TYPE;

__attribute__((constructor))
static void ut_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        UT_TYPE = [UTType class];
        initialized = 1;
    }
}

NS_ASSUME_NONNULL_END

