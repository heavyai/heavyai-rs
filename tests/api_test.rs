// This integration test expects OmniSciDB running with Backend TCP port running here:
const OMNISCI_DB_URL: &str = "omnisci://admin:HyperInteractive@localhost:6274/omnisci";

use chrono;
use omnisci;

#[test]
fn test_insert_and_query() -> Result<(), thrift::Error> {
  let table_name = format!(
    "omnisci_rs_test_{}",
    chrono::Utc::now().format("%Y%m%d_%H%M%S")
  );

  let mut client = omnisci::client::connect_url(OMNISCI_DB_URL)?;

  client.sql_execute(format!(
    "CREATE TABLE {} (date_ text, trans text, symbol text, qty int, price float, vol float);",
    table_name
  ))?;

  client.sql_execute(format!(
    "INSERT INTO {} VALUES ('2020-10-31','BUY','RHAT',100,35.14,1.1);",
    table_name
  ))?;

  client.sql_execute(format!(
    "INSERT INTO {} VALUES ('2020-10-31','BUY','GOOG',100,12.14,1.1);",
    table_name
  ))?;

  let results = client.sql_execute(format!(
    "SELECT symbol, qty FROM {} WHERE symbol = 'GOOG'",
    table_name
  ))?;

  assert!(results.row_set.unwrap().rows.unwrap().len() == 1);

  client.sql_execute(format!("DROP TABLE {};", table_name))?;

  Ok(())
}
