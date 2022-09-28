defmodule Rav1e do
  defstruct [:rav1e, :config]

  @doc """
  Initialize encoder.
  """
  def init(%Rav1e.EncoderConfig{} = config) do
    rav1e = Rav1e.Native.init(config)
    %Rav1e{rav1e: rav1e, config: config}
  end

  @doc """
  Encode a single frame.

  Will return a either an AV1 packet or a `rav1e::api::util::EncoderState`.
  """
  def encode(%Rav1e{rav1e: rav1e}, y_plane, u_plane, v_plane) do
    Rav1e.Native.encode(rav1e, y_plane, u_plane, v_plane)
  end

  @doc """
  Encode remaining frames in the encoders internal buffer.
  """
  def flush(%Rav1e{rav1e: rav1e}), do: Rav1e.Native.flush(rav1e)
end

defmodule Rav1e.Native do
  use Rustler, otp_app: :rav1e, crate: "rav1e_native", mode: :release

  def init(_settings), do: :erlang.nif_error(:nif_not_loaded)
  def encode(_rav1e, _y_plane, _u_plane, _v_plane), do: :erlang.nif_error(:nif_not_loaded)
  def flush(_rav1e), do: :erlang.nif_error(:nif_not_loaded)
end
