use std::collections::HashMap;
use crate::dom;

pub struct Parser {
    position: usize,
    data: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.data[self.position..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.data[self.position..].starts_with(s)
    }

    fn end_reached(&self) -> bool {
        self.position >= self.data.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iterator = self.data.char_indices();
        let (_, current_char) = iterator.next().unwrap();
        let (next_position, _) = iterator.next().unwrap_or((1, ' '));
        self.position += next_position;
        return current_char;
    }

    /// This function takes a generic function `test`, which must have the signature of
    /// taking a char parameter as input and returns a bool.
    /// However, the method itself (`consume_while`) takes only this generic function
    /// and returns a String.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.end_reached() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Here, we make use of closures (similar to Python's lambda-functions.
    /// It takes a single character and matches it against one condition.
    /// Either, it is a alphanumeric value, then true is returned.
    /// Otherwise, false is returned.
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => parse_text(),
        }
    }

    /// Text is parsed as long as no `<` sign is hit, which indicates a new tag is reached.
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|character| character != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // Opening tag
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        // Child-nodes
        let children = self.parse_nodes();

        // Closing tag
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        return dom::element(tag_name, attributes, children);
    }

    fn parse_attributes(&mut self) -> dom::AttributeMap {
        let mut attributes: HashMap<String, String> = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attribute();
            attributes.insert(name, value);
        }
        return attributes;
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attribute_value();
        return (name, value);
    }

    fn parse_attribute_value(&mut self) -> String {
        let quote = self.consume_char();
        assert!(quote == '"' || quote == '\'');
        let value = self.consume_while(|character| character != quote);
        assert_eq!(self.consume_char(), quote);
        return value;
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.end_reached() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    pub fn parse(&mut self, source: String) -> dom::Node {
        let mut nodes = Parser { position: 0, data: source }.parse_nodes();
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::element("html".to_string(), HashMap::new(), nodes)
        }
    }
}
