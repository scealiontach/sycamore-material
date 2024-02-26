use super::core::MDLProps;
use sycamore::prelude::*;

#[component]
pub fn mdl_layout<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout mdl-js-layout {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_header<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout__header {0}", props.class);
    view! {cx,
      header(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_header_row<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout__header-row {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_title<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout-title {0}", props.class);
    view! {cx,
      span(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_spacer<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout-spacer {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_navigation<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-navigation {0}", props.class);
    view! {cx,
      nav(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_drawer<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout__drawer {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_content<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout__content {0}", props.class);
    view! {cx,
      main(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_navigation_link<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-navigation__link {0}", props.class);
    view! {cx,
      a(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_tab_bar<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout__tab-bar {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_tab<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout__tab {0}", props.class);
    view! {cx,
      a(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_layout_tab_panel<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-layout__tab-panel {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_grid<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-grid {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_cell<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-cell {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mini_footer<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mini-footer {0}", props.class);
    view! {cx,
      footer(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mini_footer_left<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mini-footer__left-section {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mini_footer_right<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mini-footer__right-section {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mini_footer_social_btn<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mini-footer__social-btn {0}", props.class);
    view! {cx,
      button(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mini_footer_link_list<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mini-footer__link-list {0}", props.class);
    view! {cx,
      ul(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mega_footer<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mega-footer {0}", props.class);
    view! {cx,
      footer(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mega_footer_top_section<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mega-footer__top-section {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mega_footer_middle_section<'a, G: Html>(
    cx: Scope<'a>,
    props: MDLProps<'a, G>,
) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mega-footer__middle-section {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mega_footer_bottom_section<'a, G: Html>(
    cx: Scope<'a>,
    props: MDLProps<'a, G>,
) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mega-footer__bottom-section {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mega_footer_link_list<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mega-footer__link-list {0}", props.class);
    view! {cx,
      ul(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mega_footer_drop_down_section<'a, G: Html>(
    cx: Scope<'a>,
    props: MDLProps<'a, G>,
) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mega-footer__drop-down-section {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mega_footer_heading<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mega-footer__heading {0}", props.class);
    view! {cx,
      h1(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_mega_footer_right_section<'a, G: Html>(
    cx: Scope<'a>,
    props: MDLProps<'a, G>,
) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-mega-footer__right-section {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_content_tabs<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-tabs mdl-js-tabs {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_content_tabs_bar<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-tabs__tab-bar {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_content_tab<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-tabs__tab {0}", props.class);
    view! {cx,
      a(class=class) {
        (children)
      }
    }
}

#[component]
pub fn mdl_content_tab_panel<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let class = format!("mdl-tabs__panel {0}", props.class);
    view! {cx,
      div(class=class) {
        (children)
      }
    }
}
