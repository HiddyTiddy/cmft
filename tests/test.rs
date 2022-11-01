mod tests {
    use std::fs;
    use std::process::Command;

    #[test]
    fn examples() {
        let paths = fs::read_dir("./tests/examples/").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let file_name_in = path.to_str().unwrap();
            if !file_name_in.ends_with(".in") {
                continue;
            }
            let file_name_out = file_name_in.replace(".in", ".out");
            let command = Command::new("echo").arg("Hi").output().expect("lmao");
            let input = fs::read_to_string(file_name_in).unwrap();
            let output = fs::read_to_string(file_name_out).unwrap();
            println!("{}", input);
            println!("{}", output);
            let result = cmft::format_string(input);
            assert_eq!(output, result)
        }
    }
}
