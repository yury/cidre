pub mod types;
pub use types::Origin;
pub use types::Size;

pub mod pixel_format;
pub use pixel_format::PixelFormat;

pub mod argument;
pub use argument::Access as ArgumentAccess;
pub use argument::ArrayType;
pub use argument::BaseType;
pub use argument::DataType;
pub use argument::PointerType;
pub use argument::StructMember;
pub use argument::StructType;
pub use argument::TextureReferenceType;
pub use argument::Type as ArgumentType;

pub mod resource;
pub use resource::Options;
pub use resource::Resource;

pub use resource::CPUCacheMode;
pub use resource::HazardTrackingMode;
pub use resource::StorageMode;

pub use resource::Options as ResourceOptions;
pub use resource::CPU_CACHE_MODE_MASK;
pub use resource::CPU_CACHE_MODE_SHIFT;
pub use resource::HAZARD_TRACKING_MODE_MASK;
pub use resource::HAZARD_TRACKING_MODE_SHIFT;
pub use resource::STORAGE_MODE_MASK;
pub use resource::STORAGE_MODE_SHIFT;

pub mod heap;
pub use heap::Heap;

pub mod library;
pub use library::CompileOptions;
pub use library::Error as LibraryError;
pub use library::ErrorDomain as LibraryErrorDomain;
pub use library::Function;
pub use library::LanguageVersion;
pub use library::Library;
pub use library::Type as LibraryType;

pub mod command_queue;
pub use command_queue::CommandQueue;

pub mod texture;
pub use texture::CompressionType as TextureCompressionType;
pub use texture::Descriptor as TextureDescriptor;
pub use texture::Swizzle as TextureSwizzle;
pub use texture::SwizzleChannels as TextureSwizzleChannels;
pub use texture::Texture;
pub use texture::Type as TextureType;
pub use texture::Usage as TextureUsage;

pub mod device;
pub use device::ArgumentBuffersTier;
pub use device::Device;
pub use device::ReadWriteTextureTier;

pub mod function_constant_values;
pub use function_constant_values::FunctionConstantValues;

pub mod argument_encoder;
pub use argument_encoder::ArgumentEncoder;

pub mod buffer;
pub use buffer::Buffer;

pub mod function_descriptor;
pub use function_descriptor::FunctionDescriptor;
pub use function_descriptor::FunctionOptions;

pub mod compute_pipeline;
pub use compute_pipeline::Descriptor as ComputePipelineDescriptor;
pub use compute_pipeline::Reflection as ComputePipelineReflection;
pub use compute_pipeline::State as ComputePipelineState;

pub mod command_buffer;
pub use command_buffer::CommandBuffer;
pub use command_buffer::DispatchType;
pub use command_buffer::Error as CommandBufferError;
pub use command_buffer::Status as CommandBufferStatus;

pub mod render_pipeline;
pub use render_pipeline::BlendFactor;
pub use render_pipeline::BlendOperation;
pub use render_pipeline::ColorAttachmentDescriptor as RenderPipelineColorAttachmentDescriptor;
pub use render_pipeline::ColorAttachmentDescriptorArray as RenderPipelineColorAttachmentDescriptorArray;
pub use render_pipeline::ColorWriteMask;
pub use render_pipeline::Descriptor as RenderPipelineDescriptor;
pub use render_pipeline::FunctionsDescriptor as RenderPipelineFunctionsDescriptor;
pub use render_pipeline::PrimitiveTopologyClass;
pub use render_pipeline::Reflection as RenderPipelineReflection;
pub use render_pipeline::State as RenderPipelineState;
pub use render_pipeline::TessellationControlPointIndexType;
pub use render_pipeline::TessellationFactorFormat;
pub use render_pipeline::TessellationFactorStepFunction;
pub use render_pipeline::TessellationPartitionMode;
pub use render_pipeline::TileRenderPipelineColorAttachmentDescriptor;
pub use render_pipeline::TileRenderPipelineColorAttachmentDescriptorArray;
pub use render_pipeline::TileRenderPipelineDescriptor;

pub mod fence;
pub use fence::Fence;

pub mod event;
pub use event::Event;

