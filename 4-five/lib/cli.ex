defmodule Commandline.CLI do
  def partition_line(line) do
    [head | tail] = Tuple.to_list(String.split_at(line, 7))
    %{:row => head, :col => Enum.at(tail, 0)}
  end

  def narrow_row_range(line, range) do
    narrowed =
      line[:row]
      |> String.graphemes()
      |> Enum.reduce(range, fn
        character, current_range when character == "F" ->
          %{
            :min => current_range[:min],
            :max => current_range[:max] - round((current_range[:max] - current_range[:min]) / 2)
          }

        character, current_range when character == "B" ->
          %{
            :min => current_range[:min] + round((current_range[:max] - current_range[:min]) / 2),
            :max => current_range[:max]
          }
      end)

    [last_elem | _] = Enum.reverse(line[:row] |> String.graphemes())

    case last_elem do
      "F" -> narrowed[:min]
      "B" -> narrowed[:max]
    end
  end

  def narrow_col_range(line, range) do
    narrowed =
      line[:col]
      |> String.graphemes()
      |> Enum.reduce(range, fn
        character, current_range when character == "L" ->
          %{
            :min => current_range[:min],
            :max => current_range[:max] - round((current_range[:max] - current_range[:min]) / 2)
          }

        character, current_range when character == "R" ->
          %{
            :min => current_range[:min] + round((current_range[:max] - current_range[:min]) / 2),
            :max => current_range[:max]
          }
      end)

    [last_elem | _] = Enum.reverse(line[:col] |> String.graphemes())

    val =
      case last_elem do
        "L" -> narrowed[:min]
        "R" -> narrowed[:max]
      end

    val
  end

  def get_seat_ids(lines) do
    partitioned_lines = lines |> Enum.map(&partition_line/1)

    rows =
      partitioned_lines
      |> Enum.map(fn line -> narrow_row_range(line, %{:min => 0, :max => 127}) end)

    cols =
      partitioned_lines
      |> Enum.map(fn line -> narrow_col_range(line, %{:min => 0, :max => 7}) end)

    IO.inspect(Enum.zip(rows, cols), charlists: :as_lists)

    Enum.zip(rows, cols)
    |> Enum.map(fn {row, col} -> row * 8 + col end)
  end

  def solution1(lines) do
    lines
    |> get_seat_ids()
    |> Enum.max()
  end

  def solution2(lines) do
    ids =
      lines
      |> get_seat_ids()

    present = MapSet.new(ids)
    Enum.min(present)..Enum.max(present)
    |> Enum.find(nil, fn
      id ->
        MapSet.member?(present, id - 1) && MapSet.member?(present, id + 1) &&
          !MapSet.member?(present, id)
    end)
  end

  def main(args) do
    options = [strict: [filename: :string]]
    {parsed, _} = OptionParser.parse!(args, options)
    input = File.read!(parsed[:filename]) |> String.split("\n")

    input |> solution1 |> IO.inspect()
    input |> solution2 |> IO.inspect()
  end
end
