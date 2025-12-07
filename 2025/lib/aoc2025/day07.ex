defmodule Aoc2025.Day07 do
  def solve(lines) do
    [head | tail] = lines

    start_light = 
      head
      |> String.to_charlist()
      |> Enum.map(& &1 == ?S)
      |> Enum.with_index()
      |> Enum.reduce(Map.new(), fn {true, i}, acc -> Map.put(acc, i, 1) 
        {false, _}, acc -> acc
      end)

    parsed = Enum.map(tail, &String.to_charlist/1)

    {final_light, total_splits} = Enum.reduce(parsed, {start_light, 0}, fn line, {in_light, result_acc} ->
      line 
      |> Enum.with_index()
      |> Enum.reduce({Map.new(), result_acc}, fn {value, index}, {out_light, inner_acc} ->
        parent_light = Map.get(in_light, index, 0)
        cond do
          parent_light > 0 and value == ?^  -> {splti_light(out_light, index, parent_light), inner_acc + 1}
          parent_light > 0 -> {update_light(out_light, index, parent_light), inner_acc}
          true -> {out_light, inner_acc}
        end
      end)
    end)

    total_paths =
      final_light
      |> Map.values()
      |> Enum.sum()

    {total_splits, total_paths}
  end

  defp splti_light(light_map, index, parent_count) do
    update_light(update_light(light_map, index + 1, parent_count), index - 1, parent_count)
  end

  defp update_light(light_map, index, parent_count) do
    current_count = Map.get(light_map, index, 0) 
    Map.put(light_map, index, parent_count + current_count)
  end
end
