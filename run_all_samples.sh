cargo build --release
for test in $(ls samples)
do
    echo "running: $ dc samples/$test"
    target/release/dc samples/$test
    echo 
done