pub mod blit_pass;
pub use blit_pass::Descriptor as BlitPassDescriptor;
pub use blit_pass::SampleBufferAttachmentDescriptor as BlitPassSampleBufferAttachmentDescriptor;
pub use blit_pass::SampleBufferAttachmentDescriptorArray as BlitPassSampleBufferAttachmentDescriptorArray;

pub mod command_encoder;
pub use command_encoder::BarrierScope;
pub use command_encoder::CommandEncoder;
pub use command_encoder::ResourceUsage;

pub mod blit_command_encoder;
pub use blit_command_encoder::BlitCommandEncoder;
pub use blit_command_encoder::BlitOption;

pub mod render_command_encoder;
pub use render_command_encoder::CullMode;
pub use render_command_encoder::DepthClipMode;
pub use render_command_encoder::DrawIndexedPrimitivesIndirectArguments;
pub use render_command_encoder::DrawPatchIndirectArguments;
pub use render_command_encoder::DrawPrimitivesIndirectArguments;
pub use render_command_encoder::PrimitiveType;
pub use render_command_encoder::QuadTessellationFactorsHalf;
pub use render_command_encoder::RenderCommandEncoder;
pub use render_command_encoder::RenderStages;
pub use render_command_encoder::ScissorRect;
pub use render_command_encoder::TriangleFillMode;
pub use render_command_encoder::TriangleTessellationFactorsHalf;
pub use render_command_encoder::VertexAmplificationViewMapping;
pub use render_command_encoder::ViewPort;
pub use render_command_encoder::VisibilityResultMode;
pub use render_command_encoder::Winding;

pub mod compute_command_encoder;
pub use compute_command_encoder::ComputeCommandEncoder;

pub mod parallel_render_command_encoder;
pub use parallel_render_command_encoder::ParallelRenderCommandEncoder;

pub mod render_pass;
pub use render_pass::AttachmentDescriptor as RenderPassAttachmentDescriptor;
pub use render_pass::ColorAttachmentDescriptor as RenderPassColorAttachmentDescriptor;
pub use render_pass::ColorAttachmentDescriptorArray as RenderPassColorAttachmentDescriptorArray;
pub use render_pass::DepthAttachmentDescriptor as RenderPassDepthAttachmentDescriptor;
pub use render_pass::Descriptor as RenderPassDescriptor;
pub use render_pass::LoadAction;
pub use render_pass::StencilAttachmentDescriptor as RenderPassStencilAttachmentDescriptor;
pub use render_pass::StoreAction;
pub use render_pass::StoreActionOptions;

pub mod compute_pass;
pub use compute_pass::Descriptor as ComputePassDescriptor;
pub use compute_pass::SampleBufferAttachmentDescriptor as ComputePassSampleBufferAttachmentDescriptor;
pub use compute_pass::SampleBufferAttachmentDescriptorArray as ComputePassSampleBufferAttachmentDescriptorArray;

pub mod counters;
pub use counters::Counter;
pub use counters::CounterResultStatistic;
pub use counters::CounterSampleBuffer;
pub use counters::CounterSet;
pub use counters::Descriptor as CounterSampleBufferDescriptor;

pub mod indirect_command_buffer;
pub use indirect_command_buffer::Descriptor as IndirectCommandBufferDescriptor;
pub use indirect_command_buffer::ExecutionRange as IndirectCommandBufferExecutionRange;
pub use indirect_command_buffer::IndirectCommandBuffer;

pub mod sampler;
pub use sampler::AddressMode as SamplerAddressMode;
pub use sampler::BorderColor as SamplerBorderColor;
pub use sampler::Descriptor as SamplerDescriptor;
pub use sampler::MinMagFilter as SamplerMinMagFilter;
pub use sampler::MipFilter as SamplerMipFilter;
pub use sampler::State as SamplerState;


