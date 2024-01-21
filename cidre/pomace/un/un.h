//
//  un.h
//  un
//
//  Created by Yury Korolev on 1/21/24.
//

#import <Foundation/Foundation.h>
#import <UserNotifications/UserNotifications.h>

NS_ASSUME_NONNULL_BEGIN

Class UN_NOTIFICATION_SOUND;
Class UN_NOTIFICATION_CONTENT;
Class UN_MUTABLE_NOTIFICATION_CONTENT;
Class UN_NOTIFICATION_REQUEST;
Class UN_NOTIFICATION_CATEGORY;
Class UN_USER_NOTIFICATION_CENTER;

__attribute__((constructor))
static void un_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        UN_NOTIFICATION_CONTENT = [UNNotificationContent class];
        UN_MUTABLE_NOTIFICATION_CONTENT = [UNMutableNotificationContent class];
        UN_NOTIFICATION_REQUEST = [UNNotificationRequest class];
#if TARGET_OS_TV
#else
        UN_NOTIFICATION_SOUND = [UNNotificationSound class];
        UN_NOTIFICATION_CATEGORY = [UNNotificationCategory class];
#endif
        UN_USER_NOTIFICATION_CENTER = [UNUserNotificationCenter class];
    }
}

NS_ASSUME_NONNULL_END
