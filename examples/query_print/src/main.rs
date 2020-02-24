use omnisci;
use omnisci::mapd::{TDatum, TMapDSyncClient, TRow, TRowSet};

fn datum_to_str(datum: Option<&TDatum>) -> String {
  match datum {
    Some(datum) => match datum.is_null {
      Some(true) => "NULL".to_string(),
      _ => match &datum.val {
        Some(val) => {
          if let Some(int_val) = val.int_val {
            return int_val.to_string();
          } else if let Some(real_val) = val.real_val {
            return real_val.to_string();
          } else if let Some(str_val) = &val.str_val {
            return str_val.to_string();
          } else if let Some(arr_val) = &val.arr_val {
            return format!("{:?}", arr_val.iter().map(|v| datum_to_str(Some(&*v))));
          } else {
            return "NULL".to_string();
          }
        }
        _ => "N/A".to_string(),
      },
    },
    _ => "N/A".to_string(),
  }
}

fn row_to_str(row: &TRow) -> String {
  match &row.cols {
    Some(cols) => {
      let col_strs: Vec<String> = cols.iter().map(|datum| datum_to_str(Some(datum))).collect();
      return col_strs.join(", ");
    }
    None => "[Empty row]".to_string(),
  }
}

fn row_set_to_strs(row_set: TRowSet) -> Option<Vec<String>> {
  row_set
    .rows
    .map(|rows| rows.iter().map(|row| row_to_str(row)).collect())
}

fn main() -> omnisci::thrift::Result<()> {
  println!("connecting to server on 127.0.0.1:6274...");

  let mut client = omnisci::client::create("127.0.0.1:6274")?;

  println!("connection successful");

  let session = client.connect(
    "admin".to_string(),
    "HyperInteractive".to_string(),
    "omnisci".to_string(),
  )?;

  let query = "SELECT * FROM flights_donotmodify LIMIT 5;";

  let results = client.sql_execute(
    session,
    query.to_string(),
    false,
    "1".to_string(),
    10000,
    -1,
  );

  if let Ok(results) = results {
    for row_set in results.row_set {
      match row_set_to_strs(row_set) {
        Some(row_set_strs) => {
          for row in row_set_strs {
            println!("{}", row);
          }
        }
        None => {
          println!("Empty row set");
        }
      }
    }
  } else {
    println!("No results");
  }

  Ok(())
}
