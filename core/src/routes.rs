use actix_web::web;
use glob::glob;
use std::str;
use quote::quote;
use crate::views;

pub fn config(cfg: &mut web::ServiceConfig) {
    let admin_scope = web::scope("/admin");
    
    for template in glob("views/admin/**").expect("Failed to read entry path"){
        match template {
            Ok(template_pathbuf) => {                    
                let template_str = template_pathbuf.to_str().unwrap();
                let template_path = str::replace(template_str, "/", "::");
                let template_service = quote! { #template_path };
                        
                admin_scope.service( template_service.into() );
            },
            Err(e) => println! ("{:?}", e)
        }
    }
    
    cfg.service( admin_scope );
}
