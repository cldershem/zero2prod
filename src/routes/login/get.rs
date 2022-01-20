use crate::utils::make_flash_message;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let msg_html = make_flash_message(flash_messages);

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
    <html lang="en">
    <head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
    </head>
    <body>
    {}
    <form action="/login" method="post">
    <label>Username
        <input
            type="text"
            placeholder="Enter Username"
            name="username"
        >
    </label>
    <label>Password
        <input
            type="password"
            placeholder="Enter Password"
            name="password"
        >
    </label>
    <button type="submit">Login</button>
    </form>
    </body>
    </html>"#,
            msg_html
        ))
}
