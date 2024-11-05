mod types;
pub use types::Coordinate2d;
pub use types::Origin;
pub use types::Region;
pub use types::ResId;
pub use types::SamplePos;
pub use types::Size;

mod stage_input_output_descriptor;
pub use stage_input_output_descriptor::AttrFormat;
pub use stage_input_output_descriptor::IndexType;

mod allocation;
pub use allocation::Allocation;

mod acceleration_structure;
pub use acceleration_structure::AccelerationStruct;
pub use acceleration_structure::Desc as AccelerationStructDesc;
pub use acceleration_structure::GeometryDesc as AccelerationStructGeometryDesc;
pub use acceleration_structure::InstanceOpts as AccelerationStructInstanceOpts;
pub use acceleration_structure::MotionBorderMode;
pub use acceleration_structure::MotionBoundingBoxGeometryDesc;
pub use acceleration_structure::Usage as AccelerationStructUsage;

mod acceleration_structure_types;
pub use acceleration_structure_types::AxisAlignedBoundingBox;
pub use acceleration_structure_types::ComponentTransform;
pub use acceleration_structure_types::PackedF32Quaternion;
pub use acceleration_structure_types::PackedF32x3;
pub use acceleration_structure_types::PackedF32x4x3;

mod pixel_format;
pub use pixel_format::PixelFormat;

mod argument;
pub use argument::Access as ArgAccess;
pub use argument::ArrayType;
pub use argument::BaseType;
pub use argument::DType;
pub use argument::PointerType;
pub use argument::StructMember;
pub use argument::StructType;
pub use argument::TextureRefType;
pub use argument::Type as ArgType;

mod residency_set;
pub use residency_set::ResidencySet;
pub use residency_set::ResidencySetDesc;

mod resource;
pub use resource::Opts as ResOpts;
pub use resource::Res;

pub use resource::CpuCacheMode;
pub use resource::HazardTrackingMode;
pub use resource::PurgableState;
pub use resource::StorageMode;

pub use resource::CPU_CACHE_MODE_MASK;
pub use resource::CPU_CACHE_MODE_SHIFT;
pub use resource::HAZARD_TRACKING_MODE_MASK;
pub use resource::HAZARD_TRACKING_MODE_SHIFT;
pub use resource::STORAGE_MODE_MASK;
pub use resource::STORAGE_MODE_SHIFT;

mod heap;
pub use heap::Desc as HeapDesc;
pub use heap::Heap;
pub use heap::Type as HeapType;

mod library;
pub use library::Attr;
pub use library::CompileOpts;
pub use library::CompileSymbolVisibility;
pub use library::Error as LibError;
pub use library::ErrorDomain as LibErrorDomain;
pub use library::Fn;
pub use library::FnType;
pub use library::LangVersion;
pub use library::Lib;
pub use library::MathFloatingPointFns;
pub use library::MathMode;
pub use library::OptimizationLevel as LibOptimizationLevel;
pub use library::PatchType;
pub use library::Type as LibType;
pub use library::VertexAttr;

mod command_queue;
pub use command_queue::CmdQueue;

mod texture;
pub use texture::Compression as TextureCompression;
pub use texture::Desc as TextureDesc;
pub use texture::SharedTextureHandle;
pub use texture::Swizzle as TextureSwizzle;
pub use texture::SwizzleChannels as TextureSwizzleChannels;
pub use texture::Texture;
pub use texture::Type as TextureType;
pub use texture::Usage as TextureUsage;

mod device;
pub use device::ArgBufsTier;
pub use device::Device;
pub use device::GpuFamily;
pub use device::PipelineOpt;
pub use device::ReadWriteTextureTier;
pub use device::SizeAlign;
pub use device::SparsePageSize;
pub use device::Timestamp;

mod drawable;
pub use drawable::Drawable;

mod function_constant_values;
pub use function_constant_values::FnConstValues;

mod function_handle;
pub use function_handle::FnHandle;

