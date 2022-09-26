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
SEL sel_size;
SEL sel_setSize;
SEL sel_isEnabled;
SEL sel_setEnabled;
SEL sel_bounds;
SEL sel_invalidate;
SEL sel_isValid;
SEL sel_setDelegate;

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
      sel_size = @selector(size);
      sel_setSize = @selector(setSize:);
      sel_isEnabled = @selector(isEnabled);
      sel_setEnabled = @selector(setEnabled:);
      sel_bounds = @selector(bounds);
      sel_invalidate = @selector(invalidate);
      sel_isValid = @selector(isValid);
      sel_setDelegate = @selector(setDelegate:);

      initialized = 1;
    }
}

NS_ASSUME_NONNULL_END

