#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::io;

use rocket::request::{Form, FormDataError, FormError};
use rocket::response::NamedFile;
// use rocket::response::*;
// use rocket::response::content;
use rocket::http::RawStr;
// use handlebars::Handlebars;

extern crate rocket_contrib;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

use postgres::{Client, NoTls};
use rocket::response::Redirect;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;

use std::fs;
use std::fs::File;
// use std::io;
use std::io::Write;

#[derive(Debug, FromFormValue)]
enum FormOption {
    A,
    B,
    C,
}

#[derive(Debug, FromForm)]
struct FormInput<'r> {
    checkbox: bool,
    number: usize,
    #[form(field = "type")]
    radio: FormOption,
    password: &'r RawStr,
    #[form(field = "textarea")]
    text_area: String,
    select: FormOption,
}

// use crate::routes::{ static_files, get };
// mod routes;

// #[post("/", data = "<sink>")]
// fn sink(sink: Result<Form<FormInput>, FormError>) -> String {
//     match sink {
//         Ok(form) => {format!("{:?}", &*form)},
//         Err(FormDataError::Io(_)) => format!("Form input was invalid UTF-8."),
//         Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => {
//             format!("Invalid form input: {}", f)
//         }
//     }
// }
// #[post("/", data = "<sink>")]
// fn sink(sink: Result<Form<FormInput>, FormError>) -> io::Result<NamedFile> {
//     match sink {
//         Ok(form) => {

//             NamedFile::open("static/result.html")
//         },
//         Err(FormDataError::Io(_)) => NamedFile::open("static/index.html"),
//         Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => NamedFile::open("static/index.html")
//     }
// }

// #[get("/")]
// fn index() -> io::Result<NamedFile> {
//     let context = Context::new();
//     Template::render("template-name", &context);
//     NamedFile::open("static/index.html")
// }

// fn rocket() -> rocket::Rocket {
//     rocket::ignite().attach(Template::fairing()).mount("/", routes![index, sink])

// }

// fn main() {
//     rocket().launch();
// }

mod atlas;
mod static_file;
use atlas::*;
mod sql_import;
use json_import::*;
mod json_import;
use sql_import::*;
mod templating;
use templating::*;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>,
}

#[get("/")]
fn base() -> io::Result<NamedFile> {
    NamedFile::open("static/base.html")

}

// #[get("/hello/<name>")]
// fn get(name: String) -> Template {
//     let context = TemplateContext {
//         name,
//         items: vec!["Residence".to_string(), "Two".to_string()],
//     };
//     Template::render("index", &context)
// }

// #[get("/atlas")]
// fn atlas() -> Template {
//     fn atlas() {
//     let atlas = import_complete_atlas();
//     // let mut map_names = vec![];
//     // for map in atlas.maps {
//     //     map_names.push(map.name.unwrap())
//     // }
//     let mut context = AtlasDisplay::new();
//     for map in atlas.maps {
//         let display = MapDisplay::convert(map);
//         context.maps.push(display);
//     }
//     // let json_dump = serde_json::to_string(&context);
//     // let mut buffer = File::create("D:/database.json").unwrap();
//     // buffer.write_all(json_dump.unwrap().as_bytes()).unwrap();
//     Template::render("complete_table_display", &context)
// }

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![base, static_file::all])
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
