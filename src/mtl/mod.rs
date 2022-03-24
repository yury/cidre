pub mod types;
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

pub mod parallel_render_command_encoder;
pub use parallel_render_command_encoder::ParallelRenderCommandEncoder;

#[macro_export]
macro_rules! define_mtl {
    (device) => {
        #[inline]
        pub fn device(&self) -> &crate::mtl::Device {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn rsel_device(id: &crate::ns::Id) -> &crate::mtl::Device;
            }
            unsafe { rsel_device(self) }
        }
    };

    (mut label) => {
        #[inline]
        pub fn label<'copy>(&self) -> Option<crate::cf::Retained<'copy, crate::cf::String>> {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn copy_rsel_label<'copy>(
                    id: &crate::ns::Id,
                ) -> Option<crate::cf::Retained<'copy, crate::cf::String>>;
            }
            unsafe { copy_rsel_label(self) }
        }

        #[inline]
        pub fn set_label(&mut self, value: Option<&crate::cf::String>) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_setLabel(id: &mut crate::ns::Id, value: Option<&crate::cf::String>);
            }
            unsafe { wsel_setLabel(self, value) }
        }
    };

    (get label) => {
        #[inline]
        pub fn label(&self) -> Option<&crate::cf::String> {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn get_rsel_label(
                    id: &crate::ns::Id,
                ) -> Option<&crate::cf::String>;
            }
            unsafe { get_rsel_label(self) }
        }
    };

    (width) => {
        #[inline]
        pub fn width(&self) -> usize {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn rsel_width(
                    id: &crate::ns::Id,
                ) -> usize;
            }
            unsafe { rsel_width(self) }
        }
    };

    (mut width) => {
        define_mtl!(width);

        #[inline]
        pub fn set_width(&mut self, value: usize) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_setWidth(
                    id: &mut crate::ns::Id,
                    value: usize
                );
            }
            unsafe { wsel_setWidth(self, value) }
        }
    };

    (height) => {
        #[inline]
        pub fn height(&self) -> usize {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn rsel_height(
                    id: &crate::ns::Id,
                ) -> usize;
            }
            unsafe { rsel_height(self) }
        }
    };

    (mut height) => {
        define_mtl!(height);

        #[inline]
        pub fn set_height(&mut self, value: usize) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_setHeight(
                    id: &mut crate::ns::Id,
                    value: usize
                );
            }
            unsafe { wsel_setHeight(self, value) }
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

    (mut depth) => {
        define_mtl!(depth);

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

    (device, mut label) => {
        define_mtl!(device);
        define_mtl!(mut label);
    };

    (device, get label) => {
        define_mtl!(device);
        define_mtl!(get label);
    };
}
