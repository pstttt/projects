if (length(System.argv()) > 0) do
    file = hd(System.argv());

    case File.read(file) do
        {:ok, content} -> IO.puts(content);
        {:error, reason} ->
            description = List.to_string(:file.format_error(reason));
            IO.puts(String.upcase(String.first(description)) <> String.slice(description, 1..-1));
    end
else
    IO.puts("File not provided");
end
