use crate::{arc, ml, ns};

#[doc(alias = "MLAllComputeDevices")]
pub fn all_compute_devices() -> arc::R<ns::Array<ml::AnyComputeDevice>> {
    arc::rar_retain(unsafe { MLAllComputeDevices() })
}

#[link(name = "CoreML", kind = "framework")]
unsafe extern "C" {
    fn MLAllComputeDevices() -> arc::Rar<ns::Array<ml::AnyComputeDevice>>;
}

#[cfg(test)]
mod tests {
    use crate::ml;
    #[test]
    fn basics() {
        let devices = ml::all_compute_devices();
        assert!(!devices.is_empty());
    }
}
