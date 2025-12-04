defmodule Aoc2025.Day04 do
  @directions for dy <- -1..1, dx <- -1..1, {dy, dx} != {0, 0}, do: {dy, dx}

  def solve(lines) do
    room = parse(lines)
    {solve1(room), solve2(room)}
  end

  defp solve1(room), do: room |> removable_cells() |> MapSet.size()

  defp solve2(room) do
    to_remove = removable_cells(room)
    if MapSet.size(to_remove) == 0 do
      0
    else
      MapSet.size(to_remove) + solve2(Map.drop(room, MapSet.to_list(to_remove)))
    end
  end

  defp removable_cells(room) do
    for {pos, ?@} <- room, count_neighbors(room, pos) < 4, into: MapSet.new(), do: pos
  end

  defp count_neighbors(room, {y, x}) do
    Enum.count(@directions, fn {dy, dx} -> Map.get(room, {y + dy, x + dx}) == ?@ end)
  end

  defp parse(lines) do
    for {line, y} <- Enum.with_index(lines, 1),
        {char, x} <- Enum.with_index(String.to_charlist(line), 1),
        into: %{},
        do: {{y, x}, char}
  end
end
