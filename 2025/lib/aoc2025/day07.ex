defmodule Aoc2025.Day07 do
  def solve([head | tail]) do
    start_light =
      for {?S, i} <- head |> String.to_charlist() |> Enum.with_index(),
          into: %{},
          do: {i, 1}

    {final_light, total_splits} =
      tail
      |> Enum.map(&String.to_charlist/1)
      |> Enum.reduce({start_light, 0}, fn line, {in_light, splits} ->
        line
        |> Enum.with_index()
        |> Enum.reduce({%{}, splits}, fn {value, index}, {out_light, acc} ->
          parent_light = Map.get(in_light, index, 0)
          {new_light, split_count} = process_cell(value, index, parent_light, out_light)
          {new_light, acc + split_count}
        end)
      end)

    total_paths = final_light |> Map.values() |> Enum.sum()
    {total_splits, total_paths}
  end

  defp process_cell(?^, index, parent_light, out_light) when parent_light > 0 do
    {split_light(out_light, index, parent_light), 1}
  end

  defp process_cell(_value, index, parent_light, out_light) when parent_light > 0 do
    {update_light(out_light, index, parent_light), 0}
  end

  defp process_cell(_value, _index, _parent_light, out_light), do: {out_light, 0}

  defp split_light(light_map, index, parent_count) do
    light_map
    |> update_light(index + 1, parent_count)
    |> update_light(index - 1, parent_count)
  end

  defp update_light(light_map, index, parent_count) do
    Map.update(light_map, index, parent_count, &(&1 + parent_count))
  end
end
