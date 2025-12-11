defmodule Aoc2025.Day09 do
  def solve(lines) do
    parsed =
      lines
      |> Enum.map(fn l -> l |> String.split(",") |> Enum.map(&String.to_integer/1) |> then(fn [a, b] -> {a, b} end) end)

    corner_pairs = for {first, i} <- Enum.with_index(parsed), {second, j} <- Enum.with_index(parsed), j > i, do: {first, second}

    {part1(corner_pairs), part2(parsed, corner_pairs)}
  end

  defp part2(parsed, corner_pairs) do
    [_ | offset] = with [h | _] <- parsed, do: parsed ++ [h]

    {horizontal_borders, vertical_borders} = 
      Enum.zip_with(parsed, offset, &line_coords/2)
      |> Enum.reduce({[], []}, fn coords = {{x1, y1}, {x2, y2}}, {last_horizontal, last_vertical} -> 
        cond do
          x1 == x2 -> {last_horizontal, [coords | last_vertical]}
          y1 == y2 -> {[coords | last_horizontal], last_vertical}
        end
      end)
      |> then(fn {hor, ver} -> {Enum.sort_by(hor, fn {{_, y}, _} -> y end), Enum.sort_by(ver, fn {{x, _}, _} -> x end)} end)

    {critical_x, critical_y} =
      parsed
      |> Enum.reduce({[], []}, fn {x, y}, {acc_x, acc_y} -> {[x | acc_x], [y | acc_y]} end)
      |> then(fn {x_list, y_list} -> {Enum.uniq(x_list), Enum.uniq(y_list)} end) 

    out_const = {-1, -1}

    IO.puts("Step 01")
    lazy_fill = for c_y <- critical_y do
      Enum.reduce(vertical_borders, {[], out_const}, fn {from = {x, y1}, to = {x, y2}}, {res, last_in} -> 
        cuts = min(y1, y2) <= c_y and c_y < max(y1, y2) 
        current_loc = {x, c_y}
        new_in = if cuts, do: (if last_in == out_const, do: current_loc, else: out_const), else: last_in 
        new_res = if last_in == out_const, do: res, else: [line(last_in, current_loc) | res]
        {[line(from, to) | new_res], new_in}
      end)
      |> then(&elem(&1, 0))
    end
      |> then(&agro_flat_map/1)

    IO.puts("Step 02")
    lazy_fill = for c_x <- critical_x do
      Enum.reduce(horizontal_borders, {lazy_fill, out_const}, fn {from = {x1, y}, to = {x2, y}}, {res, last_in} -> 
        cuts = min(x1, x2) <= c_x and c_x < max(x1, x2) 
        current_loc = {c_x, y}
        new_in = if cuts, do: (if last_in == out_const, do: current_loc, else: out_const), else: last_in 
        new_res = if last_in == out_const, do: res, else: [line(last_in, current_loc) | res]
        {[line(from, to) | new_res], new_in}
      end)
      |> then(&elem(&1, 0))
    end
      |> then(&agro_flat_map/1)

    IO.puts("Step 03")
    fill = 
      agro_flat_map(lazy_fill) 
      |> then(&group_tuples/1)
      |> then(fn m -> %{x: group_tuples(Map.get(m, "x")), y: group_tuples(Map.get(m, "y"))}end)
      |> then(fn m ->
        Enum.map(m, fn {ordinal, coords} -> 
          {ordinal, Enum.map(coords, fn {coord, intervals} -> {coord, merge(intervals)} end) |> then(&Map.new/1)}
        end)
        |> then(&Map.new/1)
      end)

    IO.puts("Step 04")
    Enum.reduce(corner_pairs, -1, fn {first = {x1, y1}, second = {x2, y2}}, current_max -> 
      a = area(first, second)
      if a < current_max do
        current_max
      else
        if check({x1, y1}, {x1, y2}, fill) and check({x1, y2}, {x2, y2}, fill) and check({x2, y1}, {x2, y2}, fill) and check({x1, y1}, {x1, y2}, fill) do
          a
        else
          current_max
        end
      end
    end)
  end

  defp check({x, y1}, {x, y2}, fill), do: do_check(fill, :x, x, y1, y2)
  defp check({x1, y}, {x2, y}, fill), do: do_check(fill, :y, y, x1, x2)
  defp do_check(fill, ordinal, coord, check_i1, check_i2) do
    min_i = min(check_i1, check_i2)
    max_i = max(check_i1, check_i2)
    coord_map = Map.get(fill, ordinal)
    intervals = Map.get(coord_map, coord)
    Enum.any?(intervals, fn {i1, i2} -> min_i >= i1 and max_i <= i2 end)
  end

  defp merge(l) do
    Enum.sort_by(Enum.uniq(l), &elem(&1, 0))
    |> then(&do_merge/1)
  end 

  defp do_merge(l) do
    if length(l) < 2 do
      l
    else
      indexed = Enum.with_index(l)
      raw_merged_list = for {{a1, a2}, i} <- indexed, {{b1, b2}, j} <- indexed, i > j do
        if (a1 <= b2 and a2 >= b1) or (b1 <= a2 and b2 >= a1), do: [{i, j, {min(a1, b1), max(a2, b2)}}], else: []
      end
      |> then(&Enum.flat_map(&1, fn x -> x end))

      removed = 
        Enum.map(raw_merged_list, fn {i, j, _} -> [i, j] end)
        |> then(&Enum.flat_map(&1, fn x -> x end))
        |> then(&MapSet.new/1)

      keep = indexed |> then(fn l -> Enum.filter(l, fn {_, i} -> not MapSet.member?(removed, i) end) end) |> Enum.map(&elem(&1, 0))
      add = Enum.map(raw_merged_list, fn {_, _, v} -> v end)

      result = (keep ++ add) |> then(&Enum.uniq/1)

      if MapSet.size(removed) > 0, do: do_merge(result), else: result
    end
  end

  defp group_tuples(l), do: Enum.group_by(l, &elem(&1, 0), &elem(&1, 1)) 

  def line_s({x, y1}, {x, y2}), do: min(y1, y2)..max(y1, y2) |> Stream.map(&{x, &1})
  def line_s({x1, y}, {x2, y}), do: min(x1, x2)..max(x1, x2) |> Stream.map(&{&1, y})

  def line({x, y1}, {x, y2}), do: {"x", {x, {min(y1, y2), max(y1, y2)}}}
  def line({x1, y}, {x2, y}), do: {"y", {y, {min(x1, x2), max(x1, x2)}}}

  def agro_flat_map(l = [a | _]) when is_list(a), do: Enum.flat_map(l, fn x -> agro_flat_map(x) end)
  def agro_flat_map(x), do: x

  defp line_coords({x1, y1}, {x2, y2}) do
    cond do
      x1 == x2 -> {{x1, min(y1, y2)}, {x1, max(y1, y2)}}
      y1 == y2 -> {{min(x1, x2), y1}, {max(x1, x2), y1}}
    end
  end

  defp area({x1, y1}, {x2, y2}), do: (abs(x1 - x2) + 1)*(abs(y1 - y2) + 1)

  defp part1(corner_paris), do: Enum.reduce(corner_paris, -1, fn {first, second}, current_max -> max(current_max, area(first, second)) end)

end
