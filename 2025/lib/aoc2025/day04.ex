defmodule Aoc2025.Day04 do

  def solve(lines) do
    { width, height, room } = parse(lines)
    { solve1(width, height, room), solve2(width, height, room) }
  end

  defp solve1(width, height, room) do
    1..width
    |> Enum.map(fn y -> 
      1..height
      |> Enum.filter(fn x -> can_be_remove_neigbours(room, y, x) end)
      |> Enum.count()
    end) 
    |> Enum.sum()
  end

  defp solve2(width, height, room) do
    {removed, new_room} = remove(width, height, room)
    if removed > 0, do: removed + solve2(width, height, new_room), else: 0
  end


  defp remove(width, height, room) do
    remove = 1..width
      |> Enum.map(fn y -> 
        1..height
        |> Enum.filter(fn x -> can_be_remove_neigbours(room, y, x) end)
        |> Enum.map(fn x -> {y, x} end)
      end)
      |> Enum.flat_map(&(&1))
      |> then(fn x -> MapSet.new(x) end)

    remove_count = MapSet.size(remove)

    new_room = 0..(height + 1)
      |> Enum.map(fn y -> 
        line = elem(room, y)
        0..(width + 1)
        |> Enum.map(fn x -> 
          value = elem(line, x)
          if MapSet.member?(remove, {y, x}), do: ?., else: value
        end) 
        |> List.to_tuple()
      end)
      |> List.to_tuple()

    {remove_count, new_room}
  end

  defp can_be_remove_neigbours(room, y, x) do
    if elem(elem(room, y), x) == ?@ do

      num = -1..1
        |> Enum.map(fn dy -> 
          -1..1
          |> Enum.map(fn dx -> {dy, dx} end)
        end)
        |> Enum.flat_map(&(&1))
        |> Enum.reject(&(&1 == {0, 0}))
        |> Enum.map(fn {dy, dx} -> 
          if elem(elem(room, y + dy), x + dx) == ?@, do: 1, else: 0
        end)
        |> Enum.sum()
      num < 4
    else
      false
    end
  end

  defp parse(lines) do
    width = String.length(hd(lines))
    height = length(lines)
    extra_line = List.duplicate(?., width + 2) |> List.to_tuple()

    room = lines
      |> Enum.map(fn line ->
        line
        |> String.to_charlist()
        |> then(fn x -> [?. | x] ++ [?.] end)
        |> List.to_tuple()
      end)
      |> then(fn x -> [extra_line | x] ++ [extra_line] end)
      |> List.to_tuple()

    {width, height, room}
  end

end

