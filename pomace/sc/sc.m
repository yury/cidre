//
//  sc.m
//  sc
//
//  Created by Yury Korolev on 17.04.2022.
//

#import "sc.h"

@implementation SidreStreamOutDelegate

- (void)stream:(SCStream *)stream didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer ofType:(SCStreamOutputType)type  API_AVAILABLE(macos(12.3)){
  
  void(*cb)(void *, SCStream *, CMSampleBufferRef, SCStreamOutputType) = _vtable[1];
  cb(_vtable[0], stream, sampleBuffer, type);
}

- (void)dealloc {
  NSLog(@"dealloc");
}

@end

@implementation SidreStreamDelegate

- (void)stream:(SCStream *)stream didStopWithError:(NSError *)error  API_AVAILABLE(macos(12.3)){
  void(*cb)(void *, SCStream *, NSError *) = _vtable[1];
  cb(_vtable[0], stream, error);
}

@end
