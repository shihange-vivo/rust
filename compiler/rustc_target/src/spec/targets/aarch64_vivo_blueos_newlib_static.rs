// Generic AArch64 target for bare-metal code - Floating point enabled
//
// Can be used in conjunction with the `target-feature` and
// `target-cpu` compiler flags to opt-in more hardware-specific
// features.
//
// For example, `-C target-cpu=cortex-a53`.

use crate::spec::{RelocModel, Target};

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_vivo_blueos_newlib::target();
    target.options.relocation_model = RelocModel::Static;
    target.options.dynamic_linking = false;
    target
}
