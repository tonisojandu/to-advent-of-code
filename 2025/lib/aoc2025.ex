defmodule Aoc2025 do
  def pull_content(day) do
    data_dir = Path.expand("~/.aoc")
    session_file = Path.join(data_dir, "session")
    input_file = Path.join([data_dir, "inputs", "2025", "#{day}-1.in"])

    File.mkdir_p!(Path.dirname(input_file))
    File.touch!(session_file)

    with {:error, _} <- File.read(input_file),
         {:ok, token} <- File.read(session_file),
         token = String.trim(token),
         :ok <- if(token == "", do: {:error, :empty_token}, else: :ok),
         {:ok, body} <- fetch_input(day, token) do
      File.write!(input_file, body)
      {:ok, body}
    else
      {:ok, cached} -> {:ok, cached}
      {:error, :empty_token} -> {:error, "Session token missing, please write it to #{session_file}"}
      {:error, reason} -> {:error, reason}
    end
  end

  defp fetch_input(day, token) do
    headers = [
      {~c"Cookie", String.to_charlist(token)},
      {~c"User-Agent", ~c"my-elixir-aoc-solution-should-pulls-once"}
    ]
    url = ~c"https://adventofcode.com/2025/day/#{day}/input"
    http_options = [timeout: 5000, ssl: [verify: :verify_none]]

    with {:ok, {{_, 200, _}, _, body}} <- :httpc.request(:get, {url, headers}, http_options, []) do
      {:ok, to_string(body)}
    else
      {:ok, {{_, status, _}, _, _}} -> {:error, "Got #{status} from server"}
      {:error, reason} -> {:error, reason}
    end
  end

  def pull_lines(day) do
    with {:ok, content} <- pull_content(day) do
      lines =
        content
        |> String.split("\n", trim: true)
        |> Enum.map(&String.trim/1)

      {:ok, lines}
    end
  end

  def solve(day) do
    module = Module.concat(Aoc2025, "Day#{String.pad_leading(to_string(day), 2, "0")}")

    with {:ok, lines} <- pull_lines(day),
         {:module, _} <- Code.ensure_loaded(module),
         true <- function_exported?(module, :solve, 1) do
      module.solve(lines)
    else
      {:error, :nofile} -> {:error, "Module #{module} not found"}
      false -> {:error, "#{module}.solve/1 not defined"}
      {:error, reason} -> {:error, reason}
    end
  end
end
