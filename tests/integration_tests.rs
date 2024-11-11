mod testenv;

use crate::testenv::TestEnv;

#[test]
fn it_returns_the_byte_word_and_char_counts_by_default() {
    let env = TestEnv::new("This is some test file input");

    let args = [env.temp_file_path()];
    // expect 1 line, 6 words, 28 characters
    let expected_output = format!("1 6 28 {filename}\n", filename = env.temp_file_path());

    env.assert_output(&args, &expected_output);
}
