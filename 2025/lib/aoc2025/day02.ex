defmodule Aoc2025.Day02 do
  def solve(lines) do
    ranges =
      lines
      |> hd()
      |> String.split(",")
      |> Enum.reject(&(String.trim(&1) == ""))
      |> Enum.map(fn x ->
        [a, b] = String.split(x, "-")
        {String.to_integer(a), String.to_integer(b)}
      end)

    max_num =
      ranges
      |> Enum.flat_map(&Tuple.to_list/1)
      |> Enum.max()

    first = doubles(ranges, max_num)
    second = add_multiples(ranges, max_num)
    {sum_unique(first), sum_unique(second)}
  end

  defp sum_unique(values) do
    values |> Enum.uniq() |> Enum.sum()
  end

  defp doubles(ranges, max_num) do
    Stream.iterate(1, &(&1 + 1))
    |> Stream.map(&make_double/1)
    |> Stream.take_while(&(&1 <= max_num))
    |> Enum.filter(&in_ranges?(ranges, &1))
  end

  defp make_double(n) do
    digits = Integer.digits(n)
    Integer.undigits(digits ++ digits)
  end

  defp add_multiples(ranges, max_num) do
    Stream.iterate(1, &(&1 + 1))
    |> Stream.map(&{&1, shifter_for(&1)})
    |> Stream.take_while(fn {n, shifter} -> n * shifter + n <= max_num end)
    |> Stream.flat_map(fn {n, shifter} -> rep_pattern(n, shifter, max_num) end)
    |> Enum.filter(&in_ranges?(ranges, &1))
  end

  defp shifter_for(n) do
    digits = Integer.digits(n) |> length()
    Integer.pow(10, digits)
  end

  defp rep_pattern(n, shifter, max_num) do
    Stream.iterate(n * shifter + n, &(&1 * shifter + n))
    |> Stream.take_while(&(&1 <= max_num))
    |> Enum.to_list()
  end

  defp in_ranges?(ranges, x) do
    Enum.any?(ranges, fn {a, b} -> a <= x and x <= b end)
  end
end
