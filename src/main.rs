use std::io::Write;

fn main() {
    for i in 0..100000 {
        let file = std::fs::File::create(format!("r/{}.html", i)).unwrap();

        let content = format!(
            "<!DOCTYPE html>
<html>
<head>
<meta http-equiv=refresh content=\"0; url=https://github.com/bitcoin/bitcoin/issues/{}\">
</head>
<body>
</body>
</html>",
            i
        );
        std::io::BufWriter::new(&file)
            .write(&content.as_bytes())
            .unwrap();
    }
}
