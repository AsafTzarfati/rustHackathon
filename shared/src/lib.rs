pub mod proto {
    // The generated code is in a nested module structure based on the package name.
    // package operSystem.api.v1 -> oper_system.api.v1
    pub mod oper_system {
        pub mod api {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/oper_system.api.v1.rs"));
            }
        }
    }
}

pub mod models;

pub use models::MessageWrapper;
