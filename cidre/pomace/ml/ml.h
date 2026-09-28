//
//  ml.h
//  ml
//
//  Created by Yury Korolev on 6/23/25.
//
//
//  ca.h
//  ca
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <CoreML/CoreML.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void ml_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
    }
}

NS_ASSUME_NONNULL_END