mod function_stitching;
pub use function_stitching::FnStitchedLibDesc;
pub use function_stitching::FnStitchingAttr;
pub use function_stitching::FnStitchingAttrAlwaysInline;
pub use function_stitching::FnStitchingFnNode;
pub use function_stitching::FnStitchingGraph;
pub use function_stitching::FnStitchingInputNode;
pub use function_stitching::FnStitchingNode;

mod argument_encoder;
pub use argument_encoder::ArgEncoder;

mod buffer;
pub use buffer::Buf;

mod function_descriptor;
pub use function_descriptor::FnDesc;
pub use function_descriptor::FnOpts;

mod compute_pipeline;
pub use compute_pipeline::Desc as ComputePipelineDesc;
pub use compute_pipeline::Reflection as ComputePipelineReflection;
pub use compute_pipeline::State as ComputePipelineState;

mod command_buffer;
pub use command_buffer::CmdBuf;
pub use command_buffer::DispatchType;
pub use command_buffer::Error as CmdBufErr;
pub use command_buffer::Status as CmdBufStatus;

mod render_pipeline;
pub use render_pipeline::BlendFactor;
pub use render_pipeline::BlendOp;
pub use render_pipeline::ColorAttachDesc as RenderPipelineColorAttachDesc;
pub use render_pipeline::ColorAttachDescArray as RenderPipelineColorAttachDescArray;
pub use render_pipeline::ColorWriteMask;
pub use render_pipeline::Desc as RenderPipelineDesc;
pub use render_pipeline::FnsDesc as RenderPipelineFnsDesc;
pub use render_pipeline::MeshRenderPipelineDesc;
pub use render_pipeline::PrimitiveTopologyClass;
pub use render_pipeline::Reflection as RenderPipelineReflection;
pub use render_pipeline::State as RenderPipelineState;
pub use render_pipeline::TessellationControlPointIndexType;
pub use render_pipeline::TessellationFactorFormat;
pub use render_pipeline::TessellationFactorStepFn;
pub use render_pipeline::TessellationPartitionMode;
pub use render_pipeline::TileRenderPipelineColorAttachDesc;
pub use render_pipeline::TileRenderPipelineColorAttachDescArray;
pub use render_pipeline::TileRenderPipelineDesc;

mod fence;
pub use fence::Fence;

mod event;
pub use event::Event;
pub use event::SharedEvent;
pub use event::SharedEventHandle;
pub use event::SharedEventListener;
pub use event::SharedEventNotificationBlock;

mod blit_pass;
pub use blit_pass::Desc as BlitPassDesc;
pub use blit_pass::SampleBufAttachDesc as BlitPassSampleBufAttachDesc;
pub use blit_pass::SampleBufAttachDescArray as BlitPassSampleBufAttachDescArray;

mod command_encoder;
pub use command_encoder::BarrierScope;
pub use command_encoder::CmdEncoder;
pub use command_encoder::ResUsage;

mod blit_command_encoder;
pub use blit_command_encoder::BlitCmdEncoder;
pub use blit_command_encoder::BlitOpt;

mod render_command_encoder;
pub use render_command_encoder::CullMode;
pub use render_command_encoder::DepthClipMode;
pub use render_command_encoder::DrawIndexedPrimitivesIndirectArgs;
pub use render_command_encoder::DrawPatchIndirectArgs;
pub use render_command_encoder::DrawPrimitivesIndirectArgs;
pub use render_command_encoder::Primitive;
pub use render_command_encoder::QuadTessellationFactorsHalf;
pub use render_command_encoder::RenderCmdEncoder;
pub use render_command_encoder::RenderStages;
pub use render_command_encoder::ScissorRect;
pub use render_command_encoder::TriangleFillMode;
pub use render_command_encoder::TriangleTessellationFactorsHalf;
pub use render_command_encoder::VertexAmplificationViewMapping;
pub use render_command_encoder::ViewPort;
pub use render_command_encoder::VisibilityResultMode;
pub use render_command_encoder::Winding;

