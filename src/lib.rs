extern crate ordered_float; // Required by thrift
pub extern crate thrift;
extern crate try_from; // Required by thrift

pub mod common;
pub mod completion_hints;
pub mod extension_functions;
#[allow(deprecated)] // lots of deprecated warnings from the thrift-generated code
pub mod omnisci;
pub mod serialized_result_set;

pub mod client {
  use crate::omnisci::OmniSciSyncClient;
  use crate::omnisci::TOmniSciSyncClient;
  use regex;

  use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
  use thrift::transport::{
    ReadHalf, TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
  };

  pub fn create(
    remote_address: &str,
  ) -> Result<
    OmniSciSyncClient<
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

    Ok(OmniSciSyncClient::new(i_prot, o_prot))
  }

  pub trait OmniSciConnection {
    fn sql_execute(&mut self, query: String) -> thrift::Result<crate::omnisci::TQueryResult>;
  }

  // TODO support other i/o protocols? <IP, OP> where IP: TInputProtocol, OP: TOutputProtocol
  pub struct OmniSciBinarySyncClient {
    session: String,
    client: OmniSciSyncClient<
      TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>,
      TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>,
    >,
  }
  
  impl OmniSciConnection for OmniSciBinarySyncClient {
    fn sql_execute(&mut self, query: String) -> thrift::Result<crate::omnisci::TQueryResult> {
      self.client.sql_execute(
        self.session.to_string(),
        query,
        false,
        "1".to_string(),
        10000,
        -1,
      )
    }
  }

  pub fn connect(
    remote_address: &str,
    user: &str,
    password: &str,
    db_name: &str,
  )
  -> Result<Box<dyn OmniSciConnection>, thrift::Error>
  {

    let mut c = TTcpChannel::new();
    c.open(remote_address)?;

    let (i_chan, o_chan) = c.split()?;

    let i_prot = TBinaryInputProtocol::new(TBufferedReadTransport::new(i_chan), true);
    let o_prot = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(o_chan), true);

    let mut client = OmniSciSyncClient::new(i_prot, o_prot);
    let session = client.connect(String::from(user), String::from(password), String::from(db_name))?;
    
    Ok(Box::new(OmniSciBinarySyncClient{session, client}))
  }

  // Example: "omnisci://admin:HyperInteractive@omnisciserver:6274/omnisci"
  pub fn connect_url(url: &str) -> Result<Box<dyn OmniSciConnection>, thrift::Error>
  {
    // TODO allow optional values in url
    let re = regex::Regex::new(r"omnisci://(?P<user>.*):(?P<password>.*)@(?P<host_port>.*)/(?P<database>.*)").unwrap();
    // let re = regex::Regex::new(r"omnisci://(.*):(.*)@(.*)/(.*)").unwrap();
    match re.captures(&url) {
      None => panic!("Failed to parse OmniSciDB URL"),
      Some(captures) => {
        let user = captures.get(1).unwrap().as_str();
        let password = captures.get(2).unwrap().as_str();
        let host_port = captures.get(3).unwrap().as_str();
        let database = captures.get(4).unwrap().as_str();
        // print!("{} {} {} {}", user, password, host_port, database);
        connect(host_port, user, password, database)
      }
    }
  }

}
