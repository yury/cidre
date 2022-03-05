//
//  mtl.h
//  mtl
//
//  Created by Yury Korolev on 27.02.2022.
//

#import <Foundation/Foundation.h>
#import <Metal/Metal.h>
NS_ASSUME_NONNULL_BEGIN

#define wsel(Prefix, SelfType, SEL) \
void Prefix ## wsel ## _ ## SEL(SelfType _self) { [_self SEL]; } \
\

#define rsel(Prefix, SelfType, SEL, ReadType) \
ReadType Prefix ## rsel ## _ ## SEL(SelfType _self) { return  [_self SEL]; } \
\

#define rsel_a(Prefix, SelfType, SEL_A, A, ReadType) \
ReadType Prefix ## rsel ## _ ## SEL_A(SelfType _self, A a) { return  [_self SEL_A: a]; } \
\

#define rsel_ab(Prefix, SelfType, SEL_A, A, SEL_B, B, ReadType) \
ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B(SelfType _self, A a, B b) { return  [_self SEL_A: a SEL_B: b]; } \
\

#define rsel_abc(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, ReadType) \
ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(SelfType _self, A a, B b, C c) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c]; } \
\

#define wsel_a(Prefix, SelfType, SEL_A, A) \
void Prefix ## wsel ## _ ## SEL_A(SelfType _self, A a) { [_self SEL_A: a]; } \
\

#define wsel_ab(Prefix, SelfType, SEL_A, A, SEL_B, B) \
void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B(SelfType _self, A a, B b) { [_self SEL_A: a SEL_B: b]; } \
\

#define wsel_abc(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C) \
void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(SelfType _self, A a, B b, C c) { [_self SEL_A: a SEL_B: b SEL_C: c]; } \
\



#define rwsel(Prefix, SelfType, ReadSel, WriteSel, Type) \
rsel(Prefix, SelfType, ReadSel, Type) \
wsel_a(Prefix, SelfType, WriteSel, Type) \

#define csel(Prefix, ClassType, SEL, RetType) \
RetType Prefix ## ClassType ## _ ## SEL(void) { return  [ClassType SEL]; } \

#define csel_a(Prefix, ClassType, SEL_A, A, RetType) \
RetType Prefix ## ClassType ## _ ## SEL_A(A a) { return  [ClassType SEL_A: a]; } \

#define csel_ab(Prefix, ClassType, SEL_A, A, SEL_B, B, RetType) \
RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B(A a, B b) { return  [ClassType SEL_A: a SEL_B: b]; } \

#define csel_abc(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, RetType) \
RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(A a, B b, C c) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c]; } \

#define csel_abcd(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, RetType) \
RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D(A a, B b, C c, D d) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \


#pragma mark - Common

NS_RETURNS_NOT_RETAINED
rsel(, id, name, NSString * _Nonnull)

wsel(, id, reset)

#pragma mark - Device

// uint64_t registryID
rsel(, id, registryID, uint64_t);

//@property (readonly) MTLSize maxThreadsPerThreadgroup API_AVAILABLE(macos(10.11), ios(9.0));
rsel(, id, maxThreadsPerThreadgroup, MTLSize);
//@property (readonly, getter=isLowPower) BOOL lowPower API_AVAILABLE(macos(10.11), macCatalyst(13.0))
//rsel(, id, isLowPower, BOOL);

//@property (readonly) BOOL hasUnifiedMemory API_AVAILABLE(macos(10.15), ios(13.0));
rsel(, id, hasUnifiedMemory, BOOL);
//@property (readonly) MTLReadWriteTextureTier readWriteTextureSupport API_AVAILABLE(macos(10.13), ios(11.0));
rsel(, id, readWriteTextureSupport, MTLReadWriteTextureTier);
//@property (readonly) MTLArgumentBuffersTier argumentBuffersSupport API_AVAILABLE(macos(10.13), ios(11.0));
rsel(, id, argumentBuffersSupport, MTLArgumentBuffersTier);


//- (nullable id <MTLCommandQueue>)newCommandQueue;
NS_RETURNS_RETAINED
rsel(, id, newCommandQueue, id <MTLCommandQueue> _Nullable);

//- (nullable id <MTLCommandQueue>)newCommandQueueWithMaxCommandBufferCount:(NSUInteger)maxCommandBufferCount;
NS_RETURNS_RETAINED
rsel_a(, id, newCommandQueueWithMaxCommandBufferCount, NSUInteger, id <MTLCommandQueue> _Nullable )

// - (nullable id <MTLTexture>)newTextureWithDescriptor:(MTLTextureDescriptor *)descriptor;
NS_RETURNS_RETAINED
rsel_a(, id, newTextureWithDescriptor, MTLTextureDescriptor *, id <MTLTexture> _Nullable )