mod depth_stencil;
pub use depth_stencil::CompareFn;
pub use depth_stencil::DepthStencilDesc;
pub use depth_stencil::State as DepthStencilState;
pub use depth_stencil::StencilDesc;
pub use depth_stencil::StencilOp;

mod compute_command_encoder;
pub use compute_command_encoder::ComputeCmdEncoder;

mod parallel_render_command_encoder;
pub use parallel_render_command_encoder::ParallelRenderCmdEncoder;

mod vertex_descriptor;
pub use vertex_descriptor::Desc as VertexDesc;
pub use vertex_descriptor::VertexAttrDesc;
pub use vertex_descriptor::VertexAttrDescArray;
pub use vertex_descriptor::VertexBufLayoutDesc;
pub use vertex_descriptor::VertexBufLayoutDescArray;
pub use vertex_descriptor::VertexFormat;
pub use vertex_descriptor::VertexStepFn;

mod render_pass;
pub use render_pass::AttachDesc as RenderPassAttachDesc;
pub use render_pass::ClearColor;
pub use render_pass::ColorAttachDesc as RenderPassColorAttachDesc;
pub use render_pass::ColorAttachDescArray as RenderPassColorAttachDescArray;
pub use render_pass::DepthAttachDesc as RenderPassDepthAttachDesc;
pub use render_pass::Desc as RenderPassDesc;
pub use render_pass::LoadAction;
pub use render_pass::StencilAttachDesc as RenderPassStencilAttachDesc;
pub use render_pass::StoreAction;
pub use render_pass::StoreActionOpts;

mod compute_pass;
pub use compute_pass::Desc as ComputePassDesc;
pub use compute_pass::SampleBufAttachDesc as ComputePassSampleBufAttachDesc;
pub use compute_pass::SampleBufAttachDescArray as ComputePassSampleBufAttachDescArray;

mod counters;
pub use counters::Counter;
pub use counters::CounterResultStatistic;
pub use counters::CounterSampleBuf;
pub use counters::CounterSet;
pub use counters::Desc as CounterSampleBufDesc;

mod indirect_command_buffer;
pub use indirect_command_buffer::Desc as IndirectCmdBufDesc;
pub use indirect_command_buffer::ExecutionRange as IndirectCmBufExecutionRange;
pub use indirect_command_buffer::IndirectCmdBuf;
pub use indirect_command_buffer::IndirectCmdType;

mod indirect_command_encoder;
pub use indirect_command_encoder::IndirectComputeCmd;
pub use indirect_command_encoder::IndirectRenderCmd;

mod sampler;
pub use sampler::AddrMode as SamplerAddrMode;
pub use sampler::BorderColor as SamplerBorderColor;
pub use sampler::Desc as SamplerDesc;
pub use sampler::MinMagFilter as SamplerMinMagFilter;
pub use sampler::MipFilter as SamplerMipFilter;
pub use sampler::State as SamplerState;

mod visible_function_table;
pub use visible_function_table::Desc as VisibleFnTableDesc;
pub use visible_function_table::VisibleFnTable;

mod intersection_function_table;
pub use intersection_function_table::Desc as IntersectionFnTableDesc;
pub use intersection_function_table::IntersectionFnSignature;
pub use intersection_function_table::IntersectionFnTable;

mod pipeline;
pub use pipeline::Mutability;
pub use pipeline::PipelineBufDesc;
pub use pipeline::PipelineBufDescArray;

mod capture_manager;
pub use capture_manager::CaptureDesc;
pub use capture_manager::CaptureDst;
pub use capture_manager::CaptureManager;

