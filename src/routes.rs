use crate::controllers::handle_page;
use crate::controllers::index;
use actix_web::{web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/pages", web::get().to(handle_page::get_page)); // GET the page
    cfg.route("/edit", web::get().to(handle_page::get_editor)); // GET the editor
    cfg.route("/edit", web::post().to(handle_page::post)); // POST the new contents to update the file
    cfg.route("/edit", web::delete().to(handle_page::delete)); // Delete the file
    cfg.service(actix_files::Files::new("/files", "public").show_files_listing());
    cfg.route("/index.html", web::get().to(index::index));
    // with path parameters
    cfg.route(
        "/",
        web::get().to(|| async {
            HttpResponse::Found()
                .append_header(("LOCATION", "/index.html"))
                .finish()
        }),
    );
}
