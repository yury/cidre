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
extern inline void Prefix ## wsel ## _ ## SEL(SelfType _self) { [_self SEL]; } \
\

#define rsel(Prefix, SelfType, SEL, ReadType) \
ReadType Prefix ## rsel ## _ ## SEL(SelfType _self) { return  [_self SEL]; } \
\

#define rsel_a(Prefix, SelfType, SEL_A, A, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A(SelfType _self, A a) { return  [_self SEL_A: a]; } \
\

#define rsel_ab(Prefix, SelfType, SEL_A, A, SEL_B, B, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B(SelfType _self, A a, B b) { return  [_self SEL_A: a SEL_B: b]; } \
\

#define rsel_abc(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, ReadType) \
extern inline ReadType Prefix ## rsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(SelfType _self, A a, B b, C c) { \
  return  [_self SEL_A: a SEL_B: b SEL_C: c]; } \
\

#define wsel_a(Prefix, SelfType, SEL_A, A) \
extern inline void Prefix ## wsel ## _ ## SEL_A(SelfType _self, A a) { [_self SEL_A: a]; } \
\

#define wsel_ab(Prefix, SelfType, SEL_A, A, SEL_B, B) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B(SelfType _self, A a, B b) { [_self SEL_A: a SEL_B: b]; } \
\

#define wsel_abc(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(SelfType _self, A a, B b, C c) { [_self SEL_A: a SEL_B: b SEL_C: c]; } \
\

#define wsel_abcd(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D) \
extern inline void Prefix ## wsel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C_ ## SEL_D(SelfType _self, A a, B b, C c, D d) { [_self SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \
\


#define rwsel(Prefix, SelfType, ReadSel, WriteSel, Type) \
rsel(Prefix, SelfType, ReadSel, Type) \
wsel_a(Prefix, SelfType, WriteSel, Type) \

#define csel(Prefix, ClassType, SEL, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL(void) { return  [ClassType SEL]; } \

#define csel_a(Prefix, ClassType, SEL_A, A, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A(A a) { return  [ClassType SEL_A: a]; } \

#define csel_ab(Prefix, ClassType, SEL_A, A, SEL_B, B, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B(A a, B b) { return  [ClassType SEL_A: a SEL_B: b]; } \

#define csel_abc(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C(A a, B b, C c) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c]; } \

#define csel_abcd(Prefix, ClassType, SEL_A, A, SEL_B, B, SEL_C, C, SEL_D, D, RetType) \
extern inline RetType Prefix ## ClassType ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_C ## _ ## SEL_D(A a, B b, C c, D d) { return  [ClassType SEL_A: a SEL_B: b SEL_C: c SEL_D: d]; } \



#define sel_ch(Prefix, SelfType, SEL_CH) \
extern inline void Prefix ## sel ## _ ## SEL_CH(SelfType _self, void * _Nonnull * _Nonnull rb) { [_self SEL_CH: ^() {\
void(*ch)(void *) = rb[0]; \
ch(rb); \
} ]; } \
\

#define sel_ch_a(Prefix, SelfType, SEL_CH, CH_A) \
extern inline void Prefix ## sel ## _ ## SEL_CH(SelfType _self, void * _Nonnull * _Nonnull rb) { [_self SEL_CH: ^(CH_A ca) {\
void(*ch)(void *, CH_A) = rb[0]; \
ch(rb, ca); \
} ]; } \
\

#define sel_a_ch_a(Prefix, SelfType, SEL_A, A, SEL_CH, CH_A) \
extern inline void Prefix ## sel ## _ ## SEL_A ## _ ## SEL_CH(SelfType _self, A a, void * _Nonnull * _Nonnull rb) { [_self SEL_A:a SEL_CH: ^(CH_A ca) {\
void(*ch)(void *, CH_A) = rb[0]; \
ch(rb, ca); \
} ]; } \
\

#define sel_ch_ab(Prefix, SelfType, SEL_CH, CH_A, CH_B) \
extern inline void Prefix ## sel ## _ ## SEL_CH(SelfType _self, void * _Nonnull * _Nonnull rb) { [_self SEL_CH: ^(CH_A ca, CH_B cb) {\
void(*ch)(void *, CH_A, CH_B) = rb[0]; \
ch(rb, ca, cb); \
} ]; } \
\

