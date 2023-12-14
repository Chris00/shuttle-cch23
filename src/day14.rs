use axum::{Json, response::Html};
use serde::Deserialize;
use minijinja::{Environment, context, AutoEscape};

#[derive(Deserialize, Debug)]
pub struct Body {
    content: String,
}


pub async fn render_html(Json(body): Json<Body>) -> Html<String> {
    let mut env = Environment::new();
    env.set_auto_escape_callback(|_| AutoEscape::Html);
    env.add_template("render",
        "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{body}}
  </body>
</html>").unwrap();
    let tmpl = env.get_template("render").unwrap();
    Html(format!("{}", tmpl.render(context!(body => body.content)).unwrap()))
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
