for file in $(pwd)/*.rs; do
    echo "Compiling $file"
    rustc "$file"
done
