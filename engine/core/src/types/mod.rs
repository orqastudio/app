// Domain type submodules for the orqa-engine crate.
//
// Re-exports all type modules from orqa-engine-types so that consumers can
// continue to use orqa_engine::types::* without change.

pub mod artifact {
    pub use orqa_engine_types::types::artifact::*;
}

pub mod enforcement {
    pub use orqa_engine_types::types::enforcement::*;
}

pub mod governance {
    pub use orqa_engine_types::types::governance::*;
}

pub mod health {
    pub use orqa_engine_types::types::health::*;
}

pub mod knowledge {
    pub use orqa_engine_types::types::knowledge::*;
}

pub mod lesson {
    pub use orqa_engine_types::types::lesson::*;
}

pub mod message {
    pub use orqa_engine_types::types::message::*;
}

pub mod project {
    pub use orqa_engine_types::types::project::*;
}

pub mod session {
    pub use orqa_engine_types::types::session::*;
}

pub mod settings {
    pub use orqa_engine_types::types::settings::*;
}

pub mod streaming {
    pub use orqa_engine_types::types::streaming::*;
}

pub mod workflow {
    pub use orqa_engine_types::types::workflow::*;
}
