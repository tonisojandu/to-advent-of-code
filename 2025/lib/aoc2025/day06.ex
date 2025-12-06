defmodule Aoc2025.Day06 do
  def solve(lines) do
    {calculate_bundle(parse1(lines)), calculate_bundle(parse2(lines))}
  end

  defp calculate_bundle(bundles) do
    for {operator, numbers} <- bundles do
      {oper,start_acc} = if operator == "*", do: {&Kernel.*/2, 1}, else: {&Kernel.+/2,0}
      Enum.reduce(numbers, start_acc, fn x, acc -> oper.(x, acc) end)  
    end |> Enum.sum()
  end

  defp parse2(lines) do
    [operators | tail_reversed] = Enum.reverse(lines)

    new_lines = 
      tail_reversed 
      |> transpose_lines() 
      |> Enum.map(fn nums -> Enum.map(nums, &String.to_integer/1) end)

    bundle(breakup(operators), new_lines)
  end

  defp transpose_lines(lines) do
    new_lines = lines 
      |> Enum.map(&String.to_charlist/1) 
      |> transpose() 
      |> Enum.map(&Enum.reverse/1) 
      |> Enum.map(&List.to_string/1)
      |> Enum.map(&String.trim/1)
      |> then(&split_by_empty_lines/1)

    new_lines
  end

  defp split_by_empty_lines([]), do: []
  defp split_by_empty_lines(lines) do
    {head, combined_tail} = Enum.split_while(lines, &(&1 != ""))
    case combined_tail do
      [] -> [head]
      [_ | tail] -> [head | split_by_empty_lines(tail)]
    end
  end

  defp parse1(lines) do
    [operators | tail_reversed] = Enum.reverse(lines)

    numbers =
      tail_reversed
      |> Enum.map(fn line -> breakup(line, &String.to_integer/1) end)

    bundle(breakup(operators), transpose(numbers))
  end

  defp bundle([], []), do: []
  defp bundle([o | ot], [n | nt]), do: [{o, n} | bundle(ot, nt)]

  defp transpose([]), do: []
  defp transpose(items) do
    pairs = for [h | t] <- items, do: {h, t}
    heads = for {h, _} <- pairs, do: h
    tails = for {_, t} <- pairs, do: t
    [first_tail | _] = tails
    case first_tail do
      [] -> [heads]
      _ -> [heads | transpose(tails)]
    end
  end

  defp breakup(line), do: breakup(line, &(&1))
  defp breakup(line, line_parser) do
    line
    |> String.split(" ")
    |> Enum.reject(&(&1 == ""))
    |> Enum.map(line_parser)
  end
end
