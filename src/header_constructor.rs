pub mod header_constructor {

    use regex::Regex;
    use std::fs;
    use std::path::Path;

    pub struct HeaderConstructor {
        c_content: String,
        h_content: String,
        c_path: String,
    }

    impl HeaderConstructor {
        pub fn new(c_path: &str) -> HeaderConstructor {
            let mut new_header = HeaderConstructor {
                c_content: String::from(""),
                h_content: String::from(""),
                c_path: String::from(c_path),
            };
            new_header.load_file();
            new_header
        }
        pub fn generate_header(&mut self, i: bool, s: bool, d: bool) {
            let h_path: String = self.generate_h_path(String::from(&self.c_path));
            // TODO: add support for linux/windows slashes and consider using Path objects
            let new_filename = format!("#include \"{}.h\"\n", &self.c_path[self.c_path.rfind('/').unwrap() + 1..self.c_path.len() - 2]);
            //TODO: change to hashmap
            if i {
                self.extract_includes()
            };
            if d {
                self.extract_defines()
            };
            if s {
                self.extract_structs()
            };
            self.extract_function_signatures();
            self.c_content = format!("{}{}", new_filename, self.c_content);
            self.create_header_file(&h_path);
        }

        fn load_file(&mut self) {
            // TODO: hadle the exception here
            self.c_content =
                fs::read_to_string(&self.c_path).expect("Something went wrong reading the file")
        }

        fn extract_function_signatures(&mut self) {
            let re = Regex::new(
                r"(?m)(^(?:(?:unsigned )?(?:void|int|float|double|short|long|char) \w+[\d\w]* ?\((?:(?:unsigned )?(?:void|int|float|double|short|long|char) \w+[\d\w]*)*\))$)",
            )
            .unwrap();
            self.extract_content(&re, ";\n", false);
        }

        fn create_header_file(&mut self, path: &str) {
            fs::write(&self.c_path, &self.c_content).unwrap();
            fs::write(path, &self.h_content).unwrap();
        }

        fn extract_includes(&mut self) {
            let re = Regex::new(r"(?m)(^#include <.*>$\n)").unwrap();
            self.extract_content(&re, "", true);
        }

        fn extract_defines(&mut self) {
            let re = Regex::new(r"(?m)(^#define .* .*$\n)").unwrap();
            self.extract_content(&re, "", true);
        }

        // only if they dont include methods impl
        fn extract_structs(&mut self) {
            let re = Regex::new(r"(?m)(^struct .*[\s|\n]{1}\{\n?\s?.*\s?\})").unwrap();
            self.extract_content(&re, "\n", true);
        }

        fn extract_content(&mut self, pattern: &Regex, sep: &str, remove: bool) {
            let re = &pattern;
            let mut matches: Vec<String> = vec![];
            let mut _tmp = self.c_content.clone();

            for m in re.find_iter(&self.c_content) {
                let cap = m.as_str();
                matches.push(String::from(cap));
                if remove {
                    _tmp = _tmp.replace(cap, "");
                }
            }
                self.c_content = _tmp;
                self.h_content.push_str(&matches.join(&sep));
                self.h_content.push_str(&sep);
                self.h_content.push_str("\n");
        }

        fn generate_h_path(&mut self, c_path: String) -> String {
            let mut path = Path::new(&c_path).to_path_buf();
            path.set_extension(".h");
            String::from(path.to_str().unwrap())
        }
    }
}
