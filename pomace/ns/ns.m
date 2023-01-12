//
//  ns.m
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import "ns.h"
#import <Foundation/Foundation.h>

@implementation CidreMachPortDelegate

- (void)handleMachMessage:(void *)msg {
  void(*cb)(void *, void *) = _vtable[1];
  cb( _vtable[0], msg);
}

@end
