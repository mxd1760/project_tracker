use egui_winit_vulkano::egui;

use crate::{data::project::Project, AppActions};



pub struct AllProjectsViewContext{

}

struct ProjectResponse{
  should_delete:bool,
  edit_me:bool
}

fn draw_project(project:&Project,ui: &mut egui::Ui) -> ProjectResponse{
  let mut should_delete = false;
  let mut edit_me = false;
  ui.horizontal(|ui|{
    ui.label(&project.name);
    if ui.button("edit").clicked(){
      edit_me = true;
    }
    if ui.button("delete").clicked(){
      should_delete = true;
    };
  });
  ProjectResponse{should_delete,edit_me}
}

pub fn show(ctx:&egui::Context,data: &mut crate::data::app_data::AppData,context: &mut AllProjectsViewContext,ui: &mut egui::Ui) -> crate::AppActions{
  let mut deletables = vec![];
  let add;
  {
    for project in &data.projects{
      deletables.push(draw_project(project,ui));
    }
    add = ui.button("add new project").clicked();
  }
  
  for i in (0..deletables.len()).rev(){
    if deletables[i].edit_me{
      return AppActions::ChangeView(crate::View::EditProject(Some(i)))
    }
    if deletables[i].should_delete{
      data.projects.remove(i);
    }
  }
  if add{
    data.projects.push(Project::default());
  }
  crate::AppActions::DoNothing
}