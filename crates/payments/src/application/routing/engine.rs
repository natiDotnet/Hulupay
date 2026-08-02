use crate::domain::merchant_config::MerchantConfig;
use crate::domain::merchant_routing_rule::MerchantRoutingRule;
use crate::domain::merchant_routing_strategy::MerchantRoutingStrategy;
use crate::domain::payment_provider::PaymentProvider;
use crate::domain::provider_metric::ProviderMetric;
use crate::domain::provider_status::ProviderStatus;
use crate::domain::routing_strategy::RoutingStrategy;
use hulu_core::payment_request::PaymentRequest;
use rust_decimal::Decimal;
use toasty::Db;
use tracing::debug;
use uuid::Uuid;

/// The result of routing: an ordered list of provider IDs (first = preferred).
pub type RoutingResult = Vec<Uuid>;

#[derive(Clone)]
pub struct RoutingEngine {
    db: Db,
}

impl RoutingEngine {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    /// Pick the best provider for a payment, returning an ordered fallback list.
    pub async fn resolve(
        &self,
        merchant_id: Uuid,
        request: &PaymentRequest,
    ) -> anyhow::Result<RoutingResult> {
        let mut db = self.db.clone();
        let offline_providers = self.get_offline_provider_ids(&mut db).await?;

        // 1) Evaluate explicit rules first (by priority)
        let rules = MerchantRoutingRule::filter(
            MerchantRoutingRule::fields()
                .merchant_id()
                .eq(merchant_id)
                .and(MerchantRoutingRule::fields().enabled().eq(true)),
        )
        .order_by(MerchantRoutingRule::fields().priority().asc())
        .exec(&mut db)
        .await?;

        for rule in &rules {
            if matches_rule(rule, request) {
                let target = rule.target_provider_id;
                if !offline_providers.contains(&target) {
                    let mut result = vec![target];
                    if let Some(fb) = rule.fallback_provider_id {
                        if !offline_providers.contains(&fb) {
                            result.push(fb);
                        }
                    }
                    debug!(?rule, "Rule matched");
                    return Ok(result);
                }
            }
        }

        // 2) Fall back to the merchant's configured strategy
        let strategy = self.get_strategy(&mut db, merchant_id).await.unwrap_or_default();

        // Load configs, then load each provider separately (replaces find_also_related).
        let configs = MerchantConfig::filter(
            MerchantConfig::fields()
                .merchant_id()
                .eq(merchant_id)
                .and(MerchantConfig::fields().is_active().eq(true)),
        )
        .exec(&mut db)
        .await?;

        let mut healthy: Vec<(MerchantConfig, PaymentProvider)> = Vec::new();
        for config in configs {
            if let Ok(Some(provider)) = PaymentProvider::filter_by_id(config.provider_id)
                .first()
                .exec(&mut db)
                .await
            {
                if provider.is_active && !offline_providers.contains(&provider.id) {
                    healthy.push((config, provider));
                }
            }
        }

        let result = match strategy {
            RoutingStrategy::Default => {
                // Use the is_default provider
                let mut ordered = healthy
                    .iter()
                    .filter(|(c, _)| c.is_default)
                    .map(|(_, p)| p.id)
                    .collect::<Vec<_>>();
                // Add all others as fallbacks
                for (_, p) in &healthy {
                    if !ordered.contains(&p.id) {
                        ordered.push(p.id);
                    }
                }
                ordered
            }
            RoutingStrategy::Priority => {
                let mut sorted = healthy;
                sorted.sort_by_key(|(c, _)| c.priority);
                sorted.into_iter().map(|(_, p)| p.id).collect()
            }
            RoutingStrategy::PaymentMethod => {
                let method = request
                    .payment
                    .payment_methods
                    .first()
                    .cloned()
                    .unwrap_or_default();
                route_by_method(&healthy, &method)
            }
            RoutingStrategy::Currency => {
                route_by_currency(&healthy, &request.payment.currency)
            }
            RoutingStrategy::Amount => {
                route_by_amount(&healthy, request.payment.amount)
            }
            RoutingStrategy::Country => {
                let country = request.payment.lang.clone().unwrap_or_default();
                route_by_country(&healthy, &country)
            }
            RoutingStrategy::LowestCost => {
                let mut sorted = healthy;
                sorted.sort_by(|(a, _), (b, _)| {
                    let fee_a = a.config.get("fee").and_then(|v| v.as_f64()).unwrap_or(100.0);
                    let fee_b = b.config.get("fee").and_then(|v| v.as_f64()).unwrap_or(100.0);
                    fee_a.partial_cmp(&fee_b).unwrap_or(std::cmp::Ordering::Equal)
                });
                sorted.into_iter().map(|(_, p)| p.id).collect()
            }
            RoutingStrategy::RuleBased => {
                // Rules already evaluated above and didn't match; fall to default
                let mut ordered = healthy
                    .iter()
                    .filter(|(c, _)| c.is_default)
                    .map(|(_, p)| p.id)
                    .collect::<Vec<_>>();
                for (_, p) in &healthy {
                    if !ordered.contains(&p.id) {
                        ordered.push(p.id);
                    }
                }
                ordered
            }
        };

        if result.is_empty() {
            anyhow::bail!("No healthy provider available for merchant {}", merchant_id);
        }

        debug!(?result, "Routing resolved");
        Ok(result)
    }

