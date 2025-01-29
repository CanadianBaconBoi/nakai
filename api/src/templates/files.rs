use maud::{html, Markup};
use crate::templates::base;

pub(crate)  fn download() -> Markup {
    base::t_base("Hostess · Simple file hosting - Download", html! {
        // TODO: Implement file previews for text and images, and allow threaded downloading of videos
        nav {
            ul {
                li {
                    a {
                        b {
                            "Report illegal content"
                            //This will take in the ID of the file uploaded and then flag it for review
                        }
                    }
                }
            }
        }
    })
}