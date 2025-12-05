defmodule Aoc2025 do
  def pull_content(day, input) do
    data_dir = Path.expand("~/.aoc")
    session_file = Path.join(data_dir, "session")
    input_file = Path.join([data_dir, "inputs", "2025", "#{day}-#{input}.in"])

    File.mkdir_p!(Path.dirname(input_file))
    File.touch!(session_file)

    case File.read(input_file) do
      {:ok, cached} ->
        {:ok, cached}

      {:error, _} ->
        with {:ok, token} <- File.read(session_file),
             token = String.trim(token),
             :ok <- if(token == "", do: {:error, :empty_token}, else: :ok),
             {:ok, body} <- fetch_input(day, input, token) do
          File.write!(input_file, body)
          {:ok, body}
        else
          {:error, :empty_token} -> {:error, "Session token missing, please write it to #{session_file}"}
          {:error, reason} -> {:error, reason}
        end
    end
  end

  defp fetch_input(day, input, token) do
    # Ensure HTTP client is started
    :inets.start()
    :ssl.start()

    headers = [
      {~c"Cookie", String.to_charlist(token)},
      {~c"User-Agent", ~c"my-elixir-aoc-solution-should-pulls-once"}
    ]
    url = ~c"https://adventofcode.com/2025/day/#{day}/input"
    http_options = [timeout: 5000, ssl: [verify: :verify_none]]

    with {:ok} <- (if input == 1, do: {:ok}, else: {:error, "Cannot pull input #{input} != 1 . Insert it manually"}),
         {:ok, {{_, 200, _}, _, body}} <- :httpc.request(:get, {url, headers}, http_options, []) do
         {:ok, to_string(body)}
    else
      {:ok, {{_, status, _}, _, _}} -> {:error, "Got #{status} from server"}
      {:error, reason} -> {:error, reason}
    end
  end

  defp pull_lines(day, input) do
    with {:ok, content} <- pull_content(day, input) do
      lines =
        content
        |> String.split("\n")
        |> drop_last_empty()
      {:ok, lines}
    end
  end

  defp drop_last_empty(lines) do
    lines
    |> Enum.reverse()
    |> then(fn [h | t] -> 
      case String.trim(h) do 
        "" -> t
        _ -> [h | t]
      end 
    end)
    |> Enum.reverse()
  end

  def solve(day, input) do
    module = Module.concat(Aoc2025, "Day#{String.pad_leading(to_string(day), 2, "0")}")

    with {:ok, lines} <- pull_lines(day, input),
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
