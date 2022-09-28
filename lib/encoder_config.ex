defmodule Rav1e.EncoderConfig do
  @moduledoc """
  Encoder settings which impact the produced bitstream.

  Mirror implementation of `rav1e::api::config::encoder::EncoderConfig`
  """

  @enforce_keys [:width, :height]
  defstruct [
    # Width of the frames in pixels.
    :width,

    # Height of the frames in pixels.
    :height,

    # Sample aspect ratio (for anamorphic video).
    :sample_aspect_ratio,

    # Video time base.
    :time_base,

    # data format and ancillary color information

    # Bit depth.
    :bit_depth,

    # Chroma subsampling.
    :chroma_sampling,

    # Chroma sample position.
    :chroma_sample_position,

    # Pixel value range.
    :pixel_range,

    # Enable signaling timing info in the bitstream.
    :enable_timing_info,

    # Still picture mode flag.
    :still_picture,

    # Flag to force all frames to be error resilient.
    :error_resilient,

    # Interval between switch frames (0 to disable)
    :switch_frame_interval,

    # encoder configuration

    # The *minimum* interval between two keyframes
    :min_key_frame_interval,

    # The *maximum* interval between two keyframes
    :max_key_frame_interval,

    # The number of temporal units over which to distribute the reservoir
    # usage.
    :reservoir_frame_delay,

    # Flag to enable low latency mode.
    # In this mode the frame reordering is disabled.
    :low_latency,

    # The base quantizer to use.
    :quantizer,

    # The minimum allowed base quantizer to use in bitrate mode.
    :min_quantizer,

    # The target bitrate for the bitrate mode.
    :bitrate,

    # Metric to tune the quality for.
    :tune,

    # Number of tiles horizontally. Must be a power of two.
    # Overridden by [`tiles`], if present.
    :tile_cols,

    # Number of tiles vertically. Must be a power of two.
    # Overridden by [`tiles`], if present.
    :tile_rows,

    # Total number of tiles desired.
    # Encoder will try to optimally split to reach this number of tiles,
    # rounded up. Overrides [`tile_cols`] and [`tile_rows`].
    :tiles,

    # Number of frames to read ahead for the RDO lookahead computation.
    :rdo_lookahead_frames,

    # Settings which affect the encoding speed vs. quality trade-off.
    :speed_settings
  ]
end
