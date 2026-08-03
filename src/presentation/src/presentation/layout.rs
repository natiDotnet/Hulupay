use crate::presentation::components::{Navbar, Sidebar};
use leptos::prelude::*;

#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-slate-950 text-slate-100 flex">
            <Sidebar />
            <div class="flex-1 flex flex-col">
                <Navbar />
                <main class="p-6 md:p-8">{children()}</main>
            </div>
        </div>
    }
}
