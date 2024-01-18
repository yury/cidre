//
//  wk.h
//  wk
//
//  Created by Yury Korolev on 11/1/23.
//

#import <Foundation/Foundation.h>
#import <WebKit/WebKit.h>

NS_ASSUME_NONNULL_BEGIN

Class WK_WEB_VIEW;

__attribute__((constructor))
static void wk_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        WK_WEB_VIEW = [WKWebView class];
        
        initialized = 1;
    }
}

NS_ASSUME_NONNULL_END
