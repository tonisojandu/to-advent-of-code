defmodule Aoc2025.Day08 do
  def solve(lines) do
    [h | _] = lines
    times = if h == "162,817,812", do: 10, else: 1000 

    parsed = lines |> Enum.map(fn line -> String.split(line, ",") |> Enum.map(&String.to_integer/1) |> List.to_tuple end)

    sorted_distances = for {light_i, i} <- Enum.with_index(parsed), {light_j, j} <- Enum.with_index(parsed), j > i do 
      {x_i, _, _} = light_i
      {x_j, _, _} = light_j
      {i, j, distance(light_i, light_j), x_i, x_j}
    end |> Enum.sort_by(&elem(&1, 2))

    inital_groups = 0..(length(parsed) - 1) |> Enum.to_list |> List.to_tuple()

    IO.puts("Doing #{times} iterations")
    product_for_10 = sorted_distances
      |> Enum.reduce_while({inital_groups, times}, fn _, {mid_groups, 0} -> {:halt, {mid_groups, 0}} 
        {i, j, _, _, _}, {mid_groups, join_needed} -> 
          i_group = elem(mid_groups, i)
          j_group = elem(mid_groups, j)
          new_groups = Tuple.to_list(mid_groups)
            |> Enum.map(fn v ->  if v == j_group, do: i_group, else: v end)
            |> List.to_tuple()
          {:cont, {new_groups, join_needed - 1}}
      end)
      |> then(&elem(&1, 0))
      |> Tuple.to_list()
      |> Enum.group_by(& &1, fn _ -> 1 end)
      |> Stream.map(fn {_, list} -> Enum.sum(list) end)
      |> Enum.sort(&(&1 >= &2))
      |> Enum.take(3)
      |> Enum.product()

    product_2_last_coords = sorted_distances
      |> Enum.reduce_while({inital_groups, 0, 0}, fn {i, j, _, light_i, light_j}, {mid_groups, last_i, last_j} -> 
        compare_to = elem(mid_groups, 0)
        case Enum.all?(mid_groups |> Tuple.to_list, fn x -> x == compare_to end) do
          false ->
            i_group = elem(mid_groups, i)
            j_group = elem(mid_groups, j)
            new_groups = Tuple.to_list(mid_groups)
              |> Enum.map(fn v ->  if v == j_group, do: i_group, else: v end)
              |> List.to_tuple()
            {:cont, {new_groups, light_i, light_j}}
          true -> {:halt, {mid_groups, last_i, last_j}}
        end
      end)
      |> then(fn {_, x_i, x_j} -> x_i * x_j end)

    {product_for_10, product_2_last_coords}
  end

  defp distance({x_i, y_i, z_i}, {x_j, y_j, z_j}) do 
    xd = (x_i - x_j) ** 2
    yd = (y_i - y_j) ** 2
    zd = (z_i - z_j) ** 2
    xd + yd + zd
  end
end

