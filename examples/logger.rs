#[macro_use] extern crate cocaine;

use cocaine::logging::{LoggerContext, Severity};

fn main() {
    let ctx = LoggerContext::default();
    let log = ctx.create("proxy/access");

    // The simpliest message.
    log!(log, Severity::Info, "nginx/1.6 configured");

    // Using lazy format arguments.
    log!(log, Severity::Info, "{} {} HTTP/1.1 {} {}", "GET", "/static/image.png", 404, 347);

    // Attaching additional meta information.
    log!(log, Severity::Info, "nginx/1.6 configured", {
        config: "/etc/nginx/nginx.conf",
        elapsed: 42.15,
    });

    // More ...
    log!(log, Severity::Warn, "client stopped connection before send body completed", {
        host: "::1",
        port: 10053,
    });

    // And both. You can even use functions as meta for lazy evaluations.
    log!(log, Severity::Error, "file does not exist: {}", ["/var/www/favicon.ico"], {
        path: "/",
        cache: true,
        method: "GET",
        version: 1.1,
        protocol: "HTTP",
    });
}
