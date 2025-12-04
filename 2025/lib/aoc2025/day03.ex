defmodule Aoc2025.Day03 do
  def solve(lines) do
    parsed = parse(lines)
    {solve(2, parsed), solve(12, parsed)}
  end

  defp solve(n, lines) do
    lines
    |> Enum.map(&select_max_n(n, &1, 0))
    |> Enum.sum()
  end

  defp select_max_n(0, _, _), do: 0

  defp select_max_n(n, line, from) do
    until = tuple_size(line) - n

    {value, index} =
      from..until
      |> Enum.map(&{elem(line, &1), &1})
      |> Enum.max_by(&elem(&1, 0))

    Integer.pow(10, n - 1) * value + select_max_n(n - 1, line, index + 1)
  end

  defp parse(lines) do
    lines
    |> Enum.reject(&(String.trim(&1) == ""))
    |> Enum.map(fn line ->
      line
      |> String.to_charlist()
      |> Enum.map(&(&1 - ?0))
      |> List.to_tuple()
    end)
  end
end
