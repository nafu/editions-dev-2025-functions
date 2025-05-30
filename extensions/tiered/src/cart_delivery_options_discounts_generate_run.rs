use crate::schema::CartDeliveryOptionsDiscountsGenerateRunResult;
use crate::schema::DeliveryDiscountCandidate;
use crate::schema::DeliveryDiscountCandidateTarget;
use crate::schema::DeliveryDiscountCandidateValue;
use crate::schema::DeliveryDiscountSelectionStrategy;
use crate::schema::DeliveryDiscountsAddOperation;
use crate::schema::DeliveryGroupTarget;
use crate::schema::DeliveryOperation;
use crate::schema::DiscountClass;
use crate::schema::Percentage;

use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct TieredDiscountConfig {
    quantity_threshold: i32,
    quantity_discount_percentage: f64,
    order_threshold_1: f64,
    order_discount_percentage_1: f64,
    free_shipping_threshold: f64,
}

impl Default for TieredDiscountConfig {
    fn default() -> Self {
        Self {
            quantity_threshold: 2,
            quantity_discount_percentage: 5.0,
            order_threshold_1: 100.0,
            order_discount_percentage_1: 10.0,
            free_shipping_threshold: 200.0,
        }
    }
}

#[shopify_function]
fn cart_delivery_options_discounts_generate_run(
    input: schema::cart_delivery_options_discounts_generate_run::Input,
) -> Result<CartDeliveryOptionsDiscountsGenerateRunResult> {
    let config = if let Some(metafield) = input.discount().metafield() {
        let value = metafield.value();
        serde_json::from_str::<TieredDiscountConfig>(value.as_str())
            .unwrap_or_else(|_| TieredDiscountConfig::default())
    } else {
        TieredDiscountConfig::default()
    };

    let cart_subtotal = input.cart().cost().subtotal_amount().amount();

    let has_shipping_discount_class = input
        .discount()
        .discount_classes()
        .contains(&DiscountClass::Shipping);

    if !has_shipping_discount_class || cart_subtotal.0 < config.free_shipping_threshold {
        return Ok(CartDeliveryOptionsDiscountsGenerateRunResult { operations: vec![] });
    }

    let first_delivery_group = input
        .cart()
        .delivery_groups()
        .first()
        .ok_or("No delivery groups found")?;

    Ok(CartDeliveryOptionsDiscountsGenerateRunResult {
        operations: vec![DeliveryOperation::DeliveryDiscountsAdd(
            DeliveryDiscountsAddOperation {
                selection_strategy: DeliveryDiscountSelectionStrategy::All,
                candidates: vec![DeliveryDiscountCandidate {
                    targets: vec![DeliveryDiscountCandidateTarget::DeliveryGroup(
                        DeliveryGroupTarget {
                            id: first_delivery_group.id().clone(),
                        },
                    )],
                    value: DeliveryDiscountCandidateValue::Percentage(Percentage {
                        value: Decimal(100.0),
                    }),
                    message: Some(format!("FREE SHIPPING - Spend ${}+", config.free_shipping_threshold)),
                    associated_discount_code: None,
                }],
            },
        )],
    })
}
