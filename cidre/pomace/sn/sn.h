//
//  sn.h
//  sn
//
//  Created by Yury Korolev on 25.12.2022.
//

#import <SoundAnalysis/SoundAnalysis.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void sn_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
    }
}

NS_ASSUME_NONNULL_END