#define sel_a_ch_ab(Prefix, SelfType, SEL_A, A, SEL_CH, CH_A, CH_B) \
extern inline void Prefix ## sel ## _ ## SEL_A ## _ ## SEL_CH(SelfType _self, A a, void * _Nonnull * _Nonnull rb) { [_self SEL_A:a SEL_CH: ^(CH_A ca, CH_B cb) {\
void(*ch)(void *, CH_A, CH_B) = rb[0]; \
ch(rb, ca, cb); \
} ]; } \
\

#define sel_ab_ch_ab(Prefix, SelfType, SEL_A, A, SEL_B, B, SEL_CH, CH_A, CH_B) \
extern inline void Prefix ## sel ## _ ## SEL_A ## _ ## SEL_B ## _ ## SEL_CH(SelfType _self, A a, B b, void * _Nonnull * _Nonnull rb) { [_self SEL_A:a SEL_B:b SEL_CH:^(CH_A ca, CH_B cb) {\
void(*handler)(void *, CH_A, CH_B) = rb[0]; \
handler(rb, ca, cb); \
} ]; } \
\

#pragma mark - Common

NS_RETURNS_NOT_RETAINED
rsel(get_, id, name, NSString * _Nonnull)

NS_RETURNS_RETAINED
rsel(copy_nullable_, id, name, NSString * _Nullable)
wsel_a(nullable_, id, setName, NSString * _Nullable)

wsel(, id, reset)

//@property (readonly) NSUInteger length;
rsel(, id, length, NSUInteger)

//- (void)enqueue;
wsel(, id, enqueue)
wsel(, id, commit)
wsel(, id, waitUntilScheduled)
//- (void)waitUntilCompleted;
wsel(, id, waitUntilCompleted)

#pragma mark - MTLDevice

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

//- (void)newLibraryWithSource:(NSString *)source options:(nullable MTLCompileOptions *)options completionHandler:(MTLNewLibraryCompletionHandler)completionHandler
sel_ab_ch_ab(, id, newLibraryWithSource, NSString *, options, MTLCompileOptions * _Nullable, completionHandler, id <MTLLibrary> __nullable, NSError * __nullable)

// - (nullable id <MTLComputePipelineState>)newComputePipelineStateWithFunction:(id <MTLFunction>)computeFunction error:(__autoreleasing NSError **)error;
NS_RETURNS_RETAINED
rsel_ab(, id, newComputePipelineStateWithFunction, id<MTLFunction>, error, NSError * _Nullable * _Nullable, id<MTLComputePipelineState> _Nullable)

//- (nullable id <MTLBuffer>)newBufferWithLength:(NSUInteger)length options:(MTLResourceOptions)options;
NS_RETURNS_RETAINED
rsel_ab(, id, newBufferWithLength, NSUInteger, options, MTLResourceOptions, id<MTLBuffer> _Nullable)

NS_RETURNS_RETAINED
rsel(, id, newFence, id<MTLFence> _Nullable)

//- (nullable id <MTLEvent>)newEvent API_AVAILABLE(macos(10.14), ios(12.0));
NS_RETURNS_RETAINED
rsel(, id, newEvent, id<MTLEvent> _Nullable)

//@property (readonly) NSUInteger maxBufferLength API_AVAILABLE(macos(10.14), ios(12.0));
rsel(, id, maxBufferLength, NSUInteger)

//- (nullable id <MTLSharedEvent>)newSharedEvent API_AVAILABLE(macos(10.14), ios(12.0));
NS_RETURNS_RETAINED
rsel(, id, newSharedEvent, id<MTLSharedEvent> _Nullable)

#pragma mark - CompileOptions

NS_RETURNS_RETAINED csel(, MTLCompileOptions, new, MTLCompileOptions *)

//@property (readwrite, nonatomic) BOOL fastMathEnabled;
rwsel(, id, fastMathEnabled, setFastMathEnabled, BOOL)
//@property (readwrite, nonatomic) MTLLanguageVersion languageVersion
rwsel(, id, languageVersion, setLanguageVersion, MTLLanguageVersion)

// Shared

NS_RETURNS_RETAINED
rsel(copy_, id, label, NSString * _Nullable)

NS_RETURNS_NOT_RETAINED
rsel(get_, id, label, NSString * _Nullable)


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

#pragma mark - MTLCommandBuffer

