use maud::{html, Markup};
use crate::templates::base::t_base;

pub(crate) fn not_found() -> Markup {
    t_base("Not Found", html! {
        div class="card" {
            h1 class="jumbotron" {
                "404 Not Found"
            }
        }
    })
}

pub(crate) fn banned() -> Markup {
    t_base("Banned", html! {
        div class="jumbotron" {
            img src="/static/images/banned.webp" style="height:300px";
            p class="lead" {"You are banned"}
        }
    })
}