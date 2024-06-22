//
//  ui.h
//  ui
//
//  Created by Yury Korolev on 25.05.2022.
//

#import <UIKit/UIKit.h>

NS_ASSUME_NONNULL_BEGIN

Class UI_DEVICE;
Class UI_VIEW;
Class UI_COLOR;
Class UI_RESPONDER;
Class UI_VIEW_CONTROLLER;
Class UI_IMAGE;
Class NS_TEXT_ATTACHMENT;


__attribute__((constructor))
static void ui_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        UI_DEVICE = [UIDevice class];
        UI_VIEW = [UIView class];
        UI_COLOR = [UIColor class];
        UI_RESPONDER = [UIResponder class];
        UI_VIEW_CONTROLLER = [UIViewController class];
        UI_IMAGE = [UIImage class];
        
        NS_TEXT_ATTACHMENT = [NSTextAttachment class];
    }
}

NS_ASSUME_NONNULL_END
