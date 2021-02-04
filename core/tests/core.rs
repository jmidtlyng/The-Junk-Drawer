use std::net::TcpListener;
use std::collections::HashMap;
use core::runner::run;
use views;

pub struct TestApp {
    pub address: String
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { address }
}

#[actix_rt::test]
async fn server_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    routes_check(app, client).await;
}

async fn routes_check(app: TestApp, client: reqwest::Client) {
    let mut routes = HashMap::new();

    routes.insert("", views::templates::index::html());
    routes.insert("map", views::templates::map::html());
    routes.insert("details", views::templates::details::html());

    for (route, template) in routes {
        let uri = &format!("{}/{}", &app.address, route);
        println!("{:?}", uri);
        let response = client
            .get(uri)
            .send()
            .await
            .expect("Failed to execute request.");
        let response_html = response.text().await.unwrap();
        assert_eq!(template, response_html);
    }
}
