fn main() {
    // generate the lalrpop parser from the grammar file
    lalrpop::process_root().unwrap();
}