
help = fn ->
  IO.puts("invalid options, call with: \n  MIX_ENV=release mix run encode.exs -i [*.y4m] -o [*.ivf]\n")
  exit(:invalid_arg)
end

if rem(length(System.argv()), 2) != 0, do: help.()

args = System.argv()
|> Enum.chunk_every(2)
|> Enum.reduce(%{}, fn [opt, value], acc ->
  case opt do
    "-i" -> Map.put(acc, :infile, value)
    "-o" -> Map.put(acc, :outfile, value)
    _ -> help.()
  end
end)

unless [:infile, :outfile] |> Enum.all?(&Map.has_key?(args, &1)), do: help.()

%{infile: infile, outfile: outfile} = args

# read source file
# External dependency, not present in this project:
#   https://hex.pm/packages/y4m
{props, stream} = Y4m.stream(infile)
%{width: w, height: h, frame_rate: [fr_num, fr_den], aspect_ratio: [ar_num, ar_den], color_space: cs} = props

# build ivf file header and write it to disk
outfp = File.open!(outfile, [:write, :binary])
file_header = <<"DKIF", 0::size(16), 32::size(16)-little, "AV01">>
file_header = file_header <> <<w::size(16)-little, h::size(16)-little>>
file_header = file_header <> <<fr_num::size(32)-little, fr_den::size(32)-little>>
file_header = file_header <> <<0::size(64)-little>>
:ok = IO.binwrite(outfp, file_header)

# helper function to write one packet to file
write_frame = fn fp, idx, pack ->
  :ok = IO.binwrite(fp, <<byte_size(pack)::size(32)-little>>)
  :ok = IO.binwrite(fp, <<idx::size(64)-little>>)
  :ok = IO.binwrite(fp, pack)
end

# init encoder
rav1e = Rav1e.init(%Rav1e.EncoderConfig{
  width: w, height: h,
  time_base: {fr_num, fr_den},
  sample_aspect_ratio: {ar_num, ar_den},
  bit_depth: 8,
  low_latency: true,
  speed_settings: 10,
  chroma_sampling: Atom.to_string(cs),
  error_resilient: true
})

stream |> Enum.reduce([], fn [y,u,v], acc ->
  case Rav1e.encode(rav1e, y, u, v) do
    # rav1e will hold a few frames in a buffer
    :need_more_data -> acc
    {:error, :failure} ->
      IO.puts("failed to encode")

    {:error, reason} ->
      IO.puts("warning: #{inspect(reason)}")

    {idx, av1_packet} ->
      write_frame.(outfp, idx, av1_packet)
  end
end)

# there are still frames stuck in the buffer
for {idx, av1_packet} <- Rav1e.flush(rav1e),
  do: write_frame.(outfp, idx, av1_packet)

:ok = File.close(outfp)
