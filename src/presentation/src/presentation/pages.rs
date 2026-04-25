use crate::application::services;
use crate::domain::models::User;
use crate::infrastructure::api_client;
use crate::presentation::components::{KpiCard, UserModal, UsersTable};
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <section class="min-h-screen bg-slate-950 flex items-center justify-center p-6">
            <div class="w-full max-w-md rounded-xl border border-slate-800 bg-slate-900 p-8 space-y-5">
                <h1 class="text-2xl text-slate-100 font-semibold">"Welcome back"</h1>
                <p class="text-sm text-slate-400">"Sign in to continue to Atlas Admin."</p>
                <input class="w-full rounded-lg bg-slate-950 border border-slate-700 px-3 py-2" placeholder="Email" />
                <input class="w-full rounded-lg bg-slate-950 border border-slate-700 px-3 py-2" type="password" placeholder="Password" />
                <button class="w-full rounded-lg bg-indigo-600 text-white py-2">"Sign in"</button>
            </div>
        </section>
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let kpis = services::dashboard_kpis();
    let line = services::line_series();
    let bars = services::bar_series();
    let activity = services::recent_activity();

    view! {
        <div class="space-y-8">
            <section class="grid md:grid-cols-3 gap-4">
                {kpis.into_iter().map(|k| view! { <KpiCard title=k.label value=k.value delta=k.delta positive=k.positive /> }).collect_view()}
            </section>

            <section class="grid lg:grid-cols-2 gap-6">
                <article class="rounded-xl border border-slate-800 bg-slate-900 p-5">
                    <h3 class="text-slate-100 font-medium">"Revenue Trend"</h3>
                    <div class="mt-4 h-56 flex items-end gap-2">
                        {line.into_iter().map(|p| view! {
                            <div class="flex-1 flex flex-col items-center gap-2">
                                <div class="w-full bg-indigo-500/80 rounded-t" style=format!("height: {}%;", p.value)></div>
                                <span class="text-xs text-slate-500">{p.label}</span>
                            </div>
                        }).collect_view()}
                    </div>
                </article>
                <article class="rounded-xl border border-slate-800 bg-slate-900 p-5">
                    <h3 class="text-slate-100 font-medium">"Traffic by Channel"</h3>
                    <div class="mt-4 space-y-3">
                        {bars.into_iter().map(|p| view! {
                            <div>
                                <div class="flex justify-between text-sm text-slate-300"><span>{p.label}</span><span>{p.value}"%"</span></div>
                                <div class="h-2 bg-slate-800 rounded mt-1">
                                    <div class="h-2 bg-cyan-500 rounded" style=format!("width: {}%;", p.value)></div>
                                </div>
                            </div>
                        }).collect_view()}
                    </div>
                </article>
            </section>

            <section class="rounded-xl border border-slate-800 bg-slate-900 p-5">
                <h3 class="text-slate-100 font-medium mb-4">"Recent Activity"</h3>
                <table class="w-full text-sm">
                    <thead class="text-slate-400 border-b border-slate-800">
                        <tr><th class="pb-2 text-left">"Actor"</th><th class="pb-2 text-left">"Action"</th><th class="pb-2 text-left">"Target"</th></tr>
                    </thead>
                    <tbody>
                    {activity.into_iter().map(|a| view! {
                        <tr class="border-b border-slate-800/50"><td class="py-3">{a.actor}</td><td>{a.action}</td><td>{a.target}</td></tr>
                    }).collect_view()}
                    </tbody>
                </table>
            </section>
        </div>
    }
}

