// Targets the Cortex-M4 processor (ARMv7-M)

use crate::spec::{RelocModel, Target};

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7m_vivo_blueos_newlibeabi::target();
    target.options.relocation_model = RelocModel::Static;
    target.options.dynamic_linking = false;
    target
}
