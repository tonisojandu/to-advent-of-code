defmodule Aoc2025.Day05 do
  def solve(lines) do
    {ranges, ids} = parse(lines)

    sorted_borders = Enum.flat_map(ranges, fn {a, b} -> [{a, 1}, {b + 1, -1}] end)
    |> Enum.group_by(fn {index, _} -> index end, fn {_, delta} -> delta end)
    |> Enum.map(fn {index, deltas} -> {index, Enum.sum(deltas)} end)
    |> Enum.sort(fn {a, _}, {b, _} -> a < b end)

    { count(ids, sorted_borders), count_all_fresh(sorted_borders) }
  end

  defp count_all_fresh(sorted_borders) do
    Enum.reduce(sorted_borders, {0, 0, 0}, fn {index, delta}, {status, last_index, result} -> 
      new_status = status + delta

      if status > 0 do
        {new_status, index, result + (index - last_index)}
      else
        {new_status, index, result}
      end
    end)
    |> elem(2)
  end

  defp count(ids, sorted_borders) do
    Enum.map(ids, fn id -> 
      status = Enum.reduce_while(sorted_borders, 0, fn {index, delta}, acc -> 
        new_acc = acc + delta
        cond do
          id < index -> {:halt, acc}
          id >= index -> {:cont, new_acc}
        end
      end)

      if status > 0, do: 1, else: 0
    end)
    |> Enum.sum()
  end

  defp parse(lines) do
    split_at = Enum.find_index(lines, fn line -> String.trim(line) == "" end)

    ranges = Enum.slice(lines, (0..split_at - 1))
      |> Enum.map(fn line -> 
        [a, b] = String.split(line, "-")
        {String.to_integer(a), String.to_integer(b)}
      end)

    ids = Enum.slice(lines, (split_at + 1)..(length(lines) - 1))
    |> Enum.map(&String.to_integer/1) 

    {ranges, ids}
  end
end
