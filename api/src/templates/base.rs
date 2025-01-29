use maud::{html, Markup, DOCTYPE};

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        meta name="description" content="Hostess is a simple-to-use, free file hosting service.";
        title { (page_title) }
        link rel="icon" type="image/png" href="/static/images/favicon.ico";
        link rel="stylesheet" href="/static/css/style.css";
        link rel="stylesheet" href="https://unpkg.com/dropzone@5/dist/min/dropzone.min.css" type="text/css";
        script src="https://unpkg.com/dropzone@5/dist/min/dropzone.min.js" {}
    }
}

fn footer() -> Markup {
    html! {
        footer {
            script src="/static/js/hostess.js" type="module";
        }
    }
}

pub(crate) fn t_base(page_title: &str, content: Markup) -> Markup {
    html! {
        head {
            (header(page_title))
        }
        body {
            (content)
            (footer())
        }
    }
}