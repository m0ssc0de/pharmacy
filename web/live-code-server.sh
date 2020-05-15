inotifywait -m ./ -e close_write  -e moved_to |
    while read path action file; do
        echo "The file '$file' appeared in directory '$path' via '$action'"
        wasm-pack build --target web --out-name wasm --out-dir ./static && miniserve ./static --index index.html
    done
