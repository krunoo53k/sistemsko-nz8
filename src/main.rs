use ftp::FtpStream;
use ftp::FtpError;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut ftp_stream = FtpStream::connect("majnkraft.pizzeriamirakul.hr:21").unwrap_or_else(|err|
        panic!("{}", err)
    );

    let _ = ftp_stream.login("alfonzo", "baumgartner").unwrap();
    
    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    
    ftp_stream.cwd("/files").unwrap();
    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    println!("{:#?}", ftp_stream.list(None));

    let mut filename = String::new();
    println!("Enter the file you'd wish to download: ");
    let _b1 = std::io::stdin().read_line(&mut filename).unwrap();
    let trimmed_filename = filename.trim(); 

    assert!(ftp_stream.retr(&trimmed_filename, |stream| {
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).map(|_|
            {
                let mut file = File::create(&trimmed_filename).expect("Error encountered while creating file!");
                file.write_all(&buf).expect("Failed to write to file!");
            }
        ).map_err(|e| FtpError::ConnectionError(e))
    }).is_ok());

    let _ = ftp_stream.quit();
}
