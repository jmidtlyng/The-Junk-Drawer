use std::net::TcpListener;
use std::collections::HashMap;
//use core::runner::run;
use sailfish::TemplateOnce;
use glob::glob;
use assist:Assist;
use assist_derive:Assist;
/*
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

async fn server_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //routes_check(app, client).await;
}
*/
#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
struct HelloTemplate {
    // data to be passed to the template
    messages: Vec<String>,
}

#[derive(Assist)]
struct Templates;

#[actix_rt::test]
fn macro_check(){
    Templates::assist();
}

//async fn sailfish_check(app: TestApp, client: reqwest::Client)

/*
#[actix_rt::test]
async fn ayayi_check(){
    for entry in glob("../templates/- * fix me *- /*.stpl").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
    
    let ctx = HelloTemplate {
        messages: vec![String::from("foo"), String::from("bar")],
    };
    let render_html = ctx.render_once().unwrap();
    // Now render templates with given data
    // println!("{}", ctx);
    let assertion_html = "<html>
<body>

<div>foo</div>

<div>bar</div>

</body>
</html>";
    assert_eq!(render_html, assertion_html);
}
*/
/*
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