//- (nullable id <MTLTexture>)newTextureWithDescriptor:(MTLTextureDescriptor *)descriptor iosurface:(IOSurfaceRef)iosurface plane:(NSUInteger)plane API_AVAILABLE(macos(10.11), ios(11.0));
NS_RETURNS_RETAINED
rsel_abc(, id, newTextureWithDescriptor, MTLTextureDescriptor *, iosurface, IOSurfaceRef, plane, NSUInteger, id <MTLTexture> _Nullable)


//- (nullable id <MTLLibrary>)newDefaultLibrary;
NS_RETURNS_RETAINED
rsel(, id, newDefaultLibrary, id <MTLLibrary> _Nullable)

//- (nullable id <MTLLibrary>)newLibraryWithSource:(NSString *)source options:(nullable MTLCompileOptions *)options error:(__autoreleasing NSError **)error;
NS_RETURNS_RETAINED
rsel_abc(, id, newLibraryWithSource, NSString *, options, MTLCompileOptions * _Nullable, error, NSError * _Nullable * _Nullable, id <MTLLibrary> _Nullable)

NS_RETURNS_RETAINED csel(, MTLCompileOptions, new, MTLCompileOptions *)

//@property (readwrite, nonatomic) BOOL fastMathEnabled;
rwsel(, id, fastMathEnabled, setFastMathEnabled, BOOL)
//@property (readwrite, nonatomic) MTLLanguageVersion languageVersion
rwsel(, id, languageVersion, setLanguageVersion, MTLLanguageVersion)

// Shared

NS_RETURNS_RETAINED
rsel(, id, label, NSString * _Nullable)

wsel_a(, id, setLabel, NSString * _Nullable)

NS_RETURNS_NOT_RETAINED
rsel(, id, device, id<MTLDevice>)

//@property (readwrite, nonatomic) MTLResourceOptions resourceOptions;
rwsel(, id, resourceOptions, setResourceOptions, MTLResourceOptions)
rwsel(, id, storageMode, setStorageMode, MTLStorageMode)
rwsel(, id, cpuCacheMode, setCpuCacheMode, MTLCPUCacheMode)
rwsel(, id, hazardTrackingMode, setHazardTrackingMode, MTLHazardTrackingMode)

//@property (readwrite, nonatomic) MTLTextureType textureType;
rwsel(, id, textureType, setTextureType, MTLTextureType)
//@property (readwrite, nonatomic) MTLPixelFormat pixelFormat;
rwsel(, id, pixelFormat, setPixelFormat, MTLPixelFormat)



// TextureDescriptor

// + (MTLTextureDescriptor*)texture2DDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat width:(NSUInteger)width height:(NSUInteger)height mipmapped:(BOOL)mipmapped;

//NS_RETURNS_RETAINED
csel_abcd(, MTLTextureDescriptor, texture2DDescriptorWithPixelFormat, MTLPixelFormat, width, NSUInteger, height, NSUInteger, mipmapped, BOOL,  MTLTextureDescriptor * _Nonnull)

//+ (MTLTextureDescriptor*)textureCubeDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat size:(NSUInteger)size mipmapped:(BOOL)mipmapped;

//NS_RETURNS_RETAINED
csel_abc(, MTLTextureDescriptor, textureCubeDescriptorWithPixelFormat, MTLPixelFormat, size, NSUInteger, mipmapped, BOOL, MTLTextureDescriptor * _Nonnull)

//+ (MTLTextureDescriptor*)textureBufferDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat
//                                                          width:(NSUInteger)width
//                                                resourceOptions:(MTLResourceOptions)resourceOptions
//                                                          usage:(MTLTextureUsage)usage

//NS_RETURNS_RETAINED
csel_abcd(, MTLTextureDescriptor, textureBufferDescriptorWithPixelFormat, MTLPixelFormat, width, NSUInteger, resourceOptions, MTLResourceOptions, usage, MTLTextureUsage, MTLTextureDescriptor * _Nullable)

//@property (readwrite, nonatomic) NSUInteger width;
rwsel(, id, width, setWidth, NSUInteger)
//@property (readwrite, nonatomic) NSUInteger height;
rwsel(, id, height, setHeight, NSUInteger)
//@property (readwrite, nonatomic) NSUInteger depth;
rwsel(, id, depth, setDepth, NSUInteger)
//@property (readwrite, nonatomic) NSUInteger mipmapLevelCount;
rwsel(, id, mipmapLevelCount, setMipmapLevelCount, NSUInteger)
//@property (readonly) NSUInteger sampleCount;
rwsel(MTLTextureDescriptor_, MTLTextureDescriptor *, sampleCount, setSampleCount, NSUInteger)
//@property (readwrite, nonatomic) NSUInteger arrayLength;
rwsel(, id, arrayLength, setArrayLength, NSUInteger)

