use maud::{html, Markup};
use crate::templates::base::t_base;

pub(crate) fn index() -> Markup {
    t_base("Hostess · Simple file hosting", html! {
        div class="card" {
            h1 class="jumbotron" {
                "仲居"
            }
            p class="lead" {
                "Max upload size is "
                b{"64 MiB"}
                " & files expire after "
                b{"6 hours."}
            }
            br {}
            form class="dropzone" method="POST" action="/upload" enctype="multipart/form-data" id="upload-form" {
                div class="fallback" {
                    input name="file" type="file" multiple {}
                }
            }
            br {}
            nav {
                ul {
                    li {
                        a href="/#" {" API "}
                        a href="/#" {" FAQ "}
                        b{a href="/#" {" Hostess "}}
                        a href="/#" {" Legal "}
                        a href="https://github.com/ashtonqlb/nakai" {" Source "}
                    }
                }
            }
        }
    })
}