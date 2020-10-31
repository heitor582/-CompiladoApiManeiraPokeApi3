use bson::{doc, document::Document};
use chrono::Utc;
use serde::{Deserialize, Serialize};

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
pub struct Poke {
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

impl Poke {
  pub fn to_bson(&self) -> Document {
    doc! {
      "abilities": self.abilities.to_owned(),
      "base_experience": self.base_experience.to_owned(),
      "height": self.height.to_owned(),
      "id": self.id.to_owned(),
      "is_default": self.is_default.to_owned(),
      "name": self.name.to_owned(),
      "types": self.types.to_owned(),
      "weight": self.weight.to_owned(),
      "image": self.image.to_owned(),
      "base_happiness": self.base_happiness.to_owned(),
      "capture_rate": self.capture_rate.to_owned(),
      "has_gender_differences": self.has_gender_differences.to_owned(),
      "hatch_counter": self.hatch_counter.to_owned(),
      "is_baby": self.is_baby.to_owned(),
      "is_legendary": self.is_legendary.to_owned(),
      "is_mythical": self.is_mythical.to_owned(),
      "PredominantColor" self.PredominantColor.to_owned()
    }
  }
}
