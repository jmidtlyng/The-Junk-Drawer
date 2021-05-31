use std::net::TcpListener;
use core::server::serve;
use core::views;

pub struct TheJunkDrawer {
    pub address: String
}

async fn spawn_app() -> TheJunkDrawer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let server = serve(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TheJunkDrawer { address }
}

#[actix_rt::test]
async fn server_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    template_rendering_check(app, client).await;
    crud_check(app, client).await;
}

async fn template_rendering_check(app: TheJunkDrawer, client: reqwest::Client){
    let routes: &[(&str, String); 3] = &[
        ("admin", views::admin::test()),
        ("admin/entities", views::admin::entities::test()),
        ("admin/fields", views::admin::fields::test())
    ];
    
    for route in routes {
        evaluate_result_html
        let uri = &format!("{}/{}", &app.address, route.0);
        let response = client.get(uri).send().await
            .expect("Failed to execute request.");
        let response_html = response.text().await.unwrap();
        
        assert_eq!(route.1, response_html);
    }
}

async fn crud_check(app: TheJunkDrawer, client: reqwest::Client){
    views::admin::entity_types::test();
}

/*
fn data_check(app: TheJunkDrawer){
    let mut entities = app.data.entities.lock().unwrap();
    *entities += 1;
    
    assert_eq!(1, entities);
}
*/