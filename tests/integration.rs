extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;

    #[test]
    fn launching_game_exit() {
        assert_cli::Assert::main_binary()
            .stdout().contains("Please Input your Guess.")
            .stdin("exit\n")
            .stdout().contains("Thanks for playing!")
            .unwrap();
    }

    #[test]
    fn launching_game_exit_two() {
        let test = assert_cli::Assert::command(&["cargo", "run"])
            .stdout().contains("Please Input your Guess.")
            .stdin("exit\n")
            .stdout().contains("Thanks for playing!")
            .execute();
        assert!(test.is_ok());
    }
}