use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
  base_stat: u32,
  name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Genera {
  genus: String,
  language: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Names {
  language: String,
  name: String,
}
//Model from database
#[derive(Serialize, Deserialize, Debug)]
pub struct GetPoke {
  pub abilities: Vec<String>,
  pub base_experience: u32,
  pub height: f32,
  pub id: u32,
  pub is_default: bool,
  pub name: String,
  pub types: Vec<String>,
  pub weight: f32,
  pub image: String,
  pub base_happiness: u32,
  pub capture_rate: u32,
  pub has_gender_differences: bool,
  pub hatch_counter: u32,
  pub is_baby: bool,
  pub is_legendary: bool,
  pub is_mythical: bool,
  pub PredominantColor: String,
  pub stats: Vec<Stats>,
  pub genera: Vec<Genera>,
  pub names: Vec<Names>,
}