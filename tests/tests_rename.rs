#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    const TESTS_FIELDS_NOT_DRY_RUN: detox::OptionnalFields = detox::OptionnalFields {
        options: detox::OptionsFields { dry_run: false },
        verbosity: detox::VerbosityFields {
            verbose: false,
            json: false,
            json_pretty: false,
            json_error: false,
        },
    };

    #[test]
    fn no_rename() {
        let res = detox::detox(
            &TESTS_FIELDS_NOT_DRY_RUN,
            vec![PathBuf::from("my_file").into()],
        );
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].path, PathBuf::from("my_file"));
        assert_eq!(res[0].modified, None);
    }

    #[test]
    fn rename() {
        let path = PathBuf::from("my?..file");
        let res = detox::detox(&TESTS_FIELDS_NOT_DRY_RUN, vec![path.clone().into()]);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].path, path);
        assert_eq!(res[0].modified, Some(PathBuf::from("my_..file")));
    }
    #[test]
    fn rename_ascii() {
        let paths = vec![
            ("my file.ext", "my_file.ext"),
            ("my\"file.ext", "my_file.ext"),
            ("my#file.ext", "my_file.ext"),
            ("my$file.ext", "my_file.ext"),
            ("my%file.ext", "my_file.ext"),
            ("my&file.ext", "my_file.ext"),
            ("my'file.ext", "my_file.ext"),
            ("my(file.ext", "my_file.ext"),
            ("my)file.ext", "my_file.ext"),
            ("my*file.ext", "my_file.ext"),
            // ("my-file.ext", "my_file.ext"),
            // ("my.file.ext", "my_file.ext"),
            // ("my/file.ext", "my_file.ext"),
            ("my:file.ext", "my_file.ext"),
            ("my;file.ext", "my_file.ext"),
            ("my<file.ext", "my_file.ext"),
            ("my=file.ext", "my_file.ext"),
            ("my>file.ext", "my_file.ext"),
            ("my?file.ext", "my_file.ext"),
            ("my@file.ext", "my_file.ext"),
            // ("myAfile.ext", "my_file.ext"),
            // ("myZfile.ext", "my_file.ext"),
            ("my[file.ext", "my_file.ext"),
            (r"my\file.ext", "my_file.ext"),
            ("my]file.ext", "my_file.ext"),
            ("my^file.ext", "my_file.ext"),
            // ("my_file.ext", "my_file.ext"),
            ("my`file.ext", "my_file.ext"),
            // ("myafile.ext", "my_file.ext"),
            // ("myzfile.ext", "my_file.ext"),
            ("my{file.ext", "my_file.ext"),
            ("my|file.ext", "my_file.ext"),
            ("my}file.ext", "my_file.ext"),
            ("my~file.ext", "my_file.ext"),
            ("my\u{007F}file.ext", "my_file.ext"),
        ];
        for one_test in paths.iter() {
            let path_to_test = PathBuf::from(one_test.0);
            let result_to_test = PathBuf::from(one_test.1);
            print!("Testing: {:?} -> {:?}\n", path_to_test, result_to_test);
            let res = detox::detox(&TESTS_FIELDS_NOT_DRY_RUN, vec![path_to_test.clone().into()]);
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].path, path_to_test.to_owned());
            assert_eq!(res[0].modified, Some(result_to_test));
        }
    }

    #[test]
    fn ascii_should_not_rename() {
        let paths = vec![
            "my-file.ext",
            "my.file.ext",
            "my/file.ext",
            "my0file.ext",
            "my9file.ext",
            "myAfile.ext",
            "myZfile.ext",
            "my_file.ext",
            "myafile.ext",
            "myzfile.ext",
        ];
        for one_test in paths.iter() {
            let path_to_test = PathBuf::from(one_test);
            print!("Testing: {:?}\n", path_to_test);
            let res = detox::detox(&TESTS_FIELDS_NOT_DRY_RUN, vec![path_to_test.clone().into()]);
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].path, path_to_test.to_owned());
            assert_eq!(res[0].modified, None);
        }
    }

    fn is_allowed_but_changed(number: i32) -> (bool, String) {
        let mut acc: String = String::new();
        let unicode_char = std::char::from_u32(number as u32);
        if unicode_char.is_none() {
            return (false, acc);
        }
        let unicode_char = unicode_char.unwrap();
        let utf8_bytes: Vec<u8> = unicode_char.to_string().into_bytes();
        detox::check_similar(utf8_bytes, &mut acc, false);
        if acc == "_" {
            return (false, acc);
        }
        return (true, acc);
    }

    #[test]
    fn non_ascci_rename() {
        let allow_no_change: Vec<i32> = vec![95]
            .into_iter()
            .chain((45..=47).collect::<Vec<_>>())
            .chain((48..=57).collect::<Vec<_>>()) // numbers
            .chain((65..=90).collect::<Vec<_>>()) // uppercase
            .chain((97..=122).collect::<Vec<_>>()) // lowercase
            .collect();
        let diacritics: Vec<i32> = vec![]
            .into_iter()
            .chain((768..=879).collect::<Vec<_>>()) // diacritics  \u{0300} to \u{036F}
            .chain((6832..=6911).collect::<Vec<_>>()) // diacritics  \u{1AB0} to \u{1AFF}
            .chain((7616..=7679).collect::<Vec<_>>()) // diacritics  \u{1DC0} to \u{1DFF}
            .collect();
        let max_unicode_point = 0x10FFFF;
        for index in 0..(max_unicode_point + 1) {
            let correct_path;
            let unicode_point = index;
            let options = std::char::from_u32(unicode_point as u32);
            if options.is_none() {
                print!("Invalid: {:?}\n", index);
                continue;
            }
            let options = options.unwrap();
            let path_to_test = PathBuf::from(format!("my{}file.ext", options));
            let (boo, acc) = is_allowed_but_changed(index);
            if allow_no_change.contains(&index) {
                correct_path = None;
            } else if diacritics.contains(&index) {
                correct_path = Some(PathBuf::from("myfile.ext"));
            } else if boo {
                // here format
                let corrected = format!("my{}file.ext", acc);
                let corrected = PathBuf::from(corrected);
                print!(
                    "Changed: {:?} ({:?}) ({}) -> {:?}\n",
                    path_to_test,
                    index,
                    options.escape_unicode(),
                    corrected
                );
                correct_path = Some(corrected);
            } else {
                correct_path = Some(PathBuf::from("my_file.ext"));
            }
            print!(
                "Testing: {:?} ({:?}) ({})\n",
                path_to_test,
                index,
                options.escape_unicode()
            );
            let res = detox::detox(&TESTS_FIELDS_NOT_DRY_RUN, vec![path_to_test.clone().into()]);
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].path, path_to_test.to_owned());
            assert_eq!(res[0].modified, correct_path);
        }
    }
}
