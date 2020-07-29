// This integration test expects OmniSciDB running with Backend TCP port running here:
const OMNISCI_DB_URL: &str = "omnisci://admin:HyperInteractive@localhost:6274/omnisci";

use chrono;
use omnisci;
use omnisci::omnisci::{TColumn};

#[test]
fn test_insert_and_query() -> Result<(), thrift::Error> {
  let table_name = format!(
    "omnisci_rs_test_{}",
    chrono::Utc::now().format("%Y%m%d_%H%M%S")
  );

  let mut con = omnisci::client::connect_url(OMNISCI_DB_URL)?;

  con.sql_execute(format!(
    "CREATE TABLE {} (date_ text, trans text, symbol text, qty int, price float, vol float);",
    table_name
  ), false)?;

  // con.sql_execute(format!(
  //   "INSERT INTO {} VALUES ('2020-10-31','BUY','RHAT',100,35.14,1.1);",
  //   table_name
  // ))?;
  // con.sql_execute(format!(
  //   "INSERT INTO {} VALUES ('2020-10-31','BUY','GOOG',100,12.14,1.1);",
  //   table_name
  // ))?;

  let data = vec!(
    TColumn::from(vec![String::from("2020-10-31"), String::from("2020-10-31")]),
    TColumn::from(vec![String::from("BUY"), String::from("BUY")]),
    TColumn::from(vec![String::from("RHAT"), String::from("GOOG")]),
    TColumn::from(vec![100, 100]),
    TColumn::from(vec![35.14, 1.1]),
    TColumn::from(vec![12.14, 1.1]),
  );
  con.load_table_binary_columnar(&table_name, data)?;

  let results = con.sql_execute(format!(
    "SELECT symbol, qty FROM {} WHERE symbol = 'GOOG'",
    table_name
  ), false)?;

  assert_eq!(results.row_set.unwrap().rows.unwrap().len(), 1);
  // assert_eq!(results.row_set.unwrap().columns.unwrap()[0].data.unwrap().str_col.unwrap().len(), 1);

  con.sql_execute(format!("DROP TABLE {};", table_name), false)?;

  Ok(())
}
