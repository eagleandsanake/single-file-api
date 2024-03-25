use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize; // 导入 Deserialize 特性

#[get("/read/later")]
async fn read_later(params: web::Query<Params>) -> impl Responder {
    // 尝试从环境变量中获取名为SAVE_DIR的值
    // let save_dir = match std::env::var("SAVE_DIR") {
    //     Ok(val) => val,
    //     Err(_) => {
    //         return HttpResponse::InternalServerError().body("Failed to get environment variable");
    //     }
    // };

    // let chrome_dir = match std::env::var("CHROME_DIR") {
    //     Ok(val) => val,
    //     Err(_) => {
    //         return HttpResponse::InternalServerError().body("Failed to get environment variable");
    //     }
    // };
    let chrome_parma = format!("--browser-executable-path {}", "/home/wuxin/Desktop/chrome-linux64/chrome");
    let out_put_dir = format!("--output-directory {}", "/home/wuxin/Desktop/singleTest/");


    let url = format!("\"{}\"", params.str);
    let sand_box = format!("--browser-args [{}\"--no-sandbox{}\"]", "\\", "\\");
    let cmd  =  format!("/home/wuxin/single-file {} {} {} {} {} {}"
            , chrome_parma
            , url
            , out_put_dir
            , "--filename-template={page-title}.html"
            , "--load-deferred-images-dispatch-scroll-event=true"
            , sand_box
    );
    println!("{}", cmd);

    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        HttpResponse::Ok().body(format!(
            "Page '{}' added successfully",
            params.str
        ))
    } else {
        HttpResponse::InternalServerError().body("Failed to execute command")
    }
}

#[derive(Debug, Deserialize)]
struct Params {
    str: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(read_later)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
