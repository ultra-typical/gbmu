for test in `find intra-test | grep gb | grep -v .save`; do 
cargo run -- $test
done

