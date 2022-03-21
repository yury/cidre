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
pub use compute_pipeline::Descriptor as ComputePipelineDescr;
pub use compute_pipeline::State as ComputePipelineState;

pub mod command_buffer;
pub use command_buffer::CommandBuffer;
pub use command_buffer::Error as CommandBufferError;
pub use command_buffer::Status as CommandBufferStatus;

#[macro_export]
macro_rules! define_mtl {
    (device) => {
        #[inline]
        pub fn device(&self) -> &crate::mtl::Device {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn rsel_device(id: &Id) -> &crate::mtl::Device;
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
                    id: &Id,
                ) -> Option<crate::cf::Retained<'copy, crate::cf::String>>;
            }
            unsafe { copy_rsel_label(self) }
        }

        #[inline]
        pub fn set_label(&mut self, value: Option<&crate::cf::String>) {
            #[link(name = "mtl", kind = "static")]
            extern "C" {
                fn wsel_setLabel(id: &mut Id, value: Option<&crate::cf::String>);
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
                  id: &Id,
              ) -> Option<&crate::cf::String>;
          }
          unsafe { get_rsel_label(self) }
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
