//
//  wc.h
//  wc
//
//  Created by Yury Korolev on 2/11/24.
//

#import <WatchConnectivity/WatchConnectivity.h>

NS_ASSUME_NONNULL_BEGIN

Class WC_SESSION;
Class WC_SESSION_FILE;
Class WC_SESSION_FILE_TRANSFER;
Class WC_SESSION_USER_INFO_TRANSFER;

__attribute__((constructor))
static void cl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        WC_SESSION = [WCSession class];
        WC_SESSION_FILE = [WCSessionFile class];
        WC_SESSION_FILE_TRANSFER = [WCSessionFileTransfer class];
        WC_SESSION_USER_INFO_TRANSFER = [WCSessionUserInfoTransfer class];
    }
}

NS_ASSUME_NONNULL_END
