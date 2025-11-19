//
//  un.h
//  un
//
//  Created by Yury Korolev on 1/21/24.
//

#import <UserNotifications/UserNotifications.h>

NS_ASSUME_NONNULL_BEGIN

Class UN_NOTIFICATION_SOUND;
Class UN_NOTIFICATION_CONTENT;
Class UN_MUTABLE_NOTIFICATION_CONTENT;
Class UN_NOTIFICATION_REQUEST;
Class UN_NOTIFICATION_CATEGORY;
Class UN_USER_NOTIFICATION_CENTER;
Class UN_NOTIFICATION_ACTION;
Class UN_TEXT_INPUT_NOTIFICATION_ACTION;
Class UN_NOTIFICATION_ACTION_ICON;
Class UN_NOTIFICATION_ATTACHMENT;
Class UN_NOTIFICATION_TRIGGER;
Class UN_PUSH_NOTIFICATION_TRIGGER;
Class UN_TIME_INTERVAL_NOTIFICATION_TRIGGER;
Class UN_CALENDAR_NOTIFICATION_TRIGGER;
Class UN_LOCATION_NOTIFICATION_TRIGGER;

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
        UN_NOTIFICATION_ACTION = [UNNotificationAction class];
        UN_TEXT_INPUT_NOTIFICATION_ACTION = [UNTextInputNotificationAction class];
        UN_NOTIFICATION_ATTACHMENT = [UNNotificationAttachment class];
#endif
        UN_USER_NOTIFICATION_CENTER = [UNUserNotificationCenter class];
        UN_NOTIFICATION_ACTION_ICON = [UNNotificationActionIcon class];
        UN_NOTIFICATION_TRIGGER = [UNNotificationTrigger class];
        UN_PUSH_NOTIFICATION_TRIGGER = [UNPushNotificationTrigger class];
        UN_TIME_INTERVAL_NOTIFICATION_TRIGGER = [UNTimeIntervalNotificationTrigger class];
        UN_CALENDAR_NOTIFICATION_TRIGGER = [UNCalendarNotificationTrigger class];

        UN_LOCATION_NOTIFICATION_TRIGGER = NSClassFromString(@"UNLocationNotificationTrigger");
    }
}

NS_ASSUME_NONNULL_END
