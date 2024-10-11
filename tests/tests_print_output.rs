#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_print_output() {
        let args = vec![
            detox::OptionnalFields {
                options: detox::OptionsFields { dry_run: false },
                verbosity: detox::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            },
            detox::OptionnalFields {
                options: detox::OptionsFields { dry_run: true },
                verbosity: detox::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            },
            detox::OptionnalFields {
                options: detox::OptionsFields { dry_run: true },
                verbosity: detox::VerbosityFields {
                    verbose: false,
                    json: true,
                    json_pretty: false,
                    json_error: false,
                },
            },
            detox::OptionnalFields {
                options: detox::OptionsFields { dry_run: true },
                verbosity: detox::VerbosityFields {
                    verbose: false,
                    json: true,
                    json_pretty: false,
                    json_error: true,
                },
            },
            detox::OptionnalFields {
                options: detox::OptionsFields { dry_run: true },
                verbosity: detox::VerbosityFields {
                    verbose: false,
                    json: true,
                    json_pretty: true,
                    json_error: false,
                },
            },
            detox::OptionnalFields {
                options: detox::OptionsFields { dry_run: true },
                verbosity: detox::VerbosityFields {
                    verbose: false,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            },
        ];
        for options in args.iter() {
            let paths_to_check = vec![PathBuf::from("README.md")];
            let final_res = detox::detox(options, paths_to_check);
            detox::print_output(&options.verbosity, final_res).unwrap();
        }
    }

    fn setup(name1: &PathBuf, name2: &PathBuf) {
        // create
        let to_correct = PathBuf::from(&name1);
        std::fs::File::create(&to_correct).unwrap();

        // set read only
        let read_only = PathBuf::from(&name2);
        std::fs::File::create(&read_only).unwrap();
        let mut perms = std::fs::metadata(&read_only).unwrap().permissions();
        perms.set_readonly(true);
        std::fs::set_permissions(&read_only, perms).unwrap();
    }

    fn cleanup(name1: &PathBuf, read_only: &PathBuf) {
        // remove
        std::fs::remove_file(&name1).unwrap();

        // remove read only
        let mut perms = std::fs::metadata(&read_only).unwrap().permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(&read_only, perms).unwrap();
        // remove
        std::fs::remove_file(&read_only).unwrap();
    }

    #[test]
    fn test_print_output_verbose_dry() {
        let options = detox::OptionnalFields {
            options: detox::OptionsFields { dry_run: true },
            verbosity: detox::VerbosityFields {
                verbose: true,
                json: false,
                json_pretty: false,
                json_error: false,
            },
        };
        let to_correct = PathBuf::from("tes t verbose dry.txt");
        let read_only = PathBuf::from("test_verbose_dry.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = detox::detox(&options, paths_to_check);
        detox::print_output(&options.verbosity, final_res).unwrap();

        // cleanup
        cleanup(&to_correct, &read_only);
    }

    #[test]
    fn test_print_output_verbose_real() {
        let options = detox::OptionnalFields {
            options: detox::OptionsFields { dry_run: false },
            verbosity: detox::VerbosityFields {
                verbose: true,
                json: false,
                json_pretty: false,
                json_error: false,
            },
        };
        let to_correct = PathBuf::from("tes t verbose.txt");
        let read_only = PathBuf::from("test_verbose.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = detox::detox(&options, paths_to_check);
        detox::print_output(&options.verbosity, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_verbose.txt"), &read_only);
    }

    #[test]
    fn test_print_output_json_real() {
        let options = detox::OptionnalFields {
            options: detox::OptionsFields { dry_run: false },
            verbosity: detox::VerbosityFields {
                verbose: false,
                json: true,
                json_pretty: false,
                json_error: false,
            },
        };
        let to_correct = PathBuf::from("tes t json.txt");
        let read_only = PathBuf::from("test_json.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = detox::detox(&options, paths_to_check);
        detox::print_output(&options.verbosity, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_json.txt"), &read_only);
    }

    #[test]
    fn test_print_output_json_error_real() {
        let options = detox::OptionnalFields {
            options: detox::OptionsFields { dry_run: false },
            verbosity: detox::VerbosityFields {
                verbose: false,
                json: true,
                json_pretty: false,
                json_error: true,
            },
        };
        let to_correct = PathBuf::from("tes t json error.txt");
        let read_only = PathBuf::from("test_json_error.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = detox::detox(&options, paths_to_check);
        detox::print_output(&options.verbosity, final_res).unwrap();

        // cleanup
        cleanup(&PathBuf::from("tes_t_json_error.txt"), &read_only)
    }

    #[test]
    fn test_print_output_json_error_dry() {
        let options = detox::OptionnalFields {
            options: detox::OptionsFields { dry_run: true },
            verbosity: detox::VerbosityFields {
                verbose: false,
                json: true,
                json_pretty: false,
                json_error: true,
            },
        };
        let to_correct = PathBuf::from("tes t json error dry.txt");
        let read_only = PathBuf::from("test_json_error_dry.txt");
        setup(&to_correct, &read_only);

        let paths_to_check = vec![
            PathBuf::from("README.md"),
            to_correct.clone(),
            read_only.clone(),
        ];
        let final_res = detox::detox(&options, paths_to_check);
        detox::print_output(&options.verbosity, final_res).unwrap();

        // cleanup
        cleanup(&to_correct, &read_only)
    }
}
