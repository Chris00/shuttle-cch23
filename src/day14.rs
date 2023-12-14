use axum::{Json, response::Html};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Body {
    content: String,
}

fn replace_all(s: &mut String, patt: char, repl: &str) {
    while let Some(i) = s.find(patt) {
        s.replace_range(i .. i+1, repl)
    }
}

pub async fn render_html(Json(body): Json<Body>) -> Html<String> {
    // Safe encodings do these transformations:
    // & => &amp;
    // < => &lt;
    // > => &gt;
    // " => &quot;
    // ' => &#x27;
    // / => &#x2F;
    // But, here, they do not want / to be encoded.  Thus we perform
    // the substitutions by hand.
    let mut c = body.content;
    replace_all(&mut c, '&', "&amp;");
    replace_all(&mut c, '<', "&lt;");
    replace_all(&mut c, '>', "&gt;");
    replace_all(&mut c, '"', "&quot;");
    replace_all(&mut c, '\'', "&#27;");
    Html(format!("\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>", c))
}


pub async fn render_html_unsafe(Json(body): Json<Body>) -> Html<String> {
    Html(format!("\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>", body.content))
}