#[macro_export]
macro_rules! define_mtl {
    (set_label) => {
        #[$crate::objc::msg_send(setLabel:)]
        pub fn set_label(&mut self, val: Option<&crate::ns::String>);
    };

    (width) => {
        #[$crate::objc::msg_send(width)]
        pub fn width(&self) -> usize;
    };

    (set_width) => {
        #[$crate::objc::msg_send(setWidth:)]
        pub fn set_width(&mut self, val: usize);
    };

    (height) => {
        #[$crate::objc::msg_send(height)]
        pub fn height(&self) -> usize;
    };

    (set_height) => {
        #[$crate::objc::msg_send(setHeight:)]
        pub fn set_height(&mut self, val: usize);
    };

    (depth) => {
        #[$crate::objc::msg_send(depth)]
        pub fn depth(&self) -> usize;
    };

    (set_depth) => {
        #[$crate::objc::msg_send(setDepth:)]
        pub fn set_depth(&mut self, val: usize);
    };

    (gpu_res_id) => {
        #[$crate::objc::msg_send(gpuResourceID)]
        pub fn gpu_res_id(&self) -> crate::mtl::ResId;
    };

    (update_fence) => {
        #[$crate::objc::msg_send(updateFence:)]
        pub fn update_fence(&self, fence: &crate::mtl::Fence);
    };

    (wait_for_fence) => {
        #[$crate::objc::msg_send(waitForFence:)]
        pub fn wait_for_fence(&self, fence: &crate::mtl::Fence);
    };

    (use_resource) => {
        #[$crate::objc::msg_send(useResource:usage:)]
        pub fn use_resource(&mut self, resource: &crate::mtl::Res, usage: crate::mtl::ResUsage);
    };

    (use_resources) => {
        #[$crate::objc::msg_send(useResources:count:usage:)]
        pub fn use_resources_count(&mut self, resources: *const &crate::mtl::Res, count: usize, usage: crate::mtl::ResUsage);

        #[inline]
        pub fn use_resources(&mut self, resources: &[&crate::mtl::Res], usage: crate::mtl::ResUsage) {
            self.use_resources_count(resources.as_ptr(), resources.len(), usage);
        }
    };

    (use_heap) => {
        #[$crate::objc::msg_send(useHeap:)]
        pub fn use_heap(&mut self, heap: &crate::mtl::Heap);
    };

    (push_debug_group) => {
        #[$crate::objc::msg_send(pushDebugGroup:)]
        pub fn push_debug_group(&mut self, debug_group: &crate::ns::String);
    };

    (pop_debug_group) => {
        #[$crate::objc::msg_send(popDebugGroup)]
        pub fn pop_debug_group(&mut self);
    };

    (storage_mode) => {
        #[$crate::objc::msg_send(storageMode)]
        pub fn storage_mode(&self) -> crate::mtl::StorageMode;
    };

    (set_storage_mode) => {
        #[$crate::objc::msg_send(setStorageMode:)]
        pub fn set_storage_mode(&mut self, val: crate::mtl::StorageMode);
    };

    (cpu_cache_mode) => {
        #[$crate::objc::msg_send(cpuCacheMode)]
        pub fn cpu_cache_mode(&self) -> crate::mtl::CpuCacheMode;
    };

    (set_cpu_cache_mode) => {
        #[$crate::objc::msg_send(setCpuCacheMode:)]
        pub fn set_cpu_cache_mode(&mut self, val: crate::mtl::CpuCacheMode);
    };

    (hazard_tracking_mode) => {
        #[$crate::objc::msg_send(hazardTrackingMode)]
        pub fn hazard_tracking_mode(&self) -> crate::mtl::HazardTrackingMode;
    };

    (set_hazard_tracking_mode) => {
        #[$crate::objc::msg_send(setHazardTrackingMode:)]
        pub fn set_hazard_tracking_mode(&mut self, val: crate::mtl::HazardTrackingMode);
    };

    (res_opts) => {
        #[$crate::objc::msg_send(resourceOptions)]
        pub fn res_opts(&self) -> crate::mtl::ResOpts;
    };

    (set_res_opts) => {
        #[$crate::objc::msg_send(setResourceOptions:)]
        pub fn set_res_opts(&mut self, val: crate::mtl::ResOpts);
    };

    (reset) => {
        #[$crate::objc::msg_send(reset)]
        pub fn reset(&mut self);
    };

    ($first:ident, $($tail:ident),*) => {
        define_mtl!($first);
        define_mtl!($($tail),+);
    };
}
