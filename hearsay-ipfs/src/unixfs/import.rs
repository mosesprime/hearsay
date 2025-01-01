//! <https://github.com/ipfs/specs/blob/main/UNIXFS.md#importing>

const DEFAULT_MAX_WIDTH: usize = 174;
/// Default number of bytes per chunk. See <https://ipfs-search.readthedocs.io/en/latest/ipfs_datatypes.html#chunked-unixfs-protobuf>.
const DEFAULT_CHUNK_SIZE: usize = 262144;

/// Handles bringing data into UnixFS
pub(crate) struct Importer {
    max_width: usize,
    chunker: ChunkStrat,
    layout: LayoutStrat,
}

impl Importer {
    // TODO: impl UnixFS Importer
}

/// How imported data is to be chunked.
#[derive(Debug)]
enum ChunkStrat {
    // TODO: IDK
    Rabin,
    /// Input data is chunked into fixed size pieces.
    BySize(usize),
}

impl Default for ChunkStrat {
    fn default() -> Self {
        Self::BySize(DEFAULT_CHUNK_SIZE)
    }
}

#[derive(Debug, Default)]
enum LayoutStrat {
    #[default]
    Balanced,
    Trickle,
}
