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


Class ML_FEATURE_VALUE;
Class ML_MULTI_ARRAY;
Class ML_MODEL;
Class ML_MODEL_CONFIGURATION;
Class ML_ARRAY_BATCH_PROVIDER;

__attribute__((constructor))
static void ml_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        ML_FEATURE_VALUE = NSClassFromString(@"MLFeatureValue");
        ML_MULTI_ARRAY = NSClassFromString(@"MLMultiArray");
        ML_MODEL = NSClassFromString(@"MLModel");
        ML_MODEL_CONFIGURATION = NSClassFromString(@"MLModelConfiguration");
        ML_ARRAY_BATCH_PROVIDER = NSClassFromString(@"MLArrayBatchProvider");
    }
}


NS_ASSUME_NONNULL_END

