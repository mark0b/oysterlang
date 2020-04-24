mod tokenize {

    use crate::tokens;

    #[test]
    fn test_with_padding() {
        let s = String::from("  1  ");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0], tokens::Token::Num(String::from("1")));
    }

    #[test]
    fn test_single_integer() {
        let s = String::from("1");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0], tokens::Token::Num(s));
    }

    #[test]
    fn test_adding_integers() {
        let s = String::from("1 + 2");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 3);
        assert_eq!(ts[0], tokens::Token::Num(String::from("1")));
        assert_eq!(ts[1], tokens::Token::Plus);
        assert_eq!(ts[2], tokens::Token::Num(String::from("2")));
    }

    #[test]
    fn test_str_literal() {
        let s = String::from("\"my string 1\"");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0], tokens::Token::Str(s));
    }

    #[test]
    fn test_float_literal() {
        let s = String::from("1.72");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0], tokens::Token::Num(s));
    }

    mod paths {
        use super::*;

        fn test_path(s: &str) {
            let path = String::from(s);
            let ts = tokens::tokenize(&path).unwrap();
            println!("{:?}", ts);
            assert_eq!(ts.len(), 1);
            assert_eq!(ts[0], tokens::Token::Path(path));
        }

        #[test]
        fn test_multi_path() {
            let test_paths: Vec<&str> = vec![
                r"r.ext",
                r"ruokdsho.ps1",
                r"file.c",
                r".gitignore",
                r"rust.rs",
                r"something/something/something.txt",
                r"/thing/text.txt",
                r"C:/this/that.txt",
                r"~/home/.txt",
                r"../home/file.t",
                r"./dir.something",
                r"the_file.txt",
                r"the-file.txt",
            ];
            for path in test_paths.iter() {
                test_path(path);
            }
        }
    }

    #[test]
    fn test_param() {
        let params = vec![
            String::from("--parameter"),
            String::from("-r"),
            String::from("--param-eter"),
            String::from("-param"),
        ];
        for par in params.iter() {
            let ts = tokens::tokenize(par).unwrap();
            assert_eq!(ts.len(), 1);
            assert_eq!(ts[0], tokens::Token::Param(String::from(par)));
        }
    }
}

mod parse {

    use crate::parser::{self, Expr, Prog};
    use crate::tokens::Token;
    use parser::Stmt;

    #[test]
    fn test_single_integer() {
        let ts = [Token::Num(String::from("1")), Token::NewLine];
        let res = parser::parse(&ts);

        match res {
            Ok(Prog::Stmt(box Stmt::Expr(Expr::Num(n)), box Prog::End)) => assert_eq!(n, 1.0),
            _ => unreachable!(),
        }
    }

    #[test]
    fn parsing_commands() {
        let ts = vec![
            Token::Path(String::from(".\\this\\is\\a\\path.txt")),
            Token::Str(String::from("something_else")),
            Token::Path(String::from(".\\this\\is\\a\\path.txt")),
            Token::Param(String::from("-parameter")),
            Token::Str(String::from("something_else")),
            Token::Num(String::from("0.0")),
            Token::Str(String::from("something_else")),
            Token::Num(String::from("0.0")),
            Token::Param(String::from("-parameter")),
            Token::NewLine,
        ];

        let mut count_str_tok = 0;
        let mut count_param_tok = 0;
        let mut count_path_tok = 0;
        let mut count_num_tok = 0;

        for tok in ts.iter() {
            match tok {
                Token::Str(_) => count_str_tok += 1,
                Token::Param(_) => count_param_tok += 1,
                Token::Path(_) => count_path_tok += 1,
                Token::Num(_) => count_num_tok += 1,
                _ => (),
            }
        }

        let res = parser::parse(&ts);

        let mut count_str = 0;
        let mut count_param = 0;
        let mut count_path = 0;
        let mut count_num = 0;

        match res {
            Ok(Prog::Stmt(box Stmt::Expr(Expr::Command(box Expr::Path(s), v)), box Prog::End)) => {
                assert_eq!(v.len(), ts.len() - 2);
                assert_eq!(s, String::from(".\\this\\is\\a\\path.txt"));
                for ex in v.iter() {
                    match ex {
                        Expr::Str(_) => count_str += 1,
                        Expr::Param(_) => count_param += 1,
                        Expr::Path(_) => count_path += 1,
                        Expr::Num(_) => count_num += 1,
                        _ => panic!("There was something unexpected in the vector."),
                    }
                }
            }
            _ => unreachable!(),
        }
        assert_eq!(count_str, count_str_tok);
        assert_eq!(count_param, count_param_tok);
        assert_eq!(count_path, count_path_tok - 1);
        assert_eq!(count_num, count_num_tok);
    }
}

mod interpret {
    use crate::{
        interpreter,
        parser::{Expr, Prog, Stmt},
    };

    #[test]
    fn test_single_integer() {
        let prog = Prog::Stmt(box Stmt::Expr(Expr::Num(1.0)), box Prog::End);
        match interpreter::interpret(&prog) {
            Ok(out) => assert_eq!(out, "1\n"),
            _ => unreachable!(),
        }
    }
}

mod eval {

    use crate::eval;

    fn assert_eval(input: &str, expected: &str) {
        match eval(input) {
            Ok(s) => assert_eq!(s, expected),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_with_padding() {
        assert_eval("  1  \n ", "1\n")
    }

    #[test]
    fn test_single_integer() {
        assert_eval("1\n", "1\n")
    }

    #[test]
    fn test_adding_integers() {
        assert_eval("1 + 3\n", "4\n")
    }

    #[test]
    fn test_multiplying_integers() {
        assert_eval("2 * 3\n", "6\n")
    }

    #[test]
    fn test_dividing_integers() {
        assert_eval("1 / 4\n", "0.25\n")
    }

    #[test]
    fn test_parens() {
        assert_eval("(1 + 2)\n", "3\n")
    }

    #[test]
    fn test_parens_after_integer() {
        assert_eval("1 + (2 + 3) + 4\n", "10\n")
    }

    #[test]
    fn test_associative() {
        assert_eval("1.0 / 2\n", "0.5\n");
        assert_eval("1.0 / 2 / 2\n", "0.25\n");
        assert_eval("1.0 / 2 / 2 / 2\n", "0.125\n");
    }

    #[test]
    fn test_addition_with_parens() {
        assert_eval("1 - (2 + 7) + 4\n", "-4\n")
    }

    #[test]
    fn test_math_expr() {
        // assert_eval("1 + 7*(9 - 2) % 5 / 10\n", "1.4\n")
        assert_eval("1 + 7 * (9 - 2) % 5 / 10\n", "1.4\n")
    }

    #[test]
    fn test_multiline() {
        assert_eval("1 + 1\n2 + 2\n3 + 3\n", "2\n4\n6\n")
    }

    #[test]
    fn test_assign_is_void() {
        assert_eval("$a = 1 + 2\n", "")
    }

    #[test]
    fn test_vars() {
        assert_eval("$a = 1 + 1\n$a = $a + 1\n$a\n", "3\n")
    }
}
