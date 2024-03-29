// Autogenerated by Thrift Compiler (0.14.2)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use thrift::OrderedFloat;
use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TExtArgumentType {
  Int8 = 0,
  Int16 = 1,
  Int32 = 2,
  Int64 = 3,
  Float = 4,
  Double = 5,
  Void = 6,
  PInt8 = 7,
  PInt16 = 8,
  PInt32 = 9,
  PInt64 = 10,
  PFloat = 11,
  PDouble = 12,
  PBool = 13,
  Bool = 14,
  ArrayInt8 = 15,
  ArrayInt16 = 16,
  ArrayInt32 = 17,
  ArrayInt64 = 18,
  ArrayFloat = 19,
  ArrayDouble = 20,
  ArrayBool = 21,
  GeoPoint = 22,
  GeoLineString = 23,
  Cursor = 24,
  GeoPolygon = 25,
  GeoMultiPolygon = 26,
  ColumnInt8 = 27,
  ColumnInt16 = 28,
  ColumnInt32 = 29,
  ColumnInt64 = 30,
  ColumnFloat = 31,
  ColumnDouble = 32,
  ColumnBool = 33,
  TextEncodingNone = 34,
  TextEncodingDict = 35,
  ColumnListInt8 = 36,
  ColumnListInt16 = 37,
  ColumnListInt32 = 38,
  ColumnListInt64 = 39,
  ColumnListFloat = 40,
  ColumnListDouble = 41,
  ColumnListBool = 42,
  ColumnTextEncodingDict = 43,
  ColumnListTextEncodingDict = 44,
}

impl TExtArgumentType {
  pub fn write_to_out_protocol(self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    o_prot.write_i32(self as i32)
  }
  pub fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<TExtArgumentType> {
    let enum_value = i_prot.read_i32()?;
    TExtArgumentType::try_from(enum_value)  }
}

