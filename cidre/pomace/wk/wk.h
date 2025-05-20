//
//  wk.h
//  wk
//
//  Created by Yury Korolev on 11/1/23.
//

#import <WebKit/WebKit.h>

NS_ASSUME_NONNULL_BEGIN

Class WK_WEB_VIEW;
Class WK_WEB_VIEW_CONFIGURATION;
Class WK_PROCESS_POOL;
Class WK_PREFERENCES;
Class WK_USER_CONTENT_CONTROLLER;
Class WK_USER_SCRIPT;
Class WK_WEBSITE_DATA_STORE;

__attribute__((constructor))
static void wk_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        WK_WEB_VIEW = [WKWebView class];
        WK_WEB_VIEW_CONFIGURATION = [WKWebViewConfiguration class];
        WK_PROCESS_POOL = [WKProcessPool class];
        WK_PREFERENCES = [WKPreferences class];
        WK_USER_CONTENT_CONTROLLER = [WKUserContentController class];
        WK_USER_SCRIPT = [WKUserScript class];
        WK_WEBSITE_DATA_STORE = [WKWebsiteDataStore class];
    }
}

NS_ASSUME_NONNULL_END
