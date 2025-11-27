pub mod oper_system {
    pub mod api {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/oper_system.api.v1.rs"));
        }
    }
}

pub mod models;
pub use models::MessageWrapper;
pub use oper_system::api::v1 as proto;
