defmodule Aoc2025.Day02 do
  def solve(lines) do
    ranges = String.split(hd(lines), ",")
    |> Enum.filter(fn x -> String.trim(x) != "" end)
    |> Enum.map(fn x -> String.split(x, "-") end)
    |> Enum.map(fn [a, b] -> { String.to_integer(a), String.to_integer(b) } end)

    max_num = Enum.flat_map(ranges, (fn {a, b} -> [a, b] end))
    |> Enum.max()

    first = doubles(ranges, 1, 10, max_num)
    second = add_multiples(ranges, 1, 10, max_num)
    { sum_unique(first), sum_unique(second) }
  end

  def unique_sorted(values) do
    values
    |> Enum.uniq()
    |> Enum.sort()
  end

  def sum_unique(values) do
    Enum.sum(unique_sorted(values))
  end

  def doubles(ranges, n, shifter, max_num) do
    new_shifter = if n >= shifter do shifter * 10 else shifter end
    check_num = n * new_shifter + n
    if check_num <= max_num do 
      tail = doubles(ranges, n + 1, new_shifter, max_num)
      if check_ranges(ranges, check_num), do: [check_num | tail], else: tail
    else
      []
    end
  end


  def add_multiples(range, n, shifter, max_num) do
    new_shifter = if n >= shifter do shifter * 10 else shifter end
    if n * new_shifter + n <= max_num do
      Enum.concat(recurse_fixed(range, n, new_shifter, max_num, n), add_multiples(range, n + 1, new_shifter, max_num))
    else
      []
    end
  end

  def recurse_fixed(ranges, n, shifter, max_num, last_checked) do
    check_num = last_checked * shifter + n
    if check_num <= max_num do
      tail = recurse_fixed(ranges, n, shifter, max_num, check_num)
      if check_ranges(ranges, check_num), do:  [check_num | tail], else: tail
    else
      []
    end 
  end

  def check_ranges([], _), do: false 
  def check_ranges([h | t], x), do: between(x, h) or check_ranges(t, x) 

  def between(x, {a, b}) do
    a <= x and x <= b
  end
end