// - (void)addScheduledHandler:(MTLCommandBufferHandler)block;
sel_ch_a(, id<MTLCommandBuffer>, addScheduledHandler, id <MTLCommandBuffer>)
// - (void)addCompletedHandler:(MTLCommandBufferHandler)block;
sel_ch_a(, id<MTLCommandBuffer>, addCompletedHandler, id <MTLCommandBuffer>)

//- (nullable id <MTLBlitCommandEncoder>)blitCommandEncoder;

NS_RETURNS_RETAINED
rsel(, id, blitCommandEncoder, id <MTLBlitCommandEncoder> _Nullable)

//- (nullable id <MTLComputeCommandEncoder>)computeCommandEncoder;
NS_RETURNS_RETAINED
rsel(, id, computeCommandEncoder, id <MTLComputeCommandEncoder> _Nullable)

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


#pragma mark - MTLArgumentsEncoder
//@property (readonly) NSUInteger encodedLength;
rsel(, id, encodedLength, NSUInteger)

//@property (readonly) NSUInteger alignment;
rsel(, id, alignment, NSUInteger)

//@property (readonly) MTLFunctionOptions options API_AVAILABLE(macos(11.0), ios(14.0));
rsel(, id<MTLFunction>, options, MTLFunctionOptions)

//- (void)setArgumentBuffer:(nullable id <MTLBuffer>)argumentBuffer offset:(NSUInteger)offset;
wsel_ab(, id, setArgumentBuffer, id<MTLBuffer> _Nullable, offset, NSUInteger)
//- (void)setTexture:(nullable id <MTLTexture>)texture atIndex:(NSUInteger)index;
wsel_ab(, id, setTexture, id <MTLTexture> _Nullable, atIndex, NSUInteger)

// FunctionDescriptor

//+ (nonnull MTLFunctionDescriptor *)functionDescriptor;
NS_RETURNS_NOT_RETAINED csel(, MTLFunctionDescriptor, functionDescriptor, MTLFunctionDescriptor *)

#pragma mark - MTLRenderPipelineColorAttachmentDescriptor

// @property (nonatomic, getter = isBlendingEnabled) BOOL blendingEnabled;
rwsel(, id, isBlendingEnabled, setBlendingEnabled, BOOL)
//@property (nonatomic) MTLBlendFactor sourceRGBBlendFactor;
rwsel(, id, sourceRGBBlendFactor, setSourceRGBBlendFactor, MTLBlendFactor)
//@property (nonatomic) MTLBlendFactor destinationRGBBlendFactor;
rwsel(, id, destinationRGBBlendFactor, setDestinationRGBBlendFactor, MTLBlendFactor)
// @property (nonatomic) MTLBlendOperation rgbBlendOperation;
rwsel(, id, rgbBlendOperation, setRgbBlendOperation, MTLBlendOperation)
// @property (nonatomic) MTLBlendFactor sourceAlphaBlendFactor;
rwsel(, id, sourceAlphaBlendFactor, setSourceAlphaBlendFactor, MTLBlendFactor)
// @property (nonatomic) MTLBlendFactor destinationAlphaBlendFactor;
rwsel(, id, destinationAlphaBlendFactor, setDestinationAlphaBlendFactor, MTLBlendFactor)
// @property (nonatomic) MTLBlendOperation alphaBlendOperation;
rwsel(, id, alphaBlendOperation, setAlphaBlendOperation, MTLBlendOperation)
// @property (nonatomic) MTLColorWriteMask writeMask;
rwsel(render_pipeline_, MTLRenderPipelineColorAttachmentDescriptor *, writeMask, setWriteMask, MTLColorWriteMask)


#pragma mark - MTLRenderPipelineReflection

//@property (nullable, readonly) NSArray <MTLArgument *> *vertexArguments;
NS_RETURNS_NOT_RETAINED
rsel(, id, vertexArguments, NSArray <MTLArgument *> * _Nullable)
//@property (nullable, readonly) NSArray <MTLArgument *> *fragmentArguments;
NS_RETURNS_NOT_RETAINED
rsel(, id, fragmentArguments, NSArray <MTLArgument *> * _Nullable)
// @property (nullable, readonly) NSArray <MTLArgument *> *tileArguments;
NS_RETURNS_NOT_RETAINED
rsel(, id, tileArguments, NSArray <MTLArgument *> * _Nullable)

#pragma mark - MTLRenderPipelineDescriptor

NS_RETURNS_RETAINED
csel(, MTLRenderPipelineDescriptor, new, MTLRenderPipelineDescriptor *)

