use crate::domain::models::{
    Activity, ChartPoint, KpiCard, ProfileSettings, Role, SystemConfig, User, UserStatus,
};
use crate::infrastructure::api_client::ApiError;
use chrono::Utc;

pub fn filter_and_sort_users(
    users: &[User],
    query: &str,
    sort_desc: bool,
    page: usize,
    page_size: usize,
) -> Vec<User> {
    let mut filtered = users
        .iter()
        .filter(|u| {
            let q = query.to_ascii_lowercase();
            q.is_empty()
                || u.name.to_ascii_lowercase().contains(&q)
                || u.email.to_ascii_lowercase().contains(&q)
        })
        .cloned()
        .collect::<Vec<_>>();

    filtered.sort_by(|a, b| {
        if sort_desc {
            b.created_at.cmp(&a.created_at)
        } else {
            a.created_at.cmp(&b.created_at)
        }
    });

    let start = page.saturating_mul(page_size);
    filtered.into_iter().skip(start).take(page_size).collect()
}

pub fn dashboard_kpis() -> Vec<KpiCard> {
    vec![
        KpiCard {
            label: "Users",
            value: "14,208",
            delta: "+8.2%",
            positive: true,
        },
        KpiCard {
            label: "Revenue",
            value: "$278,900",
            delta: "+12.6%",
            positive: true,
        },
        KpiCard {
            label: "Transactions",
            value: "41,982",
            delta: "-1.4%",
            positive: false,
        },
    ]
}

pub fn line_series() -> Vec<ChartPoint> {
    vec![
        ChartPoint {
            label: "Jan",
            value: 32,
        },
        ChartPoint {
            label: "Feb",
            value: 38,
        },
        ChartPoint {
            label: "Mar",
            value: 52,
        },
        ChartPoint {
            label: "Apr",
            value: 47,
        },
        ChartPoint {
            label: "May",
            value: 61,
        },
        ChartPoint {
            label: "Jun",
            value: 68,
        },
    ]
}

pub fn bar_series() -> Vec<ChartPoint> {
    vec![
        ChartPoint {
            label: "API",
            value: 91,
        },
        ChartPoint {
            label: "Web",
            value: 76,
        },
        ChartPoint {
            label: "Mobile",
            value: 43,
        },
        ChartPoint {
            label: "Partners",
            value: 55,
        },
    ]
}

pub fn recent_activity() -> Vec<Activity> {
    vec![
        Activity {
            id: "act_1".into(),
            actor: "Monica Lee".into(),
            action: "approved".into(),
            target: "Enterprise contract #7821".into(),
            at: Utc::now(),
        },
        Activity {
            id: "act_2".into(),
            actor: "Sven I".into(),
            action: "invited".into(),
            target: "anna@acme.io".into(),
            at: Utc::now(),
        },
        Activity {
            id: "act_3".into(),
            actor: "Billing Bot".into(),
            action: "flagged".into(),
            target: "24 failed payments".into(),
            at: Utc::now(),
        },
    ]
}

pub fn seed_users() -> Result<Vec<User>, ApiError> {
    Ok(vec![
        User {
            id: "usr_001".into(),
            name: "Ava Patel".into(),
            email: "ava@northwind.com".into(),
            role: Role::Admin,
            status: UserStatus::Active,
            created_at: Utc::now(),
        },
        User {
            id: "usr_002".into(),
            name: "Liam Carter".into(),
            email: "liam@northwind.com".into(),
            role: Role::Operator,
            status: UserStatus::Invited,
            created_at: Utc::now(),
        },
        User {
            id: "usr_003".into(),
            name: "Nora Chen".into(),
            email: "nora@northwind.com".into(),
            role: Role::Viewer,
            status: UserStatus::Suspended,
            created_at: Utc::now(),
        },
    ])
}

pub fn default_profile() -> ProfileSettings {
    ProfileSettings {
        full_name: "Jordan Doe".into(),
        timezone: "America/New_York".into(),
        locale: "en-US".into(),
    }
}

pub fn default_config() -> SystemConfig {
    SystemConfig {
        allow_signups: false,
        default_role: Role::Viewer,
        notify_security_events: true,
    }
}
