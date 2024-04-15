//
//  vn.h
//  vn
//
//  Created by Yury Korolev on 13.10.2022.
//


#import <Foundation/Foundation.h>
#import <NaturalLanguage/NaturalLanguage.h>

NS_ASSUME_NONNULL_BEGIN

Class NL_LANGUAGE_RECOGNIZER;
Class NL_EMBEDDING;


__attribute__((constructor))
static void nl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        NL_LANGUAGE_RECOGNIZER = [NLLanguageRecognizer class];
        NL_EMBEDDING = [NLEmbedding class];
    }
}



NS_ASSUME_NONNULL_END

