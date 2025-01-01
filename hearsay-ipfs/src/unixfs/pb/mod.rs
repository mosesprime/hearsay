mod unix {
    pub mod pb {
        include!(concat!(env!("OUT_DIR"), "/unixfs.pb.rs"));
    }
}

pub(crate) use unix::pb::*;
