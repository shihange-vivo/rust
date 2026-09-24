use crate::spec::{RelocModel, Target};

pub(crate) fn target() -> Target {
    let mut target = super::riscv32imc_vivo_blueos::target();
    target.options.relocation_model = RelocModel::Static;
    target.options.dynamic_linking = false;
    target
}