#[macro_export]
macro_rules! define_mtl {
    (device) => {
        #[inline]
        pub fn device(&self) -> &crate::mtl::Device {
            crate::msg_send!("mtl", self, sel_device)
        }
    };

    (set_label) => {
        #[inline]
        pub fn set_label(&mut self, value: Option<&crate::cf::String>) {
            crate::msg_send!("mtl", self, sel_setLabel, value)
        }
    };

    (label) => {
        #[inline]
        pub fn label(&self) -> Option<&crate::cf::String> {
            crate::msg_send!("mtl", self, sel_label)
        }
    };

    (width) => {
        #[inline]
        pub fn width(&self) -> usize {
            crate::msg_send!("mtl", self, sel_width)
        }
    };

    (set_width) => {
        #[inline]
        pub fn set_width(&mut self, value: usize) {
            crate::msg_send!("mtl", self, sel_setWidth, value)
        }
    };

    (height) => {
        #[inline]
        pub fn height(&self) -> usize {
            crate::msg_send!("mtl", self, sel_height)
        }
    };

    (set_height) => {
        #[inline]
        pub fn set_height(&mut self, value: usize) {
            crate::msg_send!("mtl", self, sel_setHeight, value)
        }
    };

    (depth) => {
        #[inline]
        pub fn depth(&self) -> usize {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn rsel_depth(
                    id: &crate::ns::Id,
                ) -> usize;
            }
            unsafe { rsel_depth(self) }
        }
    };

    (set_depth) => {
        #[inline]
        pub fn set_depth(&mut self, value: usize) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_setDepth(
                    id: &mut crate::ns::Id,
                    value: usize
                );
            }
            unsafe { wsel_setDepth(self, value) }
        }
    };

    (update_fence) => {
        #[inline]
        pub fn update_fence(&self, fence: &crate::mtl::Fence) {
            crate::msg_send!("mtl", self, sel_updateFence_a, fence)
        }
    };

    (wait_for_fence) => {
        #[inline]
        pub fn wait_for_fence(&self, fence: &crate::mtl::Fence) {
            crate::msg_send!("mtl", self, sel_waitForFence_a, fence)
        }
    };

    (use_resource) => {
        #[inline]
        pub fn use_resource(&mut self, resource: &crate::mtl::Resource, usage: crate::mtl::ResourceUsage) {
            crate::msg_send!("mtl", self, sel_useResource_usage, resource, usage)
        }
    };

    (use_resources) => {
        #[inline]
        pub fn use_resources(&mut self, resources: &[crate::mtl::Resource], usage: crate::mtl::ResourceUsage) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                static sel_useResources_count_usage: &'static crate::objc::Sel;
            }
            unsafe {
                self.wsel_abc(sel_useResources_count_usage, resources.as_ptr(), resources.len(), usage)
            }
        }
    };

    (use_heap) => {
        #[inline]
        pub fn use_heap(&mut self, heap: &crate::mtl::Heap) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_useHeap(id: &mut crate::ns::Id, heap: &crate::mtl::Heap);
            }
            unsafe { wsel_useHeap(self, heap) }
        }
    };

    (push_debug_group) => {
        #[inline]
        pub fn push_debug_group(&mut self, debug_group: &crate::cf::String) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_pushDebugGroup(id: &mut crate::ns::Id, debug_group: &crate::cf::String);
            }
            unsafe { wsel_pushDebugGroup(self, debug_group) }
        }
    };

    (pop_debug_group) => {
        #[inline]
        pub fn pop_debug_group(&mut self) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_popDebugGroup(id: &mut crate::ns::Id);
            }
            unsafe { wsel_popDebugGroup(self) }
        }
    };

    (storage_mode) => {
        #[inline]
        pub fn storage_mode(&self) -> crate::mtl::StorageMode {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn rsel_storageMode(id: &crate::ns::Id) -> crate::mtl::StorageMode;
            }
            unsafe { rsel_storageMode(self) }
        }
    };

    (cpu_cache_mode) => {
        #[inline]
        pub fn cpu_cache_mode(&self) -> crate::mtl::CPUCacheMode {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn rsel_cpuCacheMode(id: &crate::ns::Id) -> crate::mtl::CPUCacheMode;
            }
            unsafe { rsel_cpuCacheMode(self) }
        }
    };

    (hazard_tracking_mode) => {
        #[inline]
        pub fn hazard_traking_mode(&self) -> crate::mtl::HazardTrackingMode {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn rsel_hazardTrackingMode(id: &crate::ns::Id) -> crate::mtl::HazardTrackingMode;
            }
            unsafe { rsel_hazardTrackingMode(self) }
        }
    };

    (reset) => {
        #[inline]
        pub fn reset(&mut self) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_reset(id: &mut crate::ns::Id);
            }
            unsafe { wsel_reset(self) }
        }
    };

    ($first:ident, $($tail:ident),+) => {
        define_mtl!($first);
        define_mtl!($($tail),+);
    };
}
