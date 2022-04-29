//
//  common.h
//  common
//
//  Created by Yury Korolev on 29.04.2022.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

SEL sel_length;
SEL sel_name;
SEL sel_label;
SEL sel_setLabel;
SEL sel_setName;
SEL sel_height;
SEL sel_setHeight;
SEL sel_width;
SEL sel_setWidth;
SEL sel_frame;
SEL sel_setFrame;

__attribute__((constructor))
static void common_initializer()
{
    static int initialized = 0;
    if (!initialized) {
      
      sel_length = @selector(length);
      sel_name = @selector(name);
      sel_label = @selector(label);
      sel_setName = @selector(setName:);
      sel_setLabel = @selector(setLabel:);
      sel_width = @selector(width);
      sel_height = @selector(height);
      sel_setWidth = @selector(setWidth:);
      sel_setHeight = @selector(setHeight:);
      sel_frame = @selector(frame);
      sel_setFrame = @selector(setFrame:);


      initialized = 1;
    }
}

NS_ASSUME_NONNULL_END