impl TryFrom<i32> for TExtArgumentType {
  type Error = thrift::Error;  fn try_from(i: i32) -> Result<Self, Self::Error> {
    match i {
      0 => Ok(TExtArgumentType::Int8),
      1 => Ok(TExtArgumentType::Int16),
      2 => Ok(TExtArgumentType::Int32),
      3 => Ok(TExtArgumentType::Int64),
      4 => Ok(TExtArgumentType::Float),
      5 => Ok(TExtArgumentType::Double),
      6 => Ok(TExtArgumentType::Void),
      7 => Ok(TExtArgumentType::PInt8),
      8 => Ok(TExtArgumentType::PInt16),
      9 => Ok(TExtArgumentType::PInt32),
      10 => Ok(TExtArgumentType::PInt64),
      11 => Ok(TExtArgumentType::PFloat),
      12 => Ok(TExtArgumentType::PDouble),
      13 => Ok(TExtArgumentType::PBool),
      14 => Ok(TExtArgumentType::Bool),
      15 => Ok(TExtArgumentType::ArrayInt8),
      16 => Ok(TExtArgumentType::ArrayInt16),
      17 => Ok(TExtArgumentType::ArrayInt32),
      18 => Ok(TExtArgumentType::ArrayInt64),
      19 => Ok(TExtArgumentType::ArrayFloat),
      20 => Ok(TExtArgumentType::ArrayDouble),
      21 => Ok(TExtArgumentType::ArrayBool),
      22 => Ok(TExtArgumentType::GeoPoint),
      23 => Ok(TExtArgumentType::GeoLineString),
      24 => Ok(TExtArgumentType::Cursor),
      25 => Ok(TExtArgumentType::GeoPolygon),
      26 => Ok(TExtArgumentType::GeoMultiPolygon),
      27 => Ok(TExtArgumentType::ColumnInt8),
      28 => Ok(TExtArgumentType::ColumnInt16),
      29 => Ok(TExtArgumentType::ColumnInt32),
      30 => Ok(TExtArgumentType::ColumnInt64),
      31 => Ok(TExtArgumentType::ColumnFloat),
      32 => Ok(TExtArgumentType::ColumnDouble),
      33 => Ok(TExtArgumentType::ColumnBool),
      34 => Ok(TExtArgumentType::TextEncodingNone),
      35 => Ok(TExtArgumentType::TextEncodingDict),
      36 => Ok(TExtArgumentType::ColumnListInt8),
      37 => Ok(TExtArgumentType::ColumnListInt16),
      38 => Ok(TExtArgumentType::ColumnListInt32),
      39 => Ok(TExtArgumentType::ColumnListInt64),
      40 => Ok(TExtArgumentType::ColumnListFloat),
      41 => Ok(TExtArgumentType::ColumnListDouble),
      42 => Ok(TExtArgumentType::ColumnListBool),
      43 => Ok(TExtArgumentType::ColumnTextEncodingDict),
      44 => Ok(TExtArgumentType::ColumnListTextEncodingDict),
      _ => {
        Err(
          thrift::Error::Protocol(
            ProtocolError::new(
              ProtocolErrorKind::InvalidData,
              format!("cannot convert enum constant {} to TExtArgumentType", i)
            )
          )
        )
      },
    }
  }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TOutputBufferSizeType {
  KConstant = 0,
  KUserSpecifiedConstantParameter = 1,
  KUserSpecifiedRowMultiplier = 2,
  KTableFunctionSpecifiedParameter = 3,
}

impl TOutputBufferSizeType {
  pub fn write_to_out_protocol(self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    o_prot.write_i32(self as i32)
  }
  pub fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<TOutputBufferSizeType> {
    let enum_value = i_prot.read_i32()?;
    TOutputBufferSizeType::try_from(enum_value)  }
}

impl TryFrom<i32> for TOutputBufferSizeType {
  type Error = thrift::Error;  fn try_from(i: i32) -> Result<Self, Self::Error> {
    match i {
      0 => Ok(TOutputBufferSizeType::KConstant),
      1 => Ok(TOutputBufferSizeType::KUserSpecifiedConstantParameter),
      2 => Ok(TOutputBufferSizeType::KUserSpecifiedRowMultiplier),
      3 => Ok(TOutputBufferSizeType::KTableFunctionSpecifiedParameter),
      _ => {
        Err(
          thrift::Error::Protocol(
            ProtocolError::new(
              ProtocolErrorKind::InvalidData,
              format!("cannot convert enum constant {} to TOutputBufferSizeType", i)
            )
          )
        )
      },
    }
  }
}

//
// TUserDefinedFunction
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TUserDefinedFunction {
  pub name: Option<String>,
  pub arg_types: Option<Vec<TExtArgumentType>>,
  pub ret_type: Option<TExtArgumentType>,
}

impl TUserDefinedFunction {
  pub fn new<F1, F2, F3>(name: F1, arg_types: F2, ret_type: F3) -> TUserDefinedFunction where F1: Into<Option<String>>, F2: Into<Option<Vec<TExtArgumentType>>>, F3: Into<Option<TExtArgumentType>> {
    TUserDefinedFunction {
      name: name.into(),
      arg_types: arg_types.into(),
      ret_type: ret_type.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<TUserDefinedFunction> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = Some("".to_owned());
    let mut f_2: Option<Vec<TExtArgumentType>> = Some(Vec::new());
    let mut f_3: Option<TExtArgumentType> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<TExtArgumentType> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_0 = TExtArgumentType::read_from_in_protocol(i_prot)?;
            val.push(list_elem_0);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        3 => {
          let val = TExtArgumentType::read_from_in_protocol(i_prot)?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = TUserDefinedFunction {
      name: f_1,
      arg_types: f_2,
      ret_type: f_3,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("TUserDefinedFunction");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.name {
      o_prot.write_field_begin(&TFieldIdentifier::new("name", TType::String, 1))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.arg_types {
      o_prot.write_field_begin(&TFieldIdentifier::new("argTypes", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I32, fld_var.len() as i32))?;
      for e in fld_var {
        e.write_to_out_protocol(o_prot)?;
        o_prot.write_list_end()?;
      }
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.ret_type {
      o_prot.write_field_begin(&TFieldIdentifier::new("retType", TType::I32, 3))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Default for TUserDefinedFunction {
  fn default() -> Self {
    TUserDefinedFunction{
      name: Some("".to_owned()),
      arg_types: Some(Vec::new()),
      ret_type: None,
    }
  }
}

//
// TUserDefinedTableFunction
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TUserDefinedTableFunction {
  pub name: Option<String>,
  pub sizer_type: Option<TOutputBufferSizeType>,
  pub sizer_arg_pos: Option<i32>,
  pub input_arg_types: Option<Vec<TExtArgumentType>>,
  pub output_arg_types: Option<Vec<TExtArgumentType>>,
  pub sql_arg_types: Option<Vec<TExtArgumentType>>,
  pub annotations: Option<Vec<BTreeMap<String, String>>>,
}

impl TUserDefinedTableFunction {
  pub fn new<F1, F2, F3, F4, F5, F6, F7>(name: F1, sizer_type: F2, sizer_arg_pos: F3, input_arg_types: F4, output_arg_types: F5, sql_arg_types: F6, annotations: F7) -> TUserDefinedTableFunction where F1: Into<Option<String>>, F2: Into<Option<TOutputBufferSizeType>>, F3: Into<Option<i32>>, F4: Into<Option<Vec<TExtArgumentType>>>, F5: Into<Option<Vec<TExtArgumentType>>>, F6: Into<Option<Vec<TExtArgumentType>>>, F7: Into<Option<Vec<BTreeMap<String, String>>>> {
    TUserDefinedTableFunction {
      name: name.into(),
      sizer_type: sizer_type.into(),
      sizer_arg_pos: sizer_arg_pos.into(),
      input_arg_types: input_arg_types.into(),
      output_arg_types: output_arg_types.into(),
      sql_arg_types: sql_arg_types.into(),
      annotations: annotations.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<TUserDefinedTableFunction> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = Some("".to_owned());
    let mut f_2: Option<TOutputBufferSizeType> = None;
    let mut f_3: Option<i32> = Some(0);
    let mut f_4: Option<Vec<TExtArgumentType>> = Some(Vec::new());
    let mut f_5: Option<Vec<TExtArgumentType>> = Some(Vec::new());
    let mut f_6: Option<Vec<TExtArgumentType>> = Some(Vec::new());
    let mut f_7: Option<Vec<BTreeMap<String, String>>> = Some(Vec::new());
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = TOutputBufferSizeType::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_i32()?;
          f_3 = Some(val);
        },
        4 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<TExtArgumentType> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_1 = TExtArgumentType::read_from_in_protocol(i_prot)?;
            val.push(list_elem_1);
          }
          i_prot.read_list_end()?;
          f_4 = Some(val);
        },
        5 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<TExtArgumentType> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_2 = TExtArgumentType::read_from_in_protocol(i_prot)?;
            val.push(list_elem_2);
          }
          i_prot.read_list_end()?;
          f_5 = Some(val);
        },
        6 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<TExtArgumentType> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_3 = TExtArgumentType::read_from_in_protocol(i_prot)?;
            val.push(list_elem_3);
          }
          i_prot.read_list_end()?;
          f_6 = Some(val);
        },
        7 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<BTreeMap<String, String>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let map_ident = i_prot.read_map_begin()?;
            let mut list_elem_4: BTreeMap<String, String> = BTreeMap::new();
            for _ in 0..map_ident.size {
              let map_key_5 = i_prot.read_string()?;
              let map_val_6 = i_prot.read_string()?;
              list_elem_4.insert(map_key_5, map_val_6);
            }
            i_prot.read_map_end()?;
            val.push(list_elem_4);
          }
          i_prot.read_list_end()?;
          f_7 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = TUserDefinedTableFunction {
      name: f_1,
      sizer_type: f_2,
      sizer_arg_pos: f_3,
      input_arg_types: f_4,
      output_arg_types: f_5,
      sql_arg_types: f_6,
      annotations: f_7,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("TUserDefinedTableFunction");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.name {
      o_prot.write_field_begin(&TFieldIdentifier::new("name", TType::String, 1))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.sizer_type {
      o_prot.write_field_begin(&TFieldIdentifier::new("sizerType", TType::I32, 2))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?
    }
    if let Some(fld_var) = self.sizer_arg_pos {
      o_prot.write_field_begin(&TFieldIdentifier::new("sizerArgPos", TType::I32, 3))?;
      o_prot.write_i32(fld_var)?;
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.input_arg_types {
      o_prot.write_field_begin(&TFieldIdentifier::new("inputArgTypes", TType::List, 4))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I32, fld_var.len() as i32))?;
      for e in fld_var {
        e.write_to_out_protocol(o_prot)?;
        o_prot.write_list_end()?;
      }
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.output_arg_types {
      o_prot.write_field_begin(&TFieldIdentifier::new("outputArgTypes", TType::List, 5))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I32, fld_var.len() as i32))?;
      for e in fld_var {
        e.write_to_out_protocol(o_prot)?;
        o_prot.write_list_end()?;
      }
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.sql_arg_types {
      o_prot.write_field_begin(&TFieldIdentifier::new("sqlArgTypes", TType::List, 6))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I32, fld_var.len() as i32))?;
      for e in fld_var {
        e.write_to_out_protocol(o_prot)?;
        o_prot.write_list_end()?;
      }
      o_prot.write_field_end()?
    }
    if let Some(ref fld_var) = self.annotations {
      o_prot.write_field_begin(&TFieldIdentifier::new("annotations", TType::List, 7))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::Map, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_map_begin(&TMapIdentifier::new(TType::String, TType::String, e.len() as i32))?;
        for (k, v) in e {
          o_prot.write_string(k)?;
          o_prot.write_string(v)?;
          o_prot.write_map_end()?;
        }
        o_prot.write_list_end()?;
      }
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Default for TUserDefinedTableFunction {
  fn default() -> Self {
    TUserDefinedTableFunction{
      name: Some("".to_owned()),
      sizer_type: None,
      sizer_arg_pos: Some(0),
      input_arg_types: Some(Vec::new()),
      output_arg_types: Some(Vec::new()),
      sql_arg_types: Some(Vec::new()),
      annotations: Some(Vec::new()),
    }
  }
}
