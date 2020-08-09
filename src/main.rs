extern crate chrono;

use chrono::offset::Local;
use chrono::DateTime;

extern crate iron;
use iron::headers::{Connection, ContentType};

use iron::prelude::*;
use iron::status;

fn run_http_server<H: iron::Handler>(service_name: &str, port: u16, handler: H) {
    use iron::Timeouts;
    use std::env;
    use std::time::Duration;

    let mut iron = Iron::new(handler);
    let threads_s = env::var("IRON_HTTP_THREADS").unwrap_or("1".to_string());
    let threads = threads_s.parse::<usize>().unwrap_or(1);
    iron.threads = threads;
    iron.timeouts = Timeouts {
        keep_alive: Some(Duration::from_millis(10)),
        read: Some(Duration::from_secs(10)),
        write: Some(Duration::from_secs(10)),
    };

    let bind_ip = env::var("BIND_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_port_s = env::var("BIND_PORT").unwrap_or(port.to_string());
    let bind_port = bind_port_s.parse::<u16>().unwrap_or(port);

    println!(
        "HTTP server for {} starting on {}:{}",
        service_name, &bind_ip, bind_port
    );
    iron.http(&format!("{}:{}", &bind_ip, bind_port)).unwrap();
}

fn handle(_req: &mut Request) -> IronResult<Response> {
    let payload = if let Ok(dir) = std::fs::read_dir("/var/www/html/PDF") {
        let mut entries = dir.collect::<Result<Vec<_>, std::io::Error>>().unwrap();

	entries.sort_by(|a,b| b.metadata().unwrap().modified().unwrap().cmp(&a.metadata().unwrap().modified().unwrap()));

        let mut p = format!("<html><head>");
        p.push_str("<meta name=\"format-detection\" content=\"telephone=no\">");
        p.push_str("<body><table>");
        let mut curr_date = format!("");
        for entry in &entries {
            if entry.metadata().unwrap().is_file() {
                let system_time = entry.metadata().unwrap().modified().unwrap();
                let datetime: DateTime<Local> = system_time.into();
                let date_str = datetime.format("%T");
		let day_str = datetime.format("%Y %B %d").to_string();
                if day_str != curr_date {
                   p.push_str(&format!("<tr><td colspan=3><h1>{}</h1></td></tr>", &day_str));
                   curr_date = day_str;
                } 

                p.push_str(&format!(
                    "<tr><td><a href=\"/PDF/{}\">{}</a></td><td>{}</td><td>{}</td></tr>\n",
                    &entry.file_name().to_str().unwrap_or("err"),
                    &entry.file_name().to_str().unwrap_or("err"),
		    &entry.metadata().unwrap().len(),
                    &date_str,
                ));
            }
        }
        p.push_str("</table></body></html>\n");
        p
    } else {
        format!("Hello")
    };

    let mut resp = Response::with((status::Ok, payload));
    resp.headers.set(ContentType::html());
    resp.headers.set(Connection::close());
    Ok(resp)
}

fn main() {
    println!("Hello, world!");
    run_http_server("file list", 8080, handle);
}
