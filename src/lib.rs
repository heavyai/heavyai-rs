extern crate ordered_float; // Required by thrift
extern crate thrift;
extern crate try_from; // Required by thrift

pub mod thrift_generated;
pub use crate::thrift_generated::mapd::TMapDSyncClient;
pub use crate::thrift_generated::mapd::*;

pub mod client {
  use crate::thrift_generated::mapd::MapDSyncClient;

  use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
  use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
  };

  pub fn create(
    remote_address: &str,
  ) -> Result<
    MapDSyncClient<
      TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>,
      TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>,
    >,
    thrift::Error,
  > {
    let mut c = TTcpChannel::new();
    c.open(remote_address)?;

    let (i_chan, o_chan) = c.split()?;

    let i_prot = TBinaryInputProtocol::new(TBufferedReadTransport::new(i_chan), true);
    let o_prot = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(o_chan), true);

    Ok(MapDSyncClient::new(i_prot, o_prot))
  }
}
