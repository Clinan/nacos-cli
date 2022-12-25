pub mod input_helper {

    use std::borrow::Cow::{self, Borrowed, Owned};

    use rustyline::completion::FilenameCompleter;
    use rustyline::error::ReadlineError;
    use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
    use rustyline::hint::HistoryHinter;
    use rustyline::validate::MatchingBracketValidator;
    use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent};
    use rustyline_derive::{Completer, Helper, Hinter, Validator};

    const HISTORY_FILE: &str = "history.txt";
    #[derive(Helper, Completer, Hinter, Validator)]
    pub struct InputHelper {
        #[rustyline(Completer)]
        completer: FilenameCompleter,
        highlighter: MatchingBracketHighlighter,
        #[rustyline(Validator)]
        validator: MatchingBracketValidator,
        #[rustyline(Hinter)]
        hinter: HistoryHinter,
        colored_prompt: String,
    }

    impl Highlighter for InputHelper {
        fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
            &'s self,
            prompt: &'p str,
            default: bool,
        ) -> Cow<'b, str> {
            if default {
                Borrowed(&self.colored_prompt)
            } else {
                Borrowed(prompt)
            }
        }

        fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
            Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
        }

        fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
            self.highlighter.highlight(line, pos)
        }

        fn highlight_char(&self, line: &str, pos: usize) -> bool {
            self.highlighter.highlight_char(line, pos)
        }
    }

    // To debug rustyline:
    // RUST_LOG=rustyline=debug cargo run --example example 2> debug.log
    pub fn get_enditor() -> Editor<InputHelper> {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build();
        let h = InputHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        };
        let mut rl = Editor::with_config(config).unwrap();
        rl.set_helper(Some(h));
        rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
        rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
        if rl.load_history(HISTORY_FILE).is_err() {}
        return rl;
    }

    pub fn readline(rl: &mut Editor<InputHelper>) -> Result<String, ()> {
        let p = format!(">> ");
        rl.helper_mut().unwrap().colored_prompt = format!("\x1b[1;32m{p}\x1b[0m");
        let readline = rl.readline(&p);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                Ok(line.as_str().to_string())
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                Err(())
            }
            Err(ReadlineError::Eof) => {
                println!("Encountered Eof");
                Err(())
            }
            Err(err) => {
                println!("Error: {err:?}");
                Err(())
            }
        }
    }

    #[allow(unused)]
    pub fn append_history(rl: &mut Editor<InputHelper>) {
        rl.append_history(HISTORY_FILE);
    }
}