#[component]
pub fn UsersPage() -> impl IntoView {
    let (all_users, set_all_users) = signal(Vec::<User>::new());
    let (query, set_query) = signal(String::new());
    let (sort_desc, set_sort_desc) = signal(true);
    let (page, set_page) = signal(0usize);
    let page_size = 10usize;
    let (modal_open, set_modal_open) = signal(false);
    let (editing_user, set_editing_user) = signal(None::<User>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    Effect::new(move |_| {
        spawn_local(async move {
            match api_client::fetch_users().await {
                Ok(users) => {
                    set_all_users.set(users);
                    set_error.set(None);
                }
                Err(err) => set_error.set(Some(err.to_string())),
            }
            set_loading.set(false);
        });
    });

    let paged_users = Memo::new(move |_| {
        services::filter_and_sort_users(
            &all_users.get(),
            &query.get(),
            sort_desc.get(),
            page.get(),
            page_size,
        )
    });

    let on_edit = Callback::new(move |user: User| {
        set_editing_user.set(Some(user));
        set_modal_open.set(true);
    });

    let on_delete = Callback::new(move |id: String| {
        set_all_users.update(|users| users.retain(|u| u.id != id));
    });

    let on_submit = Callback::new(move |user: User| {
        set_all_users.update(|users| {
            if let Some(current) = users.iter_mut().find(|u| u.id == user.id) {
                *current = user;
            } else {
                users.push(user);
            }
        });
    });

    view! {
        <section class="space-y-5">
            <div class="flex flex-wrap items-center justify-between gap-3">
                <h1 class="text-xl font-semibold">"Users"</h1>
                <div class="flex gap-2">
                    <input class="bg-slate-900 border border-slate-700 rounded-lg px-3 py-2" placeholder="Filter by name/email" on:input=move |ev| set_query.set(event_target_value(&ev)) />
                    <button class="bg-slate-800 px-3 rounded-lg" on:click=move |_| set_sort_desc.update(|s| *s = !*s)>"Toggle sort"</button>
                    <button class="bg-indigo-600 px-3 rounded-lg" on:click=move |_| { set_editing_user.set(None); set_modal_open.set(true); } >"New User"</button>
                </div>
            </div>

            <Show when=move || !loading.get() fallback=|| view! { <p class="text-slate-400">"Loading users..."</p> }>
                <Show when=move || error.get().is_none() fallback=move || view! { <p class="text-rose-400">{error.get().unwrap_or_default()}</p> }>
                    <div class="rounded-xl border border-slate-800 bg-slate-900 p-4 overflow-x-auto">
                        <UsersTable users=Signal::derive(move || paged_users.get()) on_edit=on_edit on_delete=on_delete />
                    </div>
                    <div class="flex justify-end gap-2">
                        <button class="bg-slate-800 px-3 py-2 rounded-lg" on:click=move |_| set_page.update(|p| *p = p.saturating_sub(1))>"Prev"</button>
                        <button class="bg-slate-800 px-3 py-2 rounded-lg" on:click=move |_| set_page.update(|p| *p += 1)>"Next"</button>
                    </div>
                </Show>
            </Show>

            <UserModal open=modal_open on_close=Callback::new(move |_| set_modal_open.set(false)) on_submit=on_submit initial=editing_user.get() />
        </section>
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <section class="grid lg:grid-cols-2 gap-6">
            <article class="rounded-xl border border-slate-800 bg-slate-900 p-5 space-y-3">
                <h2 class="font-medium">"Profile Settings"</h2>
                <input class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2" value="Jordan Doe" />
                <input class="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2" value="America/New_York" />
                <button class="bg-indigo-600 px-3 py-2 rounded-lg">"Save profile"</button>
            </article>
            <article class="rounded-xl border border-slate-800 bg-slate-900 p-5 space-y-3">
                <h2 class="font-medium">"System Configuration"</h2>
                <label class="flex items-center justify-between"><span>"Allow self-service signup"</span><input type="checkbox" /></label>
                <label class="flex items-center justify-between"><span>"Security alerts"</span><input type="checkbox" checked /></label>
                <button class="bg-indigo-600 px-3 py-2 rounded-lg">"Save config"</button>
            </article>
        </section>
    }
}

#[component]
pub fn NotificationsPage() -> impl IntoView {
    view! {
        <section class="rounded-xl border border-slate-800 bg-slate-900 p-6">
            <h1 class="text-xl font-semibold">"Notifications"</h1>
            <p class="mt-2 text-slate-400">"No new alerts. You'll see incident updates and policy changes here."</p>
        </section>
    }
}
