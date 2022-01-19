use crate::helpers::spawn_app;
use reqwest::header::HeaderValue;
use std::collections::HashSet;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let response = app.post_login(&login_body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");

    let cookies: HashSet<_> = response
        .headers()
        .get_all("Set-Cookie")
        .into_iter()
        .collect();
    assert!(cookies.contains(&HeaderValue::from_str("_flash=Authentication failed").unwrap()));

    let flash_cookie = response.cookies().find(|c| c.name() == "_flash").unwrap();
    assert_eq!(flash_cookie.value(), "Authentication failed");

    // Act - Part 2
    let response = app.get_login().await;
    let html_page = response.text().await.unwrap();
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    // Act - Part 3
    let response = app.get_login().await;
    let html_page = response.text().await.unwrap();
    assert!(!html_page.contains(r#"<p><i>Authentication failed</i></p>"#));
}
