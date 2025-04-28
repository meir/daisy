%start Expr
%%

Name -> Result<Str, ()>:
  "NAME" {
    let span = $lexer.span_str($1.map_err(|_| ())?.span());
    Ok(Str::new(span.to_string()))
  };

Str -> Result<Str, ()>:
  "STRING" {
    let span = $lexer.span_str($1.map_err(|_| ())?.span());
    Ok(Str::new(span.to_string()))
  };

Element -> Result<Box<Element>, ()>:
  Name "{" "}" {
      Ok(Box::new(Element::new($1?)))
    }
  | Name "{" Body "}" {
      let mut element = Element::new($1?);
      element.add_child($3?);
      Ok(Box::new(element))
    };

Body -> Result<Box<dyn Node>, ()>:
  Element { Ok($1? as Box<dyn Node>) }
  | Body Element {
    let mut body = Box::<Box::into_inner>(Box::new($1?));
    body.add_child(Box::new($2?));
    Ok(Box::new(body))
  }
  | Str { Ok(Box::new($1?) as Box<dyn Node>) };

Expr -> Result<Root, ()>:
  Body { 
    let mut root = Root::new();
    root.add_node($1?);
    Ok(root)
  };

%%

use crate::node::{
  Node,
  Body,

  str::Str,
  element::Element,
  root::Root,
};
