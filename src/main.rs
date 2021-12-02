use run_script::ScriptOptions;

fn main() {
    let options = ScriptOptions::new();

    let args = vec![];

    // run the script and get the script execution output
    let (code, _output, error) = run_script::run(
        r#"
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board.bin
         cp '/home/jake/Documents/board.bin' /lib/firmware/ath10k/QCA6174/hw2.1/board.bin
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin
         cp '/home/jake/Documents/board.bin' /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board.bin
         cp '/home/jake/Documents/board.bin' /lib/firmware/ath10k/QCA6174/hw3.0/board.bin
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin
         cp '/home/jake/Documents/board.bin' /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin
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
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board.bin
         cp '/home/jake/Documents/board.bin' /lib/firmware/ath10k/QCA6174/hw2.1/board.bin
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin
         cp '/home/jake/Documents/board.bin' /lib/firmware/ath10k/QCA6174/hw2.1/board-2.bin
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board.bin
         cp '/home/jake/Documents/board.bin' /lib/firmware/ath10k/QCA6174/hw3.0/board.bin
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin.xz
         rm /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin
         cp '/home/jake/Documents/board.bin' /lib/firmware/ath10k/QCA6174/hw3.0/board-2.bin
         "#,
        &args,
        &options,
    )
        .unwrap();

    let spawn_output = child.wait_with_output().unwrap();

    println!("Success: {}", &spawn_output.status.success());
}
