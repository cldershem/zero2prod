use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    };

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Publish Newsletter Issue</title>
  </head>
  <body>
    {}
    <form action="/admin/newsletters" method="post">
      <label for="title">Title:
        <input
          id="title"
          type="text"
          placeholder="Issue Title"
          name="title"
          >
      </label>
      <br>
      <label for="text_content">Text</label>
      <textarea id="text_content" name="text_content" rows="4" cols="50" placeholder="Enter Newsletter as Text"></textarea>
      <br>
      <label for="html_content">HTML</label>
      <textarea id="html_content" name="html_content" rows="4" cols="50" placeholder="Enter Newsletter as HTML"></textarea>
      <br>
      <button type="submit">Publish</button>
      </form>
      <p><a href="/admin/dashboard">&lt;- Back</a></p>
  </body>
</html>"#,
            msg_html
        )))
}
