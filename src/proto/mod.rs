pub(crate) mod kiapi {
    #[allow(dead_code)]
    pub mod common {
        include!(concat!(env!("OUT_DIR"), "/kiapi.common.rs"));

        pub mod commands {
            include!(concat!(env!("OUT_DIR"), "/kiapi.common.commands.rs"));
        }

        pub mod project {
            include!(concat!(env!("OUT_DIR"), "/kiapi.common.project.rs"));
        }

        pub mod types {
            include!(concat!(env!("OUT_DIR"), "/kiapi.common.types.rs"));
        }
    }

    #[allow(dead_code)]
    pub mod board {
        include!(concat!(env!("OUT_DIR"), "/kiapi.board.rs"));

        pub mod commands {
            include!(concat!(env!("OUT_DIR"), "/kiapi.board.commands.rs"));
        }

        pub mod types {
            include!(concat!(env!("OUT_DIR"), "/kiapi.board.types.rs"));
        }
    }

    #[allow(dead_code)]
    pub mod schematic {
        pub mod types {
            include!(concat!(env!("OUT_DIR"), "/kiapi.schematic.types.rs"));
        }
    }
}
