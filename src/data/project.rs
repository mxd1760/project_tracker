use crate::data::{
  skill::Skill,
  task::Task,
};
use std::fmt;

use chrono::{DateTime, Local, Months, TimeZone, Utc};
// use directories::ProjectDirs;
// use egui_winit_vulkano::egui::epaint::QuadraticBezierShape;

use serde::{de::{self, SeqAccess, Visitor}, ser::SerializeStruct, Deserialize, Deserializer};
use serde_yaml::with::singleton_map::deserialize;

#[derive(Clone)]
pub struct Project{
  pub name:String,
  pub skills:Vec<Skill>,
  pub tasks:Vec<Task>,
  pub start_date: DateTime<Local>,
  pub next_review: DateTime<Local>,
  pub extras: Vec<String>,
}

#[derive(serde::Serialize,serde::Deserialize)]
pub struct SerdeProject{
  name:String,
  skills:Vec<Skill>,
  tasks:Vec<Task>,
  start_date: String,
  next_review: String,
  extras:Vec<String>
}

impl serde::Serialize for Project{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: serde::Serializer {
      let temp = self.clone().to_serde();
      temp.serialize(serializer)
  }
}
impl<'de> serde::Deserialize<'de> for Project{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        // let x = Vec::<Skill>::deserialize(deserailizer).unwrap();
        let temp = SerdeProject::deserialize(deserializer).unwrap();
        Ok(Self::from_serde(temp))
    }
}
impl Project{
  fn to_serde(self) -> SerdeProject{
    SerdeProject{
      name:self.name,
      skills:self.skills,
      tasks:self.tasks,
      start_date:self.start_date.to_rfc3339(),
      next_review:self.next_review.to_rfc3339(),
      extras:self.extras
    }
  }
  fn from_serde(s:SerdeProject) -> Self{
    Self { name: s.name, skills: s.skills, tasks: s.tasks, 
      start_date: DateTime::from(DateTime::parse_from_rfc3339(&s.start_date).unwrap()), 
      next_review: DateTime::from(DateTime::parse_from_rfc3339(&s.next_review).unwrap()), 
      extras: s.extras }
  }
}



impl std::default::Default for Project{
  fn default() -> Self{
    Self { 
      name: "default".into(), 
      skills: vec![], 
      tasks: vec![], 
      start_date: Local::now(), 
      next_review: Local::now().checked_add_months(Months::new(1)).unwrap(), 
      extras: vec![] 
    }
  }
}


    // if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER,ORG_NAME,APP_NAME){
    //   let path = proj_dirs.project_path();
    //   path.join("Projects.data");
    //   let mut file = match std::fs::File::create(&path){
    //     Err(_) => {
    //       println!("did not save");
    //       return;
    //     },
    //     Ok(file) => file,
    //   };


    // }