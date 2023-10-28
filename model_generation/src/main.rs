#![feature(get_mut_unchecked)]

use ::csv::Reader;
use askama::Template;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, collections::HashMap, sync::Arc};

#[derive(Clone, Debug, Template)]
#[template(path = "partials/table.html")]
pub struct HtmlTableTemplate {
    records: Vec<Record>,
    caption: String,
}

#[derive(Clone, Debug, Template)]
#[template(path = "partials/svg.svg", escape = "none")]
pub struct SvgTemplate {
    pub x_axis_size: i32,
    pub y_axis_size: i32,
    pub x_width: i32,
    pub padding: i32,
    pub outline_color: String,
    pub fill_color: String,
    pub largest_value: i32,
    pub records: Vec<Record>,
}

mod filters {
    pub fn default<T: std::fmt::Display>(ot: Option<T>, default: String) -> ::askama::Result<String> {
      Ok(match ot {
        Some(t) => format!("{t}"),
        None => default,
      })
    }
    pub fn to_i32<T>(idx: &T) -> ::askama::Result<i32>
    where
        i32: TryFrom<T>,
        <i32 as TryFrom<T>>::Error: std::fmt::Debug,
        T: Copy,
    {
        Ok(i32::try_from(*idx).unwrap())
    }
    pub fn to_f64(i: &i32) -> ::askama::Result<f64> {
        Ok(f64::try_from(*i).unwrap())
    }
    pub fn f_to_i32(i: &f64) -> ::askama::Result<i32> {
        Ok((*i).round() as i32)
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "alpha.html")]
pub struct AlphaTemplate {
    pub svg: SvgTemplate,
    pub table: HtmlTableTemplate,
}

// TODO: replace with generic "dataframe"-like structure.
// we only ever can store one label, with two data points:
// an x and y axis.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub year: i32,
    pub revenue: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
#[serde(rename_all="lowercase")]
pub enum Direction {
  Left,
  Right,
}
impl std::fmt::Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Left => write!(f, "left"),
      Self::Right => write!(f, "right"),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinaryTreeRecord {
  pub id: i32,
  pub name: String,
  pub value: String,
  pub parent: Option<i32>,
  pub direction: Option<Direction>,
}

struct BinaryTree(Vec<BinaryTreeRecord>);

impl BinaryTree {
  fn children(&self, node: &BinaryTreeRecord) -> Self {
    BinaryTree(self.0.clone().into_iter().filter(|n| n.parent == Some(node.id)).collect())
  }
  fn parent(&self, node: &BinaryTreeRecord) -> Option<BinaryTreeRecord> {
    self.0.clone().into_iter().find(|n| Some(n.id) == node.parent)
  }
}

fn table_data() -> Result<(), Box<dyn Error>>{
    let file = File::open("../data.csv")?;
    let reader = BufReader::new(file);
    let mut rdr = Reader::from_reader(reader);
    let mut all_rows = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        all_rows.push(record);
    }

    let table = HtmlTableTemplate {
        records: all_rows.clone(),
        caption: "Revenue by Year".to_string(),
    };
    let largest_value = all_rows
        .iter()
        .max_by_key(|r| r.revenue)
        .unwrap()
        .revenue
        .clone()
        .into();
    let svg = SvgTemplate {
        records: all_rows,
        x_axis_size: 1000.into(),
        y_axis_size: 1000.into(),
        x_width: 50.into(),
        padding: 25.into(),
        outline_color: "none".to_string(),
        fill_color: "lightblue".to_string(),
        largest_value,
    };
    let alpha = AlphaTemplate { svg, table };

    println!("{}", &alpha.render()?);
    Ok(())
}

fn binary_tree_data() -> Result<(), Box<dyn Error>>{
    let file = File::open("../binary_data.csv")?;
    let reader = BufReader::new(file);
    let mut rdr = Reader::from_reader(reader);
    let mut all_rows = Vec::new();
    for result in rdr.deserialize() {
        let record: BinaryTreeRecord = result?;
        all_rows.push(record);
    }
    println!("{:?}", all_rows);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  Ok(binary_tree_data()?)
}
