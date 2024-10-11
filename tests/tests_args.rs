#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_args() {
        let args = vec![
            (
                vec!["detox".to_owned(), "README.md".to_owned()],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: true },
                    verbosity: detox::VerbosityFields {
                        verbose: true,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["detox".to_owned(), "README.md".to_owned(), "-d".to_owned()],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: false },
                    verbosity: detox::VerbosityFields {
                        verbose: true,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "detox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-j".to_owned(),
                ],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: false },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "detox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json".to_owned(),
                ],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: false },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "detox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-e".to_owned(),
                ],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: false },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: true,
                    },
                },
            ),
            (
                vec![
                    "detox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json-error".to_owned(),
                ],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: false },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: true,
                    },
                },
            ),
            (
                vec![
                    "detox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-p".to_owned(),
                ],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: false },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "detox".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json-pretty".to_owned(),
                ],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: false },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["detox".to_owned(), "-v".to_owned()],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: true },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["detox".to_owned(), "--version".to_owned()],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: true },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["detox".to_owned(), "-q".to_owned()],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: true },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["detox".to_owned(), "--quiet".to_owned()],
                detox::OptionnalFields {
                    options: detox::OptionsFields { dry_run: true },
                    verbosity: detox::VerbosityFields {
                        verbose: false,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
        ];
        print!("ARGS: {:?}\n", args);
        for one_test in args.iter() {
            let res = detox::parse_args(one_test.0.clone());
            if let Ok(ok_res) = res {
                assert_eq!(ok_res.0, one_test.1);
            } else {
                assert_eq!(res.err().unwrap(), 1)
            }
        }
    }

    #[test]
    fn test_parse_args_no_file_found() {
        let vec_args = vec![
            "detox".to_owned(),
            "README.md".to_owned(),
            "README".to_owned(),
        ];
        let res = detox::parse_args(vec_args);
        let (options, vect) = res.ok().unwrap();
        assert_eq!(
            options,
            detox::OptionnalFields {
                options: detox::OptionsFields { dry_run: true },
                verbosity: detox::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        assert_eq!(vect.len(), 1);
    }

    #[test]
    fn test_parse_args_star() {
        let vec_args = vec!["detox".to_owned(), "*".to_owned()];
        let res = detox::parse_args(vec_args);
        let (options, vect) = res.ok().unwrap();
        assert_eq!(
            options,
            detox::OptionnalFields {
                options: detox::OptionsFields { dry_run: true },
                verbosity: detox::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        let number = std::fs::read_dir(".")
            .ok()
            .unwrap()
            .filter(|entry| entry.is_ok())
            .count();
        assert_eq!(vect.len(), number);
    }
}
