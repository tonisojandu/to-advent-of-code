defmodule Aoc2025.Day05 do
  def solve(lines) do
    {ranges, ids} = parse(lines)

    sorted_borders =
      ranges
      |> Enum.flat_map(fn {a, b} -> [{a, 1}, {b + 1, -1}] end)
      |> Enum.group_by(&elem(&1, 0), &elem(&1, 1))
      |> Enum.map(fn {index, deltas} -> {index, Enum.sum(deltas)} end)
      |> Enum.sort_by(&elem(&1, 0))

    {count(ids, sorted_borders), count_all_fresh(sorted_borders)}
  end

  defp count_all_fresh(sorted_borders) do
    sorted_borders
    |> Enum.reduce({0, 0, 0}, fn {index, delta}, {status, last_index, result} ->
      new_result = if status > 0, do: result + index - last_index, else: result
      {status + delta, index, new_result}
    end)
    |> elem(2)
  end

  defp count(ids, sorted_borders) do
    Enum.count(ids, fn id ->
      sorted_borders
      |> Enum.reduce_while(0, fn {index, delta}, acc ->
        if id < index, do: {:halt, acc}, else: {:cont, acc + delta}
      end)
      |> Kernel.>(0)
    end)
  end

  defp parse(lines) do
    {range_lines, ["" | id_lines]} = Enum.split_while(lines, &(&1 != ""))

    ranges = Enum.map(range_lines, fn line ->
      [a, b] = String.split(line, "-")
      {String.to_integer(a), String.to_integer(b)}
    end)

    {ranges, Enum.map(id_lines, &String.to_integer/1)}
  end
end
