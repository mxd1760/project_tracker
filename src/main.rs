
// Copyright (c) 2021 Okko Hakola
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#![allow(clippy::eq_op)]

// use egui::{ScrollArea, TextEdit, TextStyle};
use egui_winit_vulkano::{egui, Gui, GuiConfig};

use vulkano_util::{
    context::{VulkanoConfig, VulkanoContext},
    window::{VulkanoWindows, WindowDescriptor},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod views;
mod data;

// fn sized_text(ui: &mut egui::Ui, text: impl Into<String>, size: f32) {
//     ui.label(egui::RichText::new(text).size(size));
// }

pub fn main() {
    let mut test_data = data::app_data::AppData::default();
    let mut npv_context = views::new_project_view::NewProjectViewContext{
      show_skills:false,
      selected_skills:vec![]
    };


    // Winit event loop
    let event_loop = EventLoop::new();
    // Vulkano context
    let context = VulkanoContext::new(VulkanoConfig::default());
    // Vulkano windows (create one)
    let mut windows = VulkanoWindows::default();
    windows.create_window(&event_loop, &context, &WindowDescriptor{
      width:400.,
      height:400.,
      ..Default::default()
    }, |ci| {
        ci.image_format = vulkano::format::Format::B8G8R8A8_UNORM;
        ci.min_image_count = ci.min_image_count.max(2);
    });
    // Create gui as main render pass (no overlay means it clears the image each frame)
    let mut gui = {
        let renderer = windows.get_primary_renderer_mut().unwrap();
        Gui::new(
            &event_loop,
            renderer.surface(),
            renderer.graphics_queue(),
            renderer.swapchain_format(),
            GuiConfig::default(),
        )
    };
    // Create gui state (pass anything your state requires)
    //let mut code = CODE.to_owned();
    event_loop.run(move |event, _, control_flow| {
        let renderer = windows.get_primary_renderer_mut().unwrap();
        match event {
            Event::WindowEvent { event, window_id } if window_id == renderer.window().id() => {
                // Update Egui integration so the UI works!
                let _pass_events_to_game = !gui.update(&event);
                match event {
                    WindowEvent::Resized(_) => {
                        renderer.resize();
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        renderer.resize();
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                }
            }
            Event::RedrawRequested(window_id) if window_id == window_id => {
                // Set immediate UI in redraw here
                gui.immediate_ui(|gui| {
                    let ctx = gui.context();
                    egui::CentralPanel::default().show(&ctx, |ui| {
                      if views::new_project_view::show(&ctx,&mut test_data.projects[0],&mut npv_context,ui){
                        println!("testing");
                        test_data.save().expect("test failed");
                      };
                    });
                });
                // Render UI
                // Acquire swapchain future
                match renderer.acquire() {
                    Ok(future) => {
                        // Render gui
                        let after_future =
                            gui.draw_on_image(future, renderer.swapchain_image_view());
                        // Present swapchain
                        renderer.present(after_future, true);
                    }
                    Err(vulkano::VulkanError::OutOfDate) => {
                        renderer.resize();
                    }
                    Err(e) => panic!("Failed to acquire swapchain future: {}", e),
                };
            }
            Event::MainEventsCleared => {
                renderer.window().request_redraw();
            }
            _ => (),
        }
    });
}

// const CODE: &str = r"
// # Some markup
// ```
// let mut gui = Gui::new(&event_loop, renderer.surface(), None, renderer.queue(), SampleCount::Sample1);
// ```
// ";