    /// Read the merchant's routing strategy, return Default if none set.
    async fn get_strategy(
        &self,
        db: &mut Db,
        merchant_id: Uuid,
    ) -> Option<RoutingStrategy> {
        let row = MerchantRoutingStrategy::filter(
            MerchantRoutingStrategy::fields().merchant_id().eq(merchant_id),
        )
        .first()
        .exec(db)
        .await
        .ok()??;
        if row.enabled {
            Some(row.strategy)
        } else {
            None
        }
    }

    /// Get all provider IDs currently marked as Offline.
    async fn get_offline_provider_ids(
        &self,
        db: &mut Db,
    ) -> anyhow::Result<Vec<Uuid>> {
        let rows = ProviderMetric::filter(
            ProviderMetric::fields()
                .current_status()
                .eq(ProviderStatus::Offline),
        )
        .exec(db)
        .await?;
        Ok(rows.into_iter().map(|r| r.provider_id).collect())
    }
}

// ── Rule matching helpers ────────────────────────────────────────────

fn matches_rule(rule: &MerchantRoutingRule, request: &PaymentRequest) -> bool {
    match rule.condition_type {
        crate::domain::routing_rule::ConditionType::PaymentMethod => {
            let method = request
                .payment
                .payment_methods
                .first()
                .cloned()
                .unwrap_or_default()
                .to_uppercase();
            rule.condition_value.to_uppercase() == method
        }
        crate::domain::routing_rule::ConditionType::Currency => {
            rule.condition_value.to_uppercase() == request.payment.currency.to_uppercase()
        }
        crate::domain::routing_rule::ConditionType::Country => {
            let country = request
                .payment
                .lang
                .as_deref()
                .unwrap_or("")
                .to_uppercase();
            rule.condition_value.to_uppercase() == country
        }
        crate::domain::routing_rule::ConditionType::AmountGreaterThan => {
            let threshold: f64 = rule.condition_value.parse().unwrap_or(0.0);
            let amount: f64 = request.payment.amount.to_string().parse().unwrap_or(0.0);
            amount > threshold
        }
        crate::domain::routing_rule::ConditionType::AmountLessThan => {
            let threshold: f64 = rule.condition_value.parse().unwrap_or(0.0);
            let amount: f64 = request.payment.amount.to_string().parse().unwrap_or(0.0);
            amount < threshold
        }
    }
}

fn route_by_method(
    providers: &[(MerchantConfig, PaymentProvider)],
    method: &str,
) -> Vec<Uuid> {
    // Try to find a provider whose config mentions this method
    let upper = method.to_uppercase();
    let mut matched: Vec<_> = providers
        .iter()
        .filter(|(_, p)| p.name.to_uppercase().contains(&upper))
        .map(|(_, p)| p.id)
        .collect();
    if matched.is_empty() {
        // Fallback: all providers
        matched = providers.iter().map(|(_, p)| p.id).collect();
    }
    matched
}

fn route_by_currency(
    providers: &[(MerchantConfig, PaymentProvider)],
    currency: &str,
) -> Vec<Uuid> {
    let upper = currency.to_uppercase();
    let mut matched: Vec<_> = providers
        .iter()
        .filter(|(c, _)| {
            c.config
                .get("supported_currencies")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().any(|s| s.as_str().unwrap_or("").to_uppercase() == upper))
                .unwrap_or(true)
        })
        .map(|(_, p)| p.id)
        .collect();
    if matched.is_empty() {
        matched = providers.iter().map(|(_, p)| p.id).collect();
    }
    matched
}

fn route_by_amount(
    providers: &[(MerchantConfig, PaymentProvider)],
    _amount: Decimal,
) -> Vec<Uuid> {
    let mut sorted = providers.to_vec();
    sorted.sort_by_key(|(c, _)| c.priority);
    sorted.into_iter().map(|(_, p)| p.id).collect()
}

fn route_by_country(
    providers: &[(MerchantConfig, PaymentProvider)],
    country: &str,
) -> Vec<Uuid> {
    let upper = country.to_uppercase();
    let mut matched: Vec<_> = providers
        .iter()
        .filter(|(c, _)| {
            c.config
                .get("supported_countries")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().any(|s| s.as_str().unwrap_or("").to_uppercase() == upper))
                .unwrap_or(true)
        })
        .map(|(_, p)| p.id)
        .collect();
    if matched.is_empty() {
        matched = providers.iter().map(|(_, p)| p.id).collect();
    }
    matched
}
