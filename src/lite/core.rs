use sycamore::prelude::*;

#[derive(Prop)]
pub struct MDLThemeProps<'a> {
    pub primary: &'a str,
    pub accent: &'a str,
}

#[component]
pub fn MDL<G: Html>(cx: Scope, props: MDLThemeProps) -> View<G> {
    let version = "1.3.0";
    let icons_url = "https://fonts.googleapis.com/icon?family=Material+Icons";
    let theme_url = format!(
        "https://code.getmdl.io/{0}/material.{1}-{2}.min.css",
        version, props.primary, props.accent
    );
    let script_url = format!("https://code.getmdl.io/{0}/material.min.js", version);
    view! {cx,
      link(rel="stylesheet", href=icons_url)
      link(rel="stylesheet", href=theme_url)
      script(defer=true, src=script_url)
    }
}

#[derive(Prop)]
pub struct MDLProps<'a, G: Html> {
    #[builder(default)]
    pub class: &'a str,
    #[builder(default)]
    pub children: Children<'a, G>,
}

#[component]
pub fn material_icon<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("material-icons {0}", props.class);
    view! {cx,
      i(class=class) {
        (children)
      }
    }
}

#[component]
pub fn textfield<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-textfield mdl-js-textfield {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn logo<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-logo {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn button<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let class = format!("mdl-button mdl-js-button {}", props.class);
    let children = props.children.call(cx);
    view! {cx,
      button(class=class) {
        (children)
      }
    }
}
