use std::net::TcpListener;
use core::server::serve;
use sailfish::TemplateOnce;

pub struct TestApp {
    pub address: String
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let server = serve(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { address }
}

#[actix_rt::test]
async fn server_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    template_rendering(app, client).await;
}

async fn template_rendering(app: TestApp, client: reqwest::Client){
    // define routes as tuple with address and local template for variables
    let routes = [
        ("admin", admin_template)
    ];
    
    foreach(route in routes){
        let uri = &format!("{}/{}", &app.address, route.0);
        route.1(uri, client);
    }
}


#[derive(TemplateOnce)]
#[template(path = "admin/template.stpl")]
struct AdminTemplate {
    //messages: Vec<String>,
}

async fn admin_template(uri: &str, client: reqwest::Client){
    let response = client
        .get(uri)
        .send()
        .await
        .expect("Failed to execute request.");
    let response_html = response.text().await.unwrap();
    
    let ctx = AdminTemplate {
        //messages: vec![String::from("foo"), String::from("bar")],
    };
    let template_html = ctx.render_once().unwrap();
        
    // Now render templates with given data
    // println!("{}", ctx);
    
    //println!("{}", response_html);
    //println!("{}", template_html);
    
    assert_eq!(template_html, response_html);
}
