#[sided(Client)]
use starbloom_core_client::prelude::*;
use starbloom_core_shared::prelude::*;
#[sided(Client)]
use starbloom_tiles::*;

mod chunk;
#[sided(Client)]
mod client;

pub use crate::chunk::*;
#[sided(Client)]
pub use crate::client::*;

#[derive(Message, Debug)]
pub struct ChunkRequest(pub IVec2);