//@property (readwrite, nonatomic) MTLTextureUsage usage API_AVAILABLE(macos(10.11), ios(9.0));
rwsel(, id, usage, setUsage, MTLTextureUsage)

//@property (readwrite, nonatomic) BOOL allowGPUOptimizedContents API_AVAILABLE(macos(10.14), ios(12.0));
rwsel(, id, allowGPUOptimizedContents, setAllowGPUOptimizedContents, BOOL)

//@property (readwrite, nonatomic) MTLTextureCompressionType compressionType API_AVAILABLE(macos(12.0), ios(15.0));

// rwsel(MTLTextureDescriptor_, MTLTextureDescriptor *, compressionType, setCompressionType, MTLTextureCompressionType)

//@property (readwrite, nonatomic) MTLTextureSwizzleChannels swizzle API_AVAILABLE(macos(10.15), ios(13.0));
rwsel(, id, swizzle, setSwizzle, MTLTextureSwizzleChannels)

// MTLTexture

//@property (nullable, readonly) id <MTLTexture> parentTexture API_AVAILABLE(macos(10.11), ios(9.0));

rsel(, id, parentTexture, id<MTLTexture> _Nullable)

//- (nullable id<MTLTexture>)newTextureViewWithPixelFormat:(MTLPixelFormat)pixelFormat;
NS_RETURNS_RETAINED
rsel_a(, id, newTextureViewWithPixelFormat, MTLPixelFormat, id <MTLTexture> _Nullable )

//@property (nullable, readonly) IOSurfaceRef iosurface API_AVAILABLE(macos(10.11), ios(11.0));
CF_RETURNS_NOT_RETAINED
rsel(, id, iosurface, IOSurfaceRef _Nullable )

//@property (readonly) NSUInteger iosurfacePlane API_AVAILABLE(macos(10.11), ios(11.0));
rsel(, id, iosurfacePlane, NSUInteger)

// MTLFunctionConstantValues

NS_RETURNS_RETAINED csel(, MTLFunctionConstantValues, new, MTLFunctionConstantValues *)
//- (void)setConstantValue:(const void *)value type:(MTLDataType)type atIndex:(NSUInteger)index;
wsel_abc(, id, setConstantValue, const void *, type, MTLDataType, atIndex, NSUInteger)
//- (void)setConstantValues:(const void *)values type:(MTLDataType)type withRange:(NSRange)range;
wsel_abc(, id, setConstantValues, const void *, type, MTLDataType, withRange, NSRange)
//- (void)setConstantValue:(const void *)value type:(MTLDataType)type withName:(NSString *)name;
wsel_abc(, id, setConstantValue, const void *, type, MTLDataType, withName, NSString *)



// MTLHeap

NS_RETURNS_RETAINED csel(, MTLHeapDescriptor, new, MTLHeapDescriptor *)

rwsel(, id, size, setSize, NSUInteger)

rwsel(MTLHeapType_MTLHeapDescriptor_, MTLHeapDescriptor *, type, setType, MTLHeapType)
rsel(MTLHeapType_MTLHeap_, id<MTLHeap>, type, MTLHeapType)


rsel(MTLHeapType_MTLHeap_, id<MTLHeap>, currentAllocatedSize, NSUInteger)

rsel_a(, id, maxAvailableSizeWithAlignment, NSUInteger, NSUInteger)


NS_RETURNS_RETAINED
rsel_abc(, id, newBufferWithBytes, const void *, length,
         NSUInteger, options, MTLResourceOptions,
         id<MTLBuffer> _Nullable)

NS_RETURNS_RETAINED
rsel_a(, id, newDepthStencilStateWithDescriptor, MTLDepthStencilDescriptor * _Nonnull,
       id<MTLDepthStencilState> _Nullable)


// MTLHeap end

// MTLLibrary

NS_RETURNS_RETAINED
rsel_a(, id, newFunctionWithName, NSString *, id<MTLFunction> _Nullable)

NS_RETURNS_RETAINED
rsel_abc(, id, newFunctionWithName, NSString *, constantValues, MTLFunctionConstantValues *, error, NSError * _Nullable * _Nullable, id<MTLFunction> _Nullable)

//@property (readonly) NSArray <NSString *> *functionNames;
NS_RETURNS_NOT_RETAINED
rsel(, id, functionNames, NSArray <NSString *> *)


// MTLFunction

//- (id <MTLArgumentEncoder>)newArgumentEncoderWithBufferIndex:(NSUInteger)bufferIndex API_AVAILABLE(macos(10.13), ios(11.0));
NS_RETURNS_RETAINED
rsel_a(, id, newArgumentEncoderWithBufferIndex, NSUInteger,id <MTLArgumentEncoder>)




NS_ASSUME_NONNULL_END