// @property (nullable, readwrite, nonatomic, strong) id <MTLFunction> vertexFunction;
NS_RETURNS_NOT_RETAINED
rwsel(, id, vertexFunction, setVertexFunction, id <MTLFunction> _Nullable)
// @property (nullable, readwrite, nonatomic, strong) id <MTLFunction> fragmentFunction;
NS_RETURNS_NOT_RETAINED
rwsel(, id, fragmentFunction, setFragmentFunction, id <MTLFunction> _Nullable)
// @property (nullable, copy, nonatomic) MTLVertexDescriptor *vertexDescriptor;

//@property (readwrite, nonatomic) NSUInteger rasterSampleCount;
rwsel(, id, rasterSampleCount, setRasterSampleCount, NSUInteger)


#pragma mark - MTLRenderPipelineState

//@property (readonly) NSUInteger maxTotalThreadsPerThreadgroup API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));
rwsel(, id, maxTotalThreadsPerThreadgroup, setMaxTotalThreadsPerThreadgroup, NSUInteger)
//@property (readonly) BOOL threadgroupSizeMatchesTileSize API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));
rsel(, id, threadgroupSizeMatchesTileSize, BOOL)
//@property (readonly) NSUInteger imageblockSampleLength API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(11.0), tvos(14.5));
rsel(, id, imageblockSampleLength, NSUInteger)

#pragma mark - MTLComputePipelineDescriptor

NS_RETURNS_RETAINED
csel(, MTLComputePipelineDescriptor, new, MTLComputePipelineDescriptor *)

//@property (readwrite, nonatomic) BOOL threadGroupSizeIsMultipleOfThreadExecutionWidth;
rwsel(, id, threadGroupSizeIsMultipleOfThreadExecutionWidth, setThreadGroupSizeIsMultipleOfThreadExecutionWidth, BOOL)

//@property (nullable, readwrite, nonatomic, strong) id <MTLFunction> computeFunction;
rwsel(, id, computeFunction, setComputeFunction, id <MTLFunction> _Nullable)

#pragma mark - MTLComputePipelineState

//@property (readonly) NSUInteger maxTotalThreadsPerThreadgroup;
//@property (readonly) NSUInteger threadExecutionWidth;
rsel(, id, threadExecutionWidth, NSUInteger)
// @property (readonly) NSUInteger staticThreadgroupMemoryLength API_AVAILABLE(macos(10.13), ios(11.0));
rsel(, id, staticThreadgroupMemoryLength, NSUInteger)

#pragma mark - MTLCommandEncoder

wsel(, id, endEncoding)
wsel_a(, id, insertDebugSignpost, NSString *)
wsel_a(, id, pushDebugGroup, NSString *)
wsel(, id, popDebugGroup)

#pragma mark - MTLRenderCommandEncoder

//- (void)setRenderPipelineState:(id <MTLRenderPipelineState>)pipelineState;
wsel_a(, id, setRenderPipelineState, id <MTLRenderPipelineState>)
//- (void)setVertexBytes:(const void *)bytes length:(NSUInteger)length atIndex:(NSUInteger)index API_AVAILABLE(macos(10.11), ios(8.3));
wsel_abc(, id, setVertexBytes, const void *, length, NSUInteger, atIndex, NSUInteger)
//- (void)setVertexBuffer:(nullable id <MTLBuffer>)buffer offset:(NSUInteger)offset atIndex:(NSUInteger)index;
wsel_abc(, id, setVertexBuffer, id<MTLBuffer> _Nullable, offset, NSUInteger, atIndex, NSUInteger)
//- (void)setFragmentBuffer:(nullable id <MTLBuffer>)buffer offset:(NSUInteger)offset atIndex:(NSUInteger)index
wsel_abc(, id, setFragmentBuffer, id <MTLBuffer> _Nullable, offset, NSUInteger, atIndex, NSUInteger)
//- (void)useResource:(id <MTLResource>)resource usage:(MTLResourceUsage)usage API_AVAILABLE(macos(10.13), ios(11.0));
wsel_ab(, id, useResource, id <MTLResource>, usage, MTLResourceUsage)
//- (void)useResources:(const id <MTLResource> __nonnull[__nonnull])resources count:(NSUInteger)count usage:(MTLResourceUsage)usage API_AVAILABLE(macos(10.13), ios(11.0));
wsel_abc(, id, useResources, const id <MTLResource> _Nonnull * _Nonnull, count, NSUInteger, usage, MTLResourceUsage)
//- (void)useHeap:(id <MTLHeap>)heap API_AVAILABLE(macos(10.13), ios(11.0));
wsel_a(, id, useHeap, id <MTLHeap> _Nonnull)

