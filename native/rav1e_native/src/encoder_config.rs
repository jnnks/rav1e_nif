use rav1e::prelude::{EncoderConfig, ChromaSampling, Rational, SpeedSettings};
use rustler::{NifStruct};

#[derive(Debug, NifStruct)]
#[module = "Rav1e.EncoderConfig"]

/// Erlang-serlializable struct to pass EncoderConfig from Elixir
pub struct ExEncoderConfig {
  // copy and pasted from rav1e::EncoderConfig
  // modified to be serializable in Erlang 

  // output size

  /// Width of the frames in pixels.
  pub width: usize,

  /// Height of the frames in pixels.
  pub height: usize,

  // Sample aspect ratio (for anamorphic video).
  pub sample_aspect_ratio: Option<(u64, u64)>,

  // Video time base.
  pub time_base: Option<(u64, u64)>,

  // data format and ancillary color information
  
  /// Bit depth.
  pub bit_depth: Option<usize>,

  /// Chroma subsampling.
  pub chroma_sampling: Option<String>,

  /// Chroma sample position.
  pub chroma_sample_position: Option<String>,

  /// Pixel value range.
  pub pixel_range: Option<String>,
//   /// Content color description (primaries, transfer characteristics, matrix).
// //   pub color_description: Option<ColorDescription>,
//   /// HDR mastering display parameters.
// //   pub mastering_display: Option<MasteringDisplay>,
//   /// HDR content light parameters.
// //   pub content_light: Option<ContentLight>,

  /// Enable signaling timing info in the bitstream.
  pub enable_timing_info: Option<bool>,

  /// Still picture mode flag.
  pub still_picture: Option<bool>,

  /// Flag to force all frames to be error resilient.
  pub error_resilient: Option<bool>,

  /// Interval between switch frames (0 to disable)
  pub switch_frame_interval: Option<u64>,

  // encoder configuration

  /// The *minimum* interval between two keyframes
  pub min_key_frame_interval: Option<u64>,

  /// The *maximum* interval between two keyframes
  pub max_key_frame_interval: Option<u64>,

  /// The number of temporal units over which to distribute the reservoir
  /// usage.
  pub reservoir_frame_delay: Option<i32>,

  /// Flag to enable low latency mode.
  ///
  /// In this mode the frame reordering is disabled.
  pub low_latency: Option<bool>,

  /// The base quantizer to use.
  pub quantizer: Option<usize>,

  /// The minimum allowed base quantizer to use in bitrate mode.
  pub min_quantizer: Option<u8>,

  /// The target bitrate for the bitrate mode.
  pub bitrate: Option<i32>,

  /// Metric to tune the quality for.
  pub tune: Option<String>,

  /// Number of tiles horizontally. Must be a power of two.
  ///
  /// Overridden by [`tiles`], if present.
  pub tile_cols: Option<usize>,
  
  /// Number of tiles vertically. Must be a power of two.
  ///
  /// Overridden by [`tiles`], if present.
  pub tile_rows: Option<usize>,

  /// Total number of tiles desired.
  ///
  /// Encoder will try to optimally split to reach this number of tiles,
  /// rounded up. Overrides [`tile_cols`] and [`tile_rows`].
  pub tiles: Option<usize>,

  /// Number of frames to read ahead for the RDO lookahead computation.
  pub rdo_lookahead_frames: Option<usize>,

  /// Settings which affect the encoding speed vs. quality trade-off.
  pub speed_settings: Option<usize>,
}

impl ExEncoderConfig {
    pub fn to_rav1e(self) -> rav1e::EncoderConfig {

        let default = EncoderConfig::default();
        let mut config = EncoderConfig {
            // output size
            width: self.width,
            height: self.height,
            sample_aspect_ratio: default.sample_aspect_ratio,
            time_base: default.time_base,
            
            // data format and ancillary color information
            bit_depth: self.bit_depth.unwrap_or(default.bit_depth),
            chroma_sampling: ChromaSampling::Cs444, // default.chroma_sampling,
            chroma_sample_position: default.chroma_sample_position,
            pixel_range: default.pixel_range,
            enable_timing_info: self.enable_timing_info.unwrap_or(default.enable_timing_info),
            still_picture: self.still_picture.unwrap_or(default.still_picture),
            error_resilient: self.error_resilient.unwrap_or(default.error_resilient),
            switch_frame_interval: self.switch_frame_interval.unwrap_or(default.switch_frame_interval),
            
            // encoder configuration
            min_key_frame_interval: self.min_key_frame_interval.unwrap_or(default.min_key_frame_interval),
            max_key_frame_interval: self.max_key_frame_interval.unwrap_or(default.max_key_frame_interval),
            reservoir_frame_delay:  self.reservoir_frame_delay,
            low_latency: self.low_latency.unwrap_or(default.low_latency),
            quantizer: self.quantizer.unwrap_or(default.quantizer),
            min_quantizer: self.min_quantizer.unwrap_or(default.min_quantizer),
            bitrate: self.bitrate.unwrap_or(default.bitrate),
            tune: default.tune,
            tile_cols: self.tile_cols.unwrap_or(default.tile_cols),
            tile_rows: self.tile_rows.unwrap_or(default.tile_rows),
            tiles: self.tiles.unwrap_or(default.tiles),
            rdo_lookahead_frames: self.rdo_lookahead_frames.unwrap_or(default.rdo_lookahead_frames),
            speed_settings: default.speed_settings,

            // unused
            color_description: default.color_description,
            mastering_display: default.mastering_display,
            content_light: default.content_light,
        };

        if let Some((num, den)) = self.sample_aspect_ratio {
            config.sample_aspect_ratio = Rational::new(num, den)
        }
        
        if let Some((num, den)) = self.time_base {
            config.time_base = Rational::new(num, den)
        }

        if let Some(chroma_sampling) = self.chroma_sampling {
          config.chroma_sampling = match chroma_sampling.as_str() {
            "C444" => ChromaSampling::Cs444, 
            "C420" => ChromaSampling::Cs420, 
            "C422" => ChromaSampling::Cs422, 
            "C400" => ChromaSampling::Cs400,
            _ => panic!("unknown ChromaSampling {}", chroma_sampling)
          }
        }

        if let Some((num, den)) = self.time_base {
          config.time_base = Rational::new(num, den)
        }
      

        if let Some(tune) = self.tune {
          config.tune = match tune.as_ref() {
            "psnr" => rav1e::prelude::Tune::Psnr,
            "psychovisual" => rav1e::prelude::Tune::Psychovisual,
            _ => panic!("unknown Tune {}", tune)
          }
        }

        if let Some(pixel_range) = self.pixel_range {
          config.pixel_range = match pixel_range.as_str() {
            "limited" => rav1e::prelude::PixelRange::Limited,
            "full" => rav1e::prelude::PixelRange::Full,
            _ => panic!("unknown PixelRange {}", pixel_range)
          }
        }

        if let Some(chroma_sample_position) = self.chroma_sample_position {
          config.chroma_sample_position = match chroma_sample_position.as_str() {
            "Unknown" => rav1e::prelude::ChromaSamplePosition::Unknown,
            "Vertical" => rav1e::prelude::ChromaSamplePosition::Vertical,
            "Colocated" => rav1e::prelude::ChromaSamplePosition::Colocated,
            _ => panic!("unknown ChromaSamplePosition {}", chroma_sample_position)
          }
        }

        if let Some(speed_settings) = self.speed_settings {
          config.speed_settings = SpeedSettings::from_preset(speed_settings)
        }

        config
    }
}