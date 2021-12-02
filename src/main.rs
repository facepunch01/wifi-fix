use run_script::ScriptOptions;

fn main() {
    let options = ScriptOptions::new();

    let args = vec![];

    // run the script and get the script execution output
    let (code, _output, error) = run_script::run(
        r#"
         curl https://web.archive.org/web/20201111213909/http://www.killernetworking.com/support/K1535_Debian/board.bin >> board.bin
         curl https://raw.githubusercontent.com/linux-surface/ath10k-firmware-override/main/board-2.bin >> board-2.bin
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board.bin
         cp board.bin /lib/firmware/ath10k/QCA6174/hw2.1/board.bin
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin
         cp board-2.bin /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board.bin
         cp board.bin /lib/firmware/ath10k/QCA6174/hw3.0/board.bin
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin
         cp board-2.bin /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin
         rm board.bin
         rm board-2.bin
         "#,
        &args,
        &options,
    )
        .unwrap();

    println!("Exit Code: {}", code);
    println!("Error: {}", error);

    // run the script and get a handle to the running child process
    let child = run_script::spawn(
        r#"
         curl https://web.archive.org/web/20201111213909/http://www.killernetworking.com/support/K1535_Debian/board.bin >> board.bin
         curl https://raw.githubusercontent.com/linux-surface/ath10k-firmware-override/main/board-2.bin >> board-2.bin
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board.bin
         cp board.bin /lib/firmware/ath10k/QCA6174/hw2.1/board.bin
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin
         cp board-2.bin /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board.bin
         cp board.bin /lib/firmware/ath10k/QCA6174/hw3.0/board.bin
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin
         cp board-2.bin /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin
         rm board.bin
         rm board-2.bin
         "#,
        &args,
        &options,
    )
        .unwrap();

    let spawn_output = child.wait_with_output().unwrap();

    println!("Success: {}", &spawn_output.status.success());
}
