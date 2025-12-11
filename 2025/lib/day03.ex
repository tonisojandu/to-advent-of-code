defmodule Aoc2025.Day03 do
  def solve(lines) do
    parsed = parse(lines)
    { solve(2, parsed), solve(12, parsed) }
  end

  defp solve(n, lines) do
    lines |> Enum.map(&(select_max_n(n, &1, 0)))
    |> Enum.sum()
  end

  defp select_max_n(0, _, _), do: 0
  defp select_max_n(n, line, from) do
    until = tuple_size(line) - n
    {selected_value, selected_index} = Enum.reduce(from..until, {-1, -1}, fn index, {max_val, max_index} -> 
      with value <- elem(line, index) do
        if value > max_val, do: {value, index}, else: {max_val, max_index}
      end
    end)
    Integer.pow(10, n - 1) * selected_value + select_max_n(n - 1, line, selected_index + 1)
  end  

  defp parse(lines) do
    lines |> Enum.reject(&(String.trim(&1) == ""))
    |> Enum.map(&String.to_charlist/1)
    |> Enum.map(&digits_to_ints/1)
  end

  defp digits_to_ints(digits) do
    digits |> Enum.map(&(&1 - ?0))
    |> List.to_tuple()
  end
end

