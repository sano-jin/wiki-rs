use crate::controllers::handle_attach;
use crate::controllers::handle_page;
use crate::controllers::handle_user;
use crate::controllers::index;
use crate::controllers::login;
use crate::controllers::validate;
use crate::gateways::db::Database;
use actix_web::{web, HttpResponse};

pub fn routes<T: 'static + Clone + Database>(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(login::login::<T>)); // POST the new contents to update the file
    cfg.route("/login", web::get().to(login::index::<T>)); // POST the new contents to update the file
    cfg.route("/validate", web::get().to(validate::get_user::<T>)); // POST the new contents to update the file

    cfg.route("/user", web::delete().to(handle_user::delete::<T>)); // POST the new contents to update the file
    cfg.route("/user", web::post().to(handle_user::post::<T>)); // POST the new contents to update the file
    cfg.route("/users", web::get().to(handle_user::get_users::<T>)); // GET the users
                                                                     //
    cfg.route("/attach", web::get().to(handle_attach::get::<T>)); // GET the attached files
    cfg.route("/attach", web::delete().to(handle_attach::delete::<T>)); // GET the attached files
    cfg.route("/attach", web::post().to(handle_attach::post::<T>)); // GET the attached files
                                                                    //
    cfg.route("/pages", web::get().to(handle_page::get_page::<T>)); // GET the page
                                                                    //
    cfg.route("/edit", web::get().to(handle_page::get_editor::<T>)); // GET the editor
    cfg.route("/edit", web::post().to(handle_page::post::<T>)); // POST the new contents to update the file
    cfg.route("/edit", web::delete().to(handle_page::delete::<T>)); // Delete the file
                                                                    //
    cfg.service(actix_files::Files::new("/files", "public").show_files_listing());
    cfg.route("/index.html", web::get().to(index::index::<T>));
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
