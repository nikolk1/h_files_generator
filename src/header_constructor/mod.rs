pub mod header_constructor {
    use regex::Regex;
    use std::fs;
    use std::path::{Path, PathBuf};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref FUNCTION_DECLERATION: Regex = Regex::new(
            r"(?m)(^(?:(?:unsigned )?(?:void|int|float|double|short|long|char) \w+[\d\w]* ?\((?:(?:unsigned )?(?:void|int|float|double|short|long|char) \w+[\d\w]*)*\))$)",
        )
        .unwrap();
        static ref INCLUDE_STATEMENT: Regex = Regex::new(r"(?m)(^#include <.*>$\n)").unwrap();
        static ref DEFINE_STATEMENT: Regex = Regex::new(r"(?m)(^#define .* .*$\n)").unwrap();
        static ref STRUCT: Regex = Regex::new(r"(?m)(^struct .*[\s|\n]{1}\{\n?\s?.*\s?\})").unwrap();
    }

    pub struct HeaderConstructor {
        c_content: String,
        h_content: String,
        c_path: Box<PathBuf>,
    }

    impl HeaderConstructor {
        /// Constructs a new `HeaderConstructor`.
        pub fn new(c_path: &str) -> HeaderConstructor {
            let mut new_header = HeaderConstructor {
                c_content: String::from(""),
                h_content: String::from(""),
                c_path: Box::new(Path::new(c_path).to_path_buf()),
            };
            new_header.load_file();
            new_header
        }

        fn load_file(&mut self) {
            self.c_content =
                fs::read_to_string((*self.c_path).to_str().unwrap()).expect("Something went wrong while reading the file");
        }

        /// Functions that extract content by regex
        fn extract_includes(&mut self, flag: bool) { if flag {self.extract_text_by_regex(&INCLUDE_STATEMENT, "", true)};}
        fn extract_defines(&mut self, flag: bool) {if flag {self.extract_text_by_regex(&DEFINE_STATEMENT, "", true)};}
        fn extract_structs(&mut self, flag: bool) {if flag {self.extract_text_by_regex(&STRUCT, "\n", true)};}
        fn extract_functions_decleration(& mut self) { self.extract_text_by_regex(&FUNCTION_DECLERATION, ";\n", false);}
        

        /// Create header file
        pub fn modify_content(&mut self, i: bool, s: bool, d: bool) {

            let h_path: String = self.get_h_path();
            // TODO: add support for linux/windows slashes and consider using Path objects
            
            self.extract_includes(i);
            self.extract_defines(d);
            self.extract_structs(s);
            self.extract_functions_decleration();
            
            let header_include = format!("#include \"{}.h\"", self.c_path.file_stem().unwrap().to_str().unwrap());
            self.c_content = format!("{}\n{}", header_include, self.c_content);
            
            self.write_to_fs(&h_path);
        }


        fn write_to_fs(&mut self, path: &str) {
            fs::write(*self.c_path.clone(), &self.c_content).unwrap();
            fs::write(path, &self.h_content).unwrap();
        }

        fn extract_text_by_regex(&mut self, pattern: &Regex, sep: &str, remove: bool) {
            let re = &pattern;
            let mut matches: Vec<String> = vec![];
            let mut _new_c_content = self.c_content.clone();

            for m in re.find_iter(&self.c_content) {
                let cap = m.as_str();
                matches.push(String::from(cap));
                if remove {
                    _new_c_content = _new_c_content.replace(cap, "");
                }
            }

            self.c_content = _new_c_content;
            let content = (matches.join(&sep).to_owned()) + sep + "\n";
            self.h_content.push_str(&content);
        }

        fn get_h_path(&self) -> String {
            let mut path = self.c_path.clone();
            path.set_extension("h");
            String::from(path.to_str().unwrap())
        }
    }
}
