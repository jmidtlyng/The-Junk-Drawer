use std::net::TcpListener;
use std::collections::HashMap;
//use core::runner::run;
use sailfish::TemplateOnce;
/*
    use glob::glob;
    use assist::Assist;
    use assist_derive::Assist;
*/

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

    template_rendering(app, client).await;
}

#[derive(TemplateOnce)]
#[template(path = "admin/index.stpl")]
struct AdminTemplate {
    // data to be passed to the template
    messages: Vec<String>,
}

#[actix_rt::test]
async fn template_rendering(app: TestApp, client: reqwest::Client){
    let route = "admin";
    let uri = &format!("{}/{}", &app.address, route);
    let response = client
        .get("/admin")
        .send()
        .await
        .expect("Failed to execute request.");
    let response_html = response.text().await.unwrap();
    
    let ctx = AdminTemplate {
        messages: vec![String::from("foo"), String::from("bar")],
    };
    let template_html = ctx.render_once().unwrap();
        
    // Now render templates with given data
    // println!("{}", ctx);
    
    println!(template_html);
    println!(response_html);
    
    assert_eq!(template_html, response_html);
}

//async fn sailfish_check(app: TestApp, client: reqwest::Client)

/*
    #[derive(Assist)]
    struct Templates;
    
    #[actix_rt::test]
    async fn macro_check(){
        Templates::assist();
    }
    
    async fn server_check() {
        let app = spawn_app().await;
        let client = reqwest::Client::new();
    
        // routes_check(app, client).await;
    }
    
    async fn routes_check(app: TestApp, client: reqwest::Client) {
        let mut routes = HashMap::new();
    
        routes.insert("", views::templates::index::html());
        routes.insert("map", views::templates::map::html());
        routes.insert("details", views::templates::details::html());
        
        
        for entry in glob("/templates").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("{:?}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
        
    
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
*/
