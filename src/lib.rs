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
  use crate::omnisci;
  use crate::omnisci::{OmniSciSyncClient, TColumn, TOmniSciSyncClient, TQueryResult, TSessionId, TColumnData};
  use regex;
  use thrift::OrderedFloat;

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

  impl From<Vec<i64>> for TColumn {
    fn from(data: Vec<i64>) -> Self {
      let nulls: Vec<bool> = data.iter().map(|_| false).collect();
      Self::new(TColumnData::new(Some(data), None, None, None), nulls)
    }
  }

  impl From<Vec<String>> for TColumn {
    fn from(data: Vec<String>) -> Self {
      let nulls: Vec<bool> = data.iter().map(|_| false).collect();
      Self::new(TColumnData::new(None, None, Some(data), None), nulls)
    }
  }

  impl From<Vec<f64>> for TColumn {
    fn from(data: Vec<f64>) -> Self {
      let cdata: Vec<OrderedFloat<f64>> = data.iter().map(|&x| OrderedFloat::from(x)).collect();
      let nulls: Vec<bool> = data.iter().map(|_| false).collect();
      Self::new(TColumnData::new(None, Some(cdata), None, None), nulls)
    }
  }

  impl From<Vec<f32>> for TColumn {
    fn from(data: Vec<f32>) -> Self {
      let cdata: Vec<OrderedFloat<f64>> = data.iter().map(|&x| OrderedFloat::from(x as f64)).collect();
      let nulls: Vec<bool> = data.iter().map(|_| false).collect();
      Self::new(TColumnData::new(None, Some(cdata), None, None), nulls)
    }
  }

  fn default_i32(v: Option<i32>) -> i64 {
    match v {
      None => 0 as i64,
      Some(x) => x as i64,
    }
  }

  fn default_str(v: &Option<String>) -> String {
    match v {
      None => String::from(""),
      Some(x) => x.to_string(),
    }
  }

  impl From<Vec<Option<i32>>> for TColumn {
    fn from(data: Vec<Option<i32>>) -> Self {
      let cdata: Vec<i64> = data.iter().map(|&x| default_i32(x)).collect();
      let nulls: Vec<bool> = data.iter().map(|x| x.is_none()).collect();
      Self::new(TColumnData::new(Some(cdata), None, None, None), nulls)
    }
  }

  impl From<Vec<&Option<String>>> for TColumn {
    fn from(data: Vec<&Option<String>>) -> Self {
      let cdata: Vec<String> = data.iter().map(|x| default_str(x)).collect();
      let nulls: Vec<bool> = data.iter().map(|x| x.is_none()).collect();
      Self::new(TColumnData::new(None, None, Some(cdata), None), nulls)
    }
  }

  impl From<Vec<Option<String>>> for TColumn {
    fn from(data: Vec<Option<String>>) -> Self {
      let cdata: Vec<String> = data.iter().map(|x| default_str(x)).collect();
      let nulls: Vec<bool> = data.iter().map(|x| x.is_none()).collect();
      Self::new(TColumnData::new(None, None, Some(cdata), None), nulls)
    }
  }

  impl From<&Vec<&Option<Vec<String>>>> for TColumn {
    fn from(data: &Vec<&Option<Vec<String>>>) -> Self {
      let cdata: Vec<Box<TColumn>> = data
        .iter()
        .map(|arr| match arr {
          None => Box::new(Self::from(vec![] as Vec<String>)),
          Some(arr) => Box::new(Self::from(arr.clone())),
        })
        .collect();
      let nulls: Vec<bool> = data.iter().map(|x| x.is_none()).collect();
      TColumn::new(TColumnData::new(None, None, None, Some(cdata)), nulls)
    }
  }

  pub trait OmniSciConnection {
    // TODO consider weld.rs as a DataFrame API https://github.com/weld-project/weld/tree/master/weld
    // TODO consider postres Row for request/response API https://docs.rs/postgres/0.17.5/postgres/row/struct.Row.html

    fn disconnect(&mut self) -> thrift::Result<()>;

    // TODO nonce for sql_execute
    fn sql_execute(&mut self, query: String, column_format: bool, nonce: String) -> thrift::Result<TQueryResult>;

    fn render_vega(&mut self, widget_id: i64, vega_json: String, compression_level: i32, nonce: String) -> thrift::Result<omnisci::TRenderResult>;

    fn load_table_binary_columnar(
      &mut self,
      table_name: &String,
      cols: Vec<TColumn>,
    ) -> thrift::Result<()>;

  }

  // TODO support other i/o protocols? <IP, OP> where IP: TInputProtocol, OP: TOutputProtocol
  pub struct OmniSciBinarySyncClient {
    session: TSessionId,
    client: OmniSciSyncClient<
      TBinaryInputProtocol<TBufferedReadTransport<ReadHalf<TTcpChannel>>>,
      TBinaryOutputProtocol<TBufferedWriteTransport<WriteHalf<TTcpChannel>>>,
    >,
  }
  impl OmniSciConnection for OmniSciBinarySyncClient {
    fn disconnect(&mut self) -> thrift::Result<()> {
      self.client.disconnect(self.session.to_string())
    }

    fn sql_execute(&mut self, query: String, column_format: bool, nonce: String) -> thrift::Result<TQueryResult> {
      self.client.sql_execute(
        self.session.to_string(),
        query,
        column_format,
        nonce,
        -1,
        -1,
      )
    }

    fn render_vega(&mut self, widget_id: i64, vega_json: String, compression_level: i32, nonce: String)
      -> thrift::Result<omnisci::TRenderResult> {
        self.client.render_vega(self.session.to_string(), widget_id, vega_json, compression_level, nonce)
    }

    fn load_table_binary_columnar(
      &mut self,
      table_name: &String,
      cols: Vec<TColumn>,
    ) -> thrift::Result<()> {
      self
        .client
        .load_table_binary_columnar(self.session.to_string(), table_name.to_string(), cols)
    }
  }

  pub fn connect(
    remote_address: &str,
    user: &str,
    password: &str,
    db_name: &str,
  ) -> thrift::Result<Box<dyn OmniSciConnection>> {
    let mut c = TTcpChannel::new();
    c.open(remote_address)?;

    let (i_chan, o_chan) = c.split()?;

    let i_prot = TBinaryInputProtocol::new(TBufferedReadTransport::new(i_chan), true);
    let o_prot = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(o_chan), true);

    let mut client = OmniSciSyncClient::new(i_prot, o_prot);
    let session = client.connect(
      String::from(user),
      String::from(password),
      String::from(db_name),
    )?;

    Ok(Box::new(OmniSciBinarySyncClient { session, client }))
  }

  // Example: "omnisci://admin:HyperInteractive@omnisciserver:6274/omnisci"
  pub fn connect_url(url: &str) -> thrift::Result<Box<dyn OmniSciConnection>> {
    // TODO allow optional values in url
    let re = regex::Regex::new(
      r"omnisci://(?P<user>.*):(?P<password>.*)@(?P<host_port>.*)/(?P<database>.*)",
    )
    .unwrap();
    match re.captures(&url) {
      None => panic!("Failed to parse OmniSciDB URL"),
      Some(captures) => {
        let user = captures.get(1).unwrap().as_str();
        let password = captures.get(2).unwrap().as_str();
        let host_port = captures.get(3).unwrap().as_str();
        let database = captures.get(4).unwrap().as_str();
        connect(host_port, user, password, database)
      }
    }
  }
}
