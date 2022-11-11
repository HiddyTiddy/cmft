#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn examples() -> Result<(), i32> {
        let paths = fs::read_dir("./tests/examples/").unwrap();
        let mut failed_cases : Vec<String>= Vec::new();
        for path in paths {
            let path = path.unwrap().path();
            let file_name_in = path.to_str().unwrap();
            if !file_name_in.ends_with(".in") {
                continue;
            }
            let file_name_out = file_name_in.replace(".in", ".out");
            let input  = fs::read_to_string(file_name_in).unwrap();
            let output = fs::read_to_string(file_name_out).unwrap();
            let result = cmft::format_string(input);
            if !result.eq(&output)
            {
                failed_cases.push(file_name_in.to_string());
            }
        }
        
        if failed_cases.len() != 0
        {
            println!("Failed Examples:\n");
            for e in &failed_cases
            {
                println!("{}",e);
            }
            let n = failed_cases.len();
            let nu = n.try_into().unwrap();
            return Err(nu);
        }
        return Ok(());
    }
}
