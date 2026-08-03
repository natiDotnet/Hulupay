use crate::domain::models::{Role, User, UserStatus};
use crate::presentation::state::{AuthState, UiState};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    let ui = expect_context::<UiState>();
    let nav_items = vec![
        ("/", "Dashboard", "📊"),
        ("/users", "Users", "👥"),
        ("/settings", "Settings", "⚙️"),
        ("/notifications", "Notifications", "🔔"),
    ];

    view! {
        <aside class=move || {
            format!(
                "bg-slate-950 border-r border-slate-800 transition-all duration-200 {}",
                if ui.sidebar_collapsed.get() { "w-20" } else { "w-64" }
            )
        }>
            <div class="px-4 py-6 flex items-center justify-between">
                <span class="font-semibold text-white tracking-tight">"Atlas Admin"</span>
                <button
                    class="text-slate-400 hover:text-white"
                    on:click=move |_| ui.sidebar_collapsed.update(|c| *c = !*c)
                >
                    "⇆"
                </button>
            </div>
            <nav class="px-2 pb-6 space-y-1">
                {nav_items
                    .into_iter()
                    .map(|(href, label, icon)| view! {
                        <A href=href class="flex items-center gap-3 px-3 py-2 rounded-lg text-slate-300 hover:bg-slate-800 hover:text-white">
                            <span>{icon}</span>
                            <Show when=move || !ui.sidebar_collapsed.get()>
                                <span class="text-sm">{label}</span>
                            </Show>
                        </A>
                    })
                    .collect_view()}
            </nav>
        </aside>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    view! {
        <header class="h-16 border-b border-slate-800 bg-slate-950 px-6 flex items-center justify-between">
            <input
                class="w-72 bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-100 focus:outline-none"
                placeholder="Search users, events, IDs..."
            />
            <div class="flex items-center gap-4">
                <button class="relative text-slate-300 hover:text-white">"🔔"<span class="absolute -top-1 -right-2 bg-indigo-500 text-xs rounded-full px-1">"3"</span></button>
                <button class="text-slate-100 bg-slate-800 px-3 py-1 rounded-lg text-sm" on:click=move |_| auth.is_authenticated.set(false)>"Sign out"</button>
            </div>
        </header>
    }
}

#[component]
pub fn KpiCard(
    title: &'static str,
    value: &'static str,
    delta: &'static str,
    positive: bool,
) -> impl IntoView {
    view! {
        <article class="rounded-xl border border-slate-800 bg-slate-900/70 p-5">
            <p class="text-sm text-slate-400">{title}</p>
            <p class="mt-2 text-2xl font-semibold text-slate-100">{value}</p>
            <p class=if positive { "text-emerald-400 text-sm" } else { "text-rose-400 text-sm" }>{delta}</p>
        </article>
    }
}

#[component]
pub fn UsersTable(
    users: ReadSignal<Vec<User>>,
    on_edit: Callback<User>,
    on_delete: Callback<String>,
) -> impl IntoView {
    view! {
        <table class="w-full text-left text-sm">
            <thead class="text-slate-400 border-b border-slate-800">
                <tr>
                    <th class="py-3">"Name"</th>
                    <th>"Email"</th>
                    <th>"Role"</th>
                    <th>"Status"</th>
                    <th>"Actions"</th>
                </tr>
            </thead>
            <tbody>
                <For
                    each=move || users.get()
                    key=|user| user.id.clone()
                    children=move |user| {
                        let role = match user.role {
                            Role::Admin => "Admin",
                            Role::Operator => "Operator",
                            Role::Viewer => "Viewer",
                        };
                        let status = match user.status {
                            UserStatus::Active => "Active",
                            UserStatus::Invited => "Invited",
                            UserStatus::Suspended => "Suspended",
                        };
                        let delete_id = user.id.clone();
                        view! {
                            <tr class="border-b border-slate-900 text-slate-200">
                                <td class="py-3">{user.name.clone()}</td>
                                <td>{user.email.clone()}</td>
                                <td>{role}</td>
                                <td>{status}</td>
                                <td class="space-x-3">
                                    <button class="text-indigo-300 hover:text-indigo-200" on:click={
                                        let user = user.clone();
                                        move |_| on_edit.run(user.clone())
                                    }>
                                        "Edit"
                                    </button>
                                    <button class="text-rose-300 hover:text-rose-200" on:click={
                                        let delete_id = delete_id.clone();
                                        move |_| on_delete.run(delete_id.clone())
                                    }>
                                        "Delete"
                                    </button>
                                </td>
                            </tr>
                        }
                    }
                />
            </tbody>
        </table>
    }
}

#[component]
pub fn UserModal(
    open: ReadSignal<bool>,
    on_close: Callback<()>,
    on_submit: Callback<User>,
    initial: Option<User>,
) -> impl IntoView {
    let (name, set_name) = signal(initial.as_ref().map(|u| u.name.clone()).unwrap_or_default());
    let (email, set_email) = signal(
        initial
            .as_ref()
            .map(|u| u.email.clone())
            .unwrap_or_default(),
    );

    view! {
        <Show when=move || open.get()>
            <div class="fixed inset-0 bg-black/60 flex items-center justify-center p-6">
                <div class="w-full max-w-lg bg-slate-900 border border-slate-700 rounded-xl p-6 space-y-4">
                    <h3 class="text-lg text-slate-100 font-semibold">"User"</h3>
                    <input class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-slate-100" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
                    <input class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-slate-100" prop:value=email on:input=move |ev| set_email.set(event_target_value(&ev)) />
                    <div class="flex justify-end gap-3">
                        <button class="px-3 py-2 rounded-lg bg-slate-800 text-slate-200" on:click=move |_| on_close.run(())>"Cancel"</button>
                        <button class="px-3 py-2 rounded-lg bg-indigo-600 text-white" on:click=move |_| {
                            on_submit.run(User {
                                id: initial.as_ref().map(|u| u.id.clone()).unwrap_or_else(|| "usr_new".into()),
                                name: name.get(),
                                email: email.get(),
                                role: initial.as_ref().map(|u| u.role.clone()).unwrap_or(Role::Viewer),
                                status: initial.as_ref().map(|u| u.status.clone()).unwrap_or(UserStatus::Invited),
                                created_at: chrono::Utc::now(),
                            });
                            on_close.run(());
                        }>
                            "Save"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
