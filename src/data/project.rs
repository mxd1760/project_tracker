use crate::data::{
  skill::Skill,
  task::Task,
};

use chrono::{DateTime, Local,Months};
// use directories::ProjectDirs;
// use egui_winit_vulkano::egui::epaint::QuadraticBezierShape;

use serde::ser::SerializeStruct;

pub struct Project{
  pub name:String,
  pub skills:Vec<Skill>,
  pub tasks:Vec<Task>,
  pub start_date: DateTime<Local>,
  pub next_review: DateTime<Local>,
  pub extras: Vec<String>,
}

impl serde::Serialize for Project{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: serde::Serializer {
      let mut state = serializer.serialize_struct("Project",6)?;
      state.serialize_field("name",&self.name)?;
      state.serialize_field("skills",&self.skills)?;
      state.serialize_field("tasks",&self.tasks)?;
      state.serialize_field("start_date",&self.start_date.timestamp())?;
      state.serialize_field("next_review",&self.next_review.timestamp())?;
      state.serialize_field("extras",&self.extras)?;
      state.end()
  }
}

// TODO
// impl<'de> serde::Deserialize<'de> for Project{
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//       where
//           D: serde::Deserializer<'de> {
      
//   }
// }
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