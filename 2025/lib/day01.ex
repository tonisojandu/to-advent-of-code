defmodule Aoc2025.Day01 do
  def is_zero(0), do: 1
  def is_zero(_), do: 0

  def get_direction("L"), do: -1
  def get_direction(_), do: 1

  def count_pause_on_zero(current, []), do: is_zero(current)

  def count_pause_on_zero(current, [head | tail]) do
    {dir, rest} = String.split_at(head, 1)

    with {amount, ""} <- Integer.parse(rest) do
      new_current = Integer.mod(current + get_direction(dir) * amount, 100)
      count_pause_on_zero(new_current, tail) + is_zero(current)
    else
      :error -> {:error, "Could not parse int: #{rest}"}
    end
  end

  def tick(current, _direction, 0), do: {current, is_zero(current)}

  def tick(current, direction, left) do
    new_current = Integer.mod(current + direction, 100)
    {return_current, zeros} = tick(new_current, direction, left - 1)
    {return_current, zeros + is_zero(current)}
  end

  def count_touch_zero(current, []), do: {current, is_zero(current)}

  def count_touch_zero(current, [head | tail]) do
    {dir, rest} = String.split_at(head, 1)

    with {amount, ""} <- Integer.parse(rest) do
      {new_current, result} = tick(current, get_direction(dir), amount)
      {final_pos, tail_result} = count_touch_zero(new_current, tail)
      {final_pos, result - is_zero(new_current) + tail_result}
    else
      :error -> {:error, "Could not parse int: #{rest}"}
    end
  end

  def solve(lines) do
    {count_pause_on_zero(50, lines), elem(count_touch_zero(50, lines), 1)}
  end
end
