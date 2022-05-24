use atomic_float::AtomicF32;
use cpal::Stream;
use cpal::{traits::DeviceTrait, Device};
use eframe::egui::{self, InnerResponse, Layout, Ui};
use eframe::emath::Align;
use std::sync::atomic::Ordering;

use crate::streaming::{create_stream, get_devices};

pub const DEFAULT_THRESHOLD: f32 = -20.0;
pub const DEFAULT_ATTACK: f32 = 25.0;
pub const DEFAULT_RELEASE: f32 = 50.0;

pub static CURR_THRESHOLD: AtomicF32 = AtomicF32::new(DEFAULT_THRESHOLD);

struct AppData {
  devices: Vec<Device>,
  input_device_idx: Option<usize>,
  output_device_idx: Option<usize>,
  threshold: f32,
  running: bool,
  input_stream: Option<Stream>,
  output_stream: Option<Stream>,
}

fn get_device_name(devices: &[Device], idx: Option<usize>) -> String {
  idx.map_or_else(
    || "No Device Selected".to_string(),
    |idx| {
      devices[idx]
        .name()
        .unwrap_or_else(|_| "Unknown Device".to_string())
    },
  )
}

fn create_combo_box(
  ui: &mut Ui,
  label: &'static str,
  devices: &[Device],
  device_idx: &mut Option<usize>,
) -> InnerResponse<Option<()>> {
  let device_name = get_device_name(devices, *device_idx);

  ui.label(label);

  let combo = egui::ComboBox::from_id_source(label)
    .width(ui.available_width() - 7.0)
    .selected_text(device_name)
    .show_ui(ui, |ui| {
      for (i, d) in devices.iter().enumerate() {
        let device_name = d.name().unwrap_or_else(|_| "Unknown Device".to_string());

        ui.selectable_value(device_idx, Some(i), device_name);
      }
    });

  ui.end_row();

  combo
}

impl AppData {
  fn start_stream(&mut self) -> Option<bool> {
    let input_device_idx = self.input_device_idx?;
    let output_device_idx = self.output_device_idx?;

    let input_device = &self.devices[input_device_idx];
    let output_device = &self.devices[output_device_idx];

    let streams = create_stream(input_device, output_device, self.threshold)?;

    self.input_stream = Some(streams.0);
    self.output_stream = Some(streams.1);

    Some(true)
  }

  fn draw_start_stop_button(&mut self, ui: &mut Ui) {
    let button_text = if self.running { "Stop" } else { "Start" };

    if ui.button(button_text).clicked() {
      if self.running {
        self.input_stream = None;
        self.output_stream = None;

        self.running = false;
      } else {
        self.running = self.start_stream().unwrap_or(false);
      }
    }

    if self.running {
      CURR_THRESHOLD.store(self.threshold, Ordering::SeqCst);
    }
  }

  fn draw_interface(&mut self, ui: &mut Ui) {
    create_combo_box(
      ui,
      "Input Device",
      &self.devices,
      &mut self.input_device_idx,
    );
    create_combo_box(
      ui,
      "Output Device",
      &self.devices,
      &mut self.output_device_idx,
    );

    ui.label("Threshold");
    ui.add(egui::Slider::new(&mut self.threshold, -200.0..=0.0).max_decimals(0));
    ui.end_row();
  }
}

impl eframe::App for AppData {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.spacing_mut().slider_width = ui.available_width() - 138.0;

      ui.with_layout(Layout::top_down_justified(Align::default()), |ui| {
        egui::Grid::new("app_grid")
          .num_columns(2)
          .spacing([10.0, 10.0])
          .show(ui, |ui| {
            self.draw_interface(ui);

            ui.label("");
            ui.with_layout(Layout::right_to_left(), |ui| {
              self.draw_start_stop_button(ui);
            });
          });
      });
    });
  }
}

pub fn run() {
  let options = eframe::NativeOptions::default();

  eframe::run_native(
    "Audio Limiter",
    options,
    Box::new(|cc| {
      cc.egui_ctx.set_pixels_per_point(2.0);

      let app_data = AppData {
        devices: get_devices(),
        input_device_idx: None,
        output_device_idx: None,
        threshold: DEFAULT_THRESHOLD,
        running: false,
        input_stream: None,
        output_stream: None,
      };

      Box::new(app_data)
    }),
  );
}
