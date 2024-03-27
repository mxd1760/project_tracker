use directories::ProjectDirs;
use crate::data::project::Project;
use crate::data::skill::Skill;

use std::fs::File;
use std::io::Write;
//use std::path::Path;


const APP_NAME: &'static str = "Project Tracker";
const ORG_NAME: &'static str = "MDM";
const QUALIFIER: &'static str = "com";


#[derive(serde::Serialize)]// TODO impl deserialize later 
pub struct AppData{
  pub projects:Vec<Project>,
  pub skills:Vec<Skill>
}

impl AppData{
  pub fn save(&mut self) -> Result<(),String>{
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER,ORG_NAME,APP_NAME){
      let path = proj_dirs.data_dir();
      match std::fs::create_dir_all(path){
        Err(why) => return Err(format!("Failed to create path because: {}",why).into()),
        Ok(_)=>(),
      };
      let mut file = match File::create(path.join("app_data.yaml")){
        Err(why) => return Err(format!("Failed to open file because: {}",why).into()),
        Ok(f) => f,
      };

      let data = match serde_yaml::to_string(&self){
        Ok(d) => d,
        Err(why) => return Err(format!("Failed to Serialize data because: {}",why).into()),
      };
      println!("{:?}",data);
      match file.write_all(&data.into_bytes()){
        Ok(_) => {},
        Err(why) => return Err(format!("Failed to write to file because: {}",why).into()),
      };
      Ok(())
    }else{
      Err("directory not found".into())
    }
    
  }
  // pub fn load() -> Self{
  //   Self{
  //     projects:vec![],
  //     skills:vec![],
  //   }
  // }
}

impl Default for AppData{
  fn default() -> Self{
    Self { 
      projects: vec![crate::data::project::Project::default()], 
      skills: vec![] 
    }
  }
}