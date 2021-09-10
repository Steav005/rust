use crate::spec::{Target, TargetOptions, PanicStrategy};

pub fn target() -> Target {
    let base = super::l4re_base::opts();

    Target {
        llvm_target: "armv7-unknown-linux-gnueabihf".to_string(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        options: TargetOptions{
            features: "+v7,+vfp3,-d32,+thumb2,-neon".to_string(),
            cpu: "generic".to_string(),
            max_atomic_width: Some(64),
            mcount: "_mcount".to_string(),

            //L4Re stuff
            crt_static_default: true,
            panic_strategy: PanicStrategy::Abort,
            crt_static_allows_dylibs: false,
            dynamic_linking: false,

            ..base
        },
    }
}