//- (void)drawPrimitives:(MTLPrimitiveType)primitiveType vertexStart:(NSUInteger)vertexStart vertexCount:(NSUInteger)vertexCount instanceCount:(NSUInteger)instanceCount;
wsel_abcd(ic_, id, drawPrimitives, MTLPrimitiveType, vertexStart, NSUInteger, vertexCount, NSUInteger, instanceCount, NSUInteger)
//- (void)drawPrimitives:(MTLPrimitiveType)primitiveType vertexStart:(NSUInteger)vertexStart vertexCount:(NSUInteger)vertexCount;
wsel_abc(, id, drawPrimitives, MTLPrimitiveType, vertexStart, NSUInteger, vertexCount, NSUInteger)



#pragma mark - MTLParallelRenderCommandEncoder

//- (nullable id <MTLRenderCommandEncoder>)renderCommandEncoder;
NS_RETURNS_NOT_RETAINED
rsel(, id, renderCommandEncoder, id<MTLRenderCommandEncoder> _Nullable)

#pragma mark - MTLBlitCommandEncoder

//- (void)updateFence:(id <MTLFence>)fence API_AVAILABLE(macos(10.13), ios(10.0));
wsel_a(, id, updateFence, id<MTLFence>)
//- (void)waitForFence:(id <MTLFence>)fence API_AVAILABLE(macos(10.13), ios(10.0));
wsel_a(, id, waitForFence, id<MTLFence>)

//- (void)fillBuffer:(id<MTLBuffer>)buffer range:(NSRange)range value:(uint8_t)value;
wsel_abc(, id, fillBuffer, id<MTLBuffer>, range, NSRange, value, uint8_t)



#pragma mark - MTLCommandQueue

//- (nullable id <MTLCommandBuffer>)commandBuffer;
NS_RETURNS_RETAINED
rsel(, id, commandBuffer, id<MTLCommandBuffer> _Nullable)

#pragma mark - MTLRenderPassDescriptor

NS_RETURNS_RETAINED
csel(, MTLRenderPassDescriptor, new, MTLRenderPassDescriptor *)

NS_RETURNS_NOT_RETAINED
csel(, MTLRenderPassDescriptor, renderPassDescriptor, MTLRenderPassDescriptor *)

//@property (readonly) MTLRenderPassColorAttachmentDescriptorArray *colorAttachments;
NS_RETURNS_NOT_RETAINED
rsel(, id, colorAttachments, MTLRenderPassColorAttachmentDescriptorArray *)

//@property (copy, nonatomic, null_resettable) MTLRenderPassDepthAttachmentDescriptor *depthAttachment;
NS_RETURNS_NOT_RETAINED
rsel(, id, depthAttachment, MTLRenderPassDepthAttachmentDescriptor *)
wsel_a(, id, setDepthAttachment, MTLRenderPassDepthAttachmentDescriptor *)


//@property (copy, nonatomic, null_resettable) MTLRenderPassStencilAttachmentDescriptor *stencilAttachment;
NS_RETURNS_NOT_RETAINED
rsel(, id, stencilAttachment, MTLRenderPassStencilAttachmentDescriptor *)
wsel_a(, id, setStencilAttachment, MTLRenderPassStencilAttachmentDescriptor *)

//- (MTLRenderPassColorAttachmentDescriptor *)objectAtIndexedSubscript:(NSUInteger)attachmentIndex;
NS_RETURNS_NOT_RETAINED
rsel_a(, MTLRenderPassColorAttachmentDescriptorArray *, objectAtIndexedSubscript, NSUInteger, MTLRenderPassColorAttachmentDescriptor *)

//- (void)setObject:(nullable MTLRenderPassColorAttachmentDescriptor *)attachment atIndexedSubscript:(NSUInteger)attachmentIndex;
wsel_ab(, MTLRenderPassColorAttachmentDescriptorArray *, setObject, MTLRenderPassColorAttachmentDescriptor * _Nullable, atIndexedSubscript, NSUInteger)



NS_ASSUME_NONNULL_END
