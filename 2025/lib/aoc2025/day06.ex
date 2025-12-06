defmodule Aoc2025.Day06 do
  def solve(lines) do
    {calculate_bundle(parse1(lines)), calculate_bundle(parse2(lines))}
  end

  defp calculate_bundle(bundles) do
    bundles
    |> Enum.map(fn
      {"*", nums} -> Enum.product(nums)
      {"+", nums} -> Enum.sum(nums)
    end)
    |> Enum.sum()
  end

  defp parse1(lines) do
    [operators | data] = Enum.reverse(lines)
    numbers = Enum.map(data, &parse_numbers/1)
    Enum.zip(parse_tokens(operators), transpose(numbers))
  end

  defp parse2(lines) do
    [operators | data] = Enum.reverse(lines)

    columns =
      data
      |> Enum.map(&String.to_charlist/1)
      |> transpose()
      |> Enum.map(&(&1 |> Enum.reverse() |> List.to_string() |> String.trim()))
      |> Enum.chunk_by(&(&1 == ""))
      |> Enum.reject(&(&1 == [""]))
      |> Enum.map(fn col -> Enum.map(col, &String.to_integer/1) end)

    Enum.zip(parse_tokens(operators), columns)
  end

  defp transpose(lists), do: Enum.zip_with(lists, & &1)
  defp parse_tokens(line), do: String.split(line, " ", trim: true)
  defp parse_numbers(line), do: line |> parse_tokens() |> Enum.map(&String.to_integer/1)
end
