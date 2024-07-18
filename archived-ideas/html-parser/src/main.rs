use scraper::ElementRef;
use std::fs;
fn main() {
    let html_content = fs::read_to_string("index.html").expect("Unable to read file");

    let modified_html = modify_html(&html_content);
    dbg!(&modified_html);
    write_to_file(&modified_html, "index_new.html");
}

fn modify_html(html_content: &str) -> String {
    use scraper::Html;
    let mut output = "<!doctype html>\n".to_string();
    let document = Html::parse_document(html_content);
    let root = document.root_element();
    // output.push_str(string);
    output.push_str(&render_html_element(root));
    // dbg!(root.value());

    for element in root.child_elements() {
        if element.value().name() == "head" {
            output.push_str(&render_html_element(element));
            for el in element.child_elements() {
                output.push_str(&el.html());
            }
            output.push_str("</head>");
        } else if element.value().name() == "body" {
            output.push_str(&render_html_element(element));

            for el in element.child_elements() {
                output.push_str(&el.html());
            }
            output.push_str("</body>");
        } else {
            dbg!(element.value());
        }

        // dbg!(element);
    }
    output.push_str("</html>");
    output
}

fn write_to_file(modified_html: &str, file_path: &str) {
    fs::write(file_path, modified_html).expect("Unable to write file");
}

fn render_html_element(element: ElementRef) -> String {
    let mut output = "".to_string();
    let name = element.value().name();
    output.push('<');
    output.push_str(name);

    let attrs = element
        .value()
        .attrs()
        .map(|(k, v)| format!(r#"{k}="{v}""#))
        .collect::<Vec<_>>();
    let classes = element.value().classes().collect::<Vec<_>>();

    if attrs.len() > 0 || classes.len() > 0 {
        output.push(' ');
    }

    for attr in attrs {
        output.push_str(&attr);
    }

    if classes.len() > 0 {
        for class in classes {
            output.push_str(class);
        }
    }
    output.push('>');
    output
}
