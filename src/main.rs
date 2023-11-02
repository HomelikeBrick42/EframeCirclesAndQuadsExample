#![allow(dead_code, unused)]

use cgmath::{prelude::*, Vector2};
use eframe::{
    egui,
    egui_wgpu::{Callback, WgpuConfiguration},
    wgpu::{self},
    NativeOptions, Renderer,
};
use rendering::{create_render_state, GpuCamera, GpuCircle, RenderCallback};
use std::collections::HashSet;

mod rendering;

struct Camera {
    position: Vector2<f32>,
    zoom: f32,
}

struct App {
    last_frame_time: Option<std::time::Instant>,
    info_window_open: bool,
    background_color: egui::Color32,
    physics_ticks: u32,
    physics_time: std::time::Duration,
    time_scale: f32,
    camera: Camera,
}

impl App {
    fn new(cc: &eframe::CreationContext) -> App {
        create_render_state(cc);
        App {
            last_frame_time: None,
            info_window_open: true,
            background_color: egui::Color32::from_rgb(0, 0, 0),
            physics_ticks: 100,
            physics_time: std::time::Duration::ZERO,
            time_scale: 1.0,
            camera: Camera {
                position: Vector2 { x: 0.0, y: 0.0 },
                zoom: 0.25,
            },
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let time = std::time::Instant::now();
        let dt = time.duration_since(self.last_frame_time.unwrap_or(time));
        self.last_frame_time = Some(time);
        self.physics_time += dt.mul_f32(self.time_scale.abs());

        let time_step = std::time::Duration::from_secs(1) / self.physics_ticks;
        let ts = time_step.as_secs_f32() * self.time_scale.signum();
        while self.physics_time >= time_step {
            // maybe do phyiscs stuff here

            self.physics_time -= time_step;
        }

        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.info_window_open |= ui.button("Info").clicked();
            });
        });

        egui::Window::new("Info")
            .open(&mut self.info_window_open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.collapsing("Background Color", |ui| {
                    egui::color_picker::color_picker_color32(
                        ui,
                        &mut self.background_color,
                        egui::color_picker::Alpha::Opaque,
                    );
                });
                ui.label(format!("FPS: {:.3}", 1.0 / dt.as_secs_f64()));
                ui.label(format!("Frame Time: {:.3}ms", 1000.0 * dt.as_secs_f64()));
                ui.horizontal(|ui| {
                    ui.label("Physics Ticks: ");
                    ui.add(egui::Slider::new(&mut self.physics_ticks, 1..=1000));
                });
                ui.horizontal(|ui| {
                    ui.label("Time Scale: ");
                    ui.add(egui::Slider::new(&mut self.time_scale, -20.0..=20.0));
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(self.background_color))
            .show(ctx, |ui| {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
                let aspect = rect.width() / rect.height();

                if response.dragged_by(egui::PointerButton::Secondary) {
                    let delta = response.drag_delta();
                    self.camera.position.x -=
                        delta.x / self.camera.zoom / rect.width() * 2.0 * aspect;
                    self.camera.position.y += delta.y / self.camera.zoom / rect.height() * 2.0;
                }

                if response.dragged_by(egui::PointerButton::Primary) {
                    'drag: {
                        let Some(interact_pointer_pos) = response.interact_pointer_pos() else {
                            break 'drag;
                        };
                        let mouse_position =
                            ((interact_pointer_pos - rect.left_top()) / rect.size() * 2.0
                                - egui::vec2(1.0, 1.0))
                                * egui::vec2(1.0, -1.0);
                        let world_position = Vector2 {
                            x: mouse_position.x * aspect / self.camera.zoom
                                + self.camera.position.x,
                            y: mouse_position.y / self.camera.zoom + self.camera.position.y,
                        };

                        // use the world position to find out where you clicked
                        _ = world_position;
                    }
                } else {
                    // not being dragged anymore
                }

                if response.hovered() {
                    'hover: {
                        let Some(hover_pos) = response.hover_pos() else {
                            break 'hover;
                        };
                        let mouse_position =
                            ((response.hover_pos().unwrap() - rect.left_top()) / rect.size() * 2.0
                                - egui::vec2(1.0, 1.0))
                                * egui::vec2(1.0, -1.0);
                        let world_position = Vector2 {
                            x: mouse_position.x * aspect / self.camera.zoom
                                + self.camera.position.x,
                            y: mouse_position.y / self.camera.zoom + self.camera.position.y,
                        };

                        // use the world position to know where you are hovering
                    }

                    ctx.input(|input| match input.scroll_delta.y.total_cmp(&0.0) {
                        std::cmp::Ordering::Less => self.camera.zoom *= 0.9,
                        std::cmp::Ordering::Greater => self.camera.zoom /= 0.9,
                        _ => {}
                    });
                }

                ui.painter().add(Callback::new_paint_callback(
                    rect,
                    RenderCallback {
                        camera: GpuCamera {
                            position: self.camera.position,
                            aspect,
                            zoom: self.camera.zoom,
                        },
                        circles: vec![GpuCircle {
                            position: cgmath::vec2(0.0, 0.0),
                            color: cgmath::vec3(1.0, 0.0, 0.0),
                            radius: 1.0,
                        }],
                        rectangles: vec![],
                    },
                ));
            });

        ctx.request_repaint();
    }
}

fn main() {
    eframe::run_native(
        "eframe circles and rectangles",
        NativeOptions {
            vsync: false,
            renderer: Renderer::Wgpu,
            wgpu_options: WgpuConfiguration {
                power_preference: wgpu::PowerPreference::HighPerformance,
                present_mode: wgpu::PresentMode::AutoNoVsync,
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}
