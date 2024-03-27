use egui_winit_vulkano::egui;

const ALL_SKILLS: &[&str] = &[
  "rust",
  "python",
  "c++",
  "project planning",
  "math",
  "science",
  "art",
  "philosophy",
  "history",
];

pub struct NewProjectViewContext{
  pub show_skills:bool,
  pub selected_skills:Vec<String>,
}

pub fn show(ctx:&egui::Context,data: &mut crate::data::project::Project,context: &mut NewProjectViewContext,ui: &mut egui::Ui) -> bool{
  ui.label("Create A New Project!");
  ui.horizontal(|ui|{
    ui.label("Name:");
    if ui.text_edit_singleline(&mut data.name).changed(){
      println!("{}",data.name);
    };
  });
  let button = ui.button("+ Skills");
  if button.clicked(){
    context.show_skills = !context.show_skills;
  }
  if context.show_skills{
    egui::Window::new("").anchor(egui::Align2::LEFT_TOP,(60.,50.)).collapsible(false).title_bar(false).resizable(false).show(ctx, |ui|{
      ui.horizontal_wrapped(|ui|{
        for skill in ALL_SKILLS {
          let mut checked = context.selected_skills.contains(&String::from(*skill));
          if ui.checkbox(&mut checked ,*skill).clicked(){
            if checked{
              context.selected_skills.push(String::from(*skill));
            }else{
              let index = context.selected_skills.iter().position(|x| *x == *skill).unwrap();
              context.selected_skills.remove(index);
            }
          }
        }
      })
    });
  }

  ui.label(format!("{}",context.selected_skills.len()));
  if ui.button("Submit").clicked(){
    return true;
  }
  false
}