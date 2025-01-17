use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize; // 导入 Deserialize 特性
use tokio::process::Command;
use env_logger::Env;

#[get("/read/later")]
async fn read_later(params: web::Query<Params>) -> impl Responder {
    // 尝试从环境变量中获取名为SAVE_DIR的值
    let save_dir = match std::env::var("SAVE_DIR") {
        Ok(val) => val,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to get environment variable");
        }
    };

    let chrome_dir = match std::env::var("CHROME_DIR") {
        Ok(val) => val,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to get environment variable");
        }
    };

    let single_page_dir = std::env::var("SINGLE_FILE_DIR").unwrap_or_else(|_| {
        // 如果获取环境变量失败，返回一个默认值
        "/app/single-file".to_string()
    });

    let back_dir = std::env::var("BACK_UP_DIR").unwrap_or_else(|_| {
        // 如果获取环境变量失败，返回一个默认值
        "NO_NEED_TO_BACK".to_string()
    });

    let single_file_proxy = std::env::var("SINGLE_FILE_PROXY").unwrap_or_else(|_| {
        // 如果获取环境变量失败，返回一个默认值
        "NO_PROXY".to_string()
    });

    // let chrome_parma = format!("--browser-executable-path {}", "/home/wuxin/Desktop/chrome-linux64/chrome");
    // let out_put_dir = format!("--output-directory {}", "/home/wuxin/Desktop/singleTest/");

    let chrome_parma = format!("--browser-executable-path {}",chrome_dir );
    let out_put_dir = format!("--output-directory {}", save_dir);


    let url = format!("\"{}\"", params.str);
    let sand_box = format!("--browser-args [{}\"--no-sandbox{}\"]", "\\", "\\");

    // cmd async exe
    tokio::spawn(async move {
        if !back_dir.eq("NO_NEED_TO_BACK"){
            let back_dir_full_dir = format!("--output-directory {}", back_dir);
            let mut  cmd_back  =  format!("{} {} {} {} {} {} {}"
                                          , single_page_dir
                                          , chrome_parma
                                          , url
                                          , back_dir_full_dir
                                          , "--filename-template={page-title}.html"
                                          , "--load-deferred-images-dispatch-scroll-event=true"
                                          , sand_box
            );

            if !single_file_proxy.eq("NO_PROXY") {
                cmd_back += &format!(" --http-proxy-server {}", single_file_proxy);
            }
            println!("文件备份命令 {}", cmd_back);

            let output_back = Command::new("bash")
                .arg("-c")
                .arg(cmd_back)
                .output()
                .await;
            match output_back {
                Ok(output_back) => {
                    if output_back.status.success() {
                        println!("Command executed successfully");
                    } else {
                        println!("Command execution failed");
                    }
                }
                Err(e) => {
                    println!("Failed to execute command: {}", e);
                }
            }
        }

        // download to target
        let mut  cmd  =  format!("{} {} {} {} {} {} {}"
                                 , single_page_dir
                                 , chrome_parma
                                 , url
                                 , out_put_dir
                                 , "--filename-template={page-title}.html"
                                 , "--load-deferred-images-dispatch-scroll-event=true"
                                 , sand_box
        );

        if !single_file_proxy.eq("NO_PROXY") {
            cmd += &format!(" --http-proxy-server {}", single_file_proxy);
        }
        println!("文件下载命令: {}", cmd);

        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .await;
        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Command executed successfully");
                } else {
                    println!("Command execution failed");
                }
            }
            Err(e) => {
                println!("Failed to execute command: {}", e);
            }
        }
    });

    // http success
    HttpResponse::Ok().body(format!(
        "Page '{}' added successfully",
        params.str
    ))

    // let output = std::process::Command::new("bash")
    //     .arg("-c")
    //     .arg(cmd)
    //     .output()
    //     .expect("Failed to execute command");

    // if output.status.success() {
    //     HttpResponse::Ok().body(format!(
    //         "Page '{}' added successfully",
    //         params.str
    //     ))
    // } else {
    //     HttpResponse::InternalServerError().body("Failed to execute command")
    // }
}

#[derive(Debug, Deserialize)]
struct Params {
    str: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .service(read_later)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
