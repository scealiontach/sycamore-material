use super::core::MDLProps;
use sycamore::prelude::*;

#[component]
pub fn data_table<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let class = format!("mdl-data-table mdl-js-data-table {}", props.class);
    let children = props.children.call(cx);
    view! {cx,
      table(class=class) {
        (children)
      }
    }
}

#[component]
pub fn data_table_head<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let class = props.class;
    let children = props.children.call(cx);
    view! {cx,
      thead(class=class) {
        (children)
      }
    }
}

#[component]
pub fn data_table_body<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let class = props.class;
    let children = props.children.call(cx);
    view! {cx,
      tbody(class=class) {
        (children)
      }
    }
}

#[component]
pub fn data_table_header<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let class = format!("mdl-data-table__cell--non-numeric {}", props.class);
    let children = props.children.call(cx);
    view! {cx,
      th(class=class) {
        (children)
      }
    }
}

#[component]
pub fn data_table_cell<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let class = format!("mdl-data-table__cell--non-numeric {}", props.class);
    let children = props.children.call(cx);
    view! {cx,
      td(class=class) {
        (children)
      }
    }
}

#[component]
pub fn data_table_numcell<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let class = props.class;
    let children = props.children.call(cx);
    view! {cx,
      td(class=class) {
        (children)
      }
    }
}

#[component]
pub fn data_table_row<'a, G: Html>(cx: Scope<'a>, props: MDLProps<'a, G>) -> View<G> {
    let class = props.class;
    let children = props.children.call(cx);
    view! {cx,
      tr(class=class) {
        (children)
      }
    }
}
