// Targets the Cortex-M33 processor (Armv8-M Mainline architecture profile),
// with the Floating Point extension.

use crate::spec::{RelocModel, Target};

pub(crate) fn target() -> Target {
    let mut target = super::thumbv8m_main_vivo_blueos_newlibeabihf::target();
    target.options.relocation_model = RelocModel::Static;
    target.options.dynamic_linking = false;
    target
}
