for test in $(ls samples)
do
    cargo run samples/$test
done
