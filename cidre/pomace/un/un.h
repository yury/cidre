//
//  un.h
//  un
//
//  Created by Yury Korolev on 1/21/24.
//

#import <UserNotifications/UserNotifications.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void un_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

    }
}

NS_ASSUME_NONNULL_END
