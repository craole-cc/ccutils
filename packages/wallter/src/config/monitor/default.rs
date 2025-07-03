use super::{Position, Size};
use crate::config::path::Config as PathConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{
  cell::RefCell,
  fmt::{self, Display, Formatter}
};
use thiserror::Error as ThisError;
use winit::{
  application::ApplicationHandler,
  dpi::{PhysicalPosition, PhysicalSize},
  event::WindowEvent,
  event_loop::{ActiveEventLoop, EventLoop},
  monitor::MonitorHandle,
  window::WindowId
};

#[derive(ThisError, Debug)]
pub enum Error {
  #[error("Winit event loop error: {0}")]
  EventLoop(#[from] winit::error::EventLoopError)
}

pub type Result<T> = std::result::Result<T, Error>;

/// Represents a physical monitor and its properties.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
  /// Unique identifier for the monitor (based on enumeration order).
  pub id: u32,
  /// Human-readable monitor name (e.g., "DP-1", "HDMI-1").
  pub name: String,
  /// The dimensions of the monitor in pixels.
  pub size: Size,
  /// The monitor's position in the virtual screen space (x, y).
  pub position: Position,
  /// The monitor's scale factor (DPI scaling, e.g., 1.0 for 100%).
  pub scale: f32,
  /// Whether the monitor is the primary monitor. (Windows only)
  pub primary: bool
}

impl Display for Config {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    printf!(f, "Id", self.id)?;
    printf!(f, "Name", &self.name)?;
    printf!(f, "Height", self.size.height)?;
    printf!(f, "Width", self.size.width)?;
    printf!(f, "Resolution", self.size.resolution_str())?;
    printf!(f, "Orientation", &self.size.orientation())?;
    printf!(f, "Ratio", self.size.ratio_str())?;
    printf!(f, "Scale", format!("{:.1}x", self.scale))?;
    printf!(f, "Position", &self.position)?;
    printf!(f, "Primary", self.primary)?;

    Ok(())
  }
}

impl Config {
  /// Enumerate all monitors and return their information.
  pub fn get_info() -> Result<Vec<Self>> {
    let result = RefCell::new(Vec::new());

    struct Handler<'a> {
      result: &'a RefCell<Vec<Config>>
    }

    impl ApplicationHandler for Handler<'_> {
      fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        //{ Get the primary monitor handle }
        let primary_monitor = event_loop.primary_monitor();

        //{ Enumerate all monitors and store their info in the result }
        let monitors = event_loop
          .available_monitors()
          .enumerate()
          .map(|(i, handle)| {
            let id = i as u32;
            let raw_name =
              handle.name().unwrap_or_else(|| format!("Monitor // {id}"));
            let name = {
              let prefix = r"\\.\";
              raw_name
                .strip_prefix(prefix)
                .unwrap_or(&raw_name)
                .to_string()
            };
            let PhysicalSize { width, height } = &handle.size();
            let PhysicalPosition { x, y } = &handle.position();
            let size = Size::new(width, height);
            let position = Position::new(x, y);
            let scale = handle.scale_factor() as f32;
            let mut monitor = Config {
              id,
              name,
              size,
              position,
              scale,
              primary: false
            };

            //{ Determine if this is the primary monitor }
            monitor.primary = match &primary_monitor {
              Some(primary) => primary == &handle,
              None => false
            };
            monitor
          })
          .collect();

        //{ Set the result and exit the event loop }
        *self.result.borrow_mut() = monitors;
        event_loop.exit();
      }

      //{ Implement the other event handlers as no-ops }
      fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        _: WindowEvent
      ) {
      }
      fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        _device_event: winit::event::DeviceEvent
      ) {
        // No-op implementation
      }
      fn suspended(&mut self, _: &ActiveEventLoop) {}
      fn memory_warning(&mut self, _: &ActiveEventLoop) {}
    }

    let event_loop = EventLoop::new()?;
    let mut handler = Handler { result: &result };
    event_loop.run_app(&mut handler)?;

    Ok(result.into_inner())
  }

  /// Helper function to display wallpaper paths for a given monitor.
  /// This can be commented out in the `Display` impl to toggle visibility.
  pub fn display_wallpaper_paths(
    &self,
    f: &mut Formatter<'_>,
    path_config: &PathConfig
  ) -> fmt::Result {
    if let Some(monitor_path) = path_config
      .monitor_paths
      .iter()
      .find(|p| p.name == self.name)
    {
      // The default padding for `printf!` is 24, with 4 spaces of indentation.
      // To indent by 6 spaces while keeping the separator aligned,
      // we reduce the key padding by 2 (from 24 to 22).
      const PAD: usize = 22;
      const INDENT: usize = 6;
      printf!(
        f,
        "Available",
        monitor_path.download_dir.display(),
        PAD,
        INDENT
      )?;
      printf!(
        f,
        "Activated",
        monitor_path.current_wallpaper.display(),
        PAD,
        INDENT
      )?;
    }
    Ok(())
  }
}
