#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::borrow::Cow;
use std::env;
use std::fs;

use regex::Regex;

fn main() { delete_idea_cache(); }

fn delete_idea_cache() {
    let path_str = env::args().nth(1).expect("Missing");
    let data = fs::read_to_string(&path_str).expect("Unable to read file");
    let data_cow = regex_filter(&data);
    fs::write("/tmp/ws.xml", data_cow.as_bytes());
}

fn regex_filter(data: &str) -> Cow<str> {
    lazy_static! {
        static ref SANITIZING_REGEX: Regex =
        Regex::new(r"\s*<state.*(find|search\.everywhere|run\.anything)\.popup.*(/>|\n.*\s*</state>)").unwrap();
  }
    return SANITIZING_REGEX.replace_all(&data, "");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_filter_test_simple() {
        const BODY: &str = r#"    <state x="2321" y="1028" width="704" height="578" key="find.popup/0.27.3360.2533@0.27.3360.2533" timestamp="1582534698871" />"#;
        assert_eq!(regex_filter(BODY).len(), 0);
    }

    #[test]
    fn regex_filter_test_multiline() {
        const BODY: &str = r#"        <state x="2147" y="650" width="704" height="578" key="find.popup" timestamp="1584638114604">
      <screen x="1920" y="0" width="1080" height="1920" />
    </state>"#;
        assert_eq!(regex_filter(BODY).len(), 0);
    }


    #[test]
    fn regex_filter_test() {
        const BODY: &str = r#"    <state x="2147" y="650" width="704" height="578" key="find.popup" timestamp="1584638114604">
      <screen x="1920" y="0" width="1080" height="1920" />
    </state>
    <state x="2321" y="1028" width="704" height="578" key="find.popup/0.27.3360.2533@0.27.3360.2533" timestamp="1582534698871" />
    <state x="1034" y="528" width="730" height="552" key="find.popup/0.462.1920.1080/1920.0.1080.1920@0.462.1920.1080" timestamp="1582022975020" />
    <state x="2296" y="759" width="704" height="1087" key="find.popup/0.840.1920.1080/1920.0.1080.1920@1920.0.1080.1920" timestamp="1576760322390" />
    <state x="2265" y="844" width="939" height="1571" key="find.popup/1920.0.1440.2560/0.27.1920.1053@1920.0.1440.2560" timestamp="1583849262682" />
    <state x="604" y="168" width="1163" height="578" key="find.popup/1920.0.1440.2560/67.27.1853.1053@67.27.1853.1053" timestamp="1584115773106" />
    <state x="511" y="374" width="1208" height="646" key="find.popup/67.27.1853.1053@67.27.1853.1053" timestamp="1583509243376" />
    <state x="2147" y="650" width="704" height="578" key="find.popup/67.489.1853.1053/1920.0.1080.1920@1920.0.1080.1920" timestamp="1584638114604" />
    <state x="2050" y="484" width="950" height="704" key="find.popup/67.489.2933.1431@67.489.2933.1431" timestamp="1584459125235" />
    <state x="2296" y="759" width="704" height="1087" key="find.popup/67.867.1853.1053/1920.0.1080.1920@1920.0.1080.1920" timestamp="1576754614634" />
    <state x="2093" y="1283" width="863" height="596" key="find.popup/67.867.2933.1053@67.867.2933.1053" timestamp="1576669064174" />
    <state x="2123" y="673" width="663" height="572" key="git4idea.branch.GitSmartOperationDialog" timestamp="1576511297915">
      <screen x="67" y="867" width="2933" height="1053" />
    </state>"#;

        const EXPECTED: &str = r#"
    <state x="2123" y="673" width="663" height="572" key="git4idea.branch.GitSmartOperationDialog" timestamp="1576511297915">
      <screen x="67" y="867" width="2933" height="1053" />
    </state>"#;
        assert_eq!(regex_filter(BODY), EXPECTED);
    }

// #[test]
//   fn test_bad_add() {
//     // This assert would fire and test will fail.
//     // Please note, that private functions can be tested too!
//     assert_eq !(bad_add(1, 2), 3);
//   }
}
