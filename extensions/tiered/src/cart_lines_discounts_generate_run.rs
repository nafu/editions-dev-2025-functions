use crate::schema::CartLineTarget;
use crate::schema::CartLinesDiscountsGenerateRunResult;
use crate::schema::CartOperation;
use crate::schema::DiscountClass;
use crate::schema::OrderDiscountCandidate;
use crate::schema::OrderDiscountCandidateTarget;
use crate::schema::OrderDiscountCandidateValue;
use crate::schema::OrderDiscountSelectionStrategy;
use crate::schema::OrderDiscountsAddOperation;
use crate::schema::OrderSubtotalTarget;
use crate::schema::Percentage;
use crate::schema::ProductDiscountCandidate;
use crate::schema::ProductDiscountCandidateTarget;
use crate::schema::ProductDiscountCandidateValue;
use crate::schema::ProductDiscountSelectionStrategy;
use crate::schema::ProductDiscountsAddOperation;

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
fn cart_lines_discounts_generate_run(
    input: schema::cart_lines_discounts_generate_run::Input,
) -> Result<CartLinesDiscountsGenerateRunResult> {
    let config = if let Some(metafield) = input.discount().metafield() {
        let value = metafield.value();
        serde_json::from_str::<TieredDiscountConfig>(value.as_str())
            .unwrap_or_else(|_| TieredDiscountConfig::default())
    } else {
        TieredDiscountConfig::default()
    };

    let cart_lines = input.cart().lines();
    let cart_subtotal = input.cart().cost().subtotal_amount().amount();

    let has_order_discount_class = input
        .discount()
        .discount_classes()
        .contains(&DiscountClass::Order);
    let has_product_discount_class = input
        .discount()
        .discount_classes()
        .contains(&DiscountClass::Product);

    if !has_order_discount_class && !has_product_discount_class {
        return Ok(CartLinesDiscountsGenerateRunResult { operations: vec![] });
    }

    let mut operations = vec![];

    // Product-level discounts: 5% off items when buying 2+ of any item
    if has_product_discount_class {
        for line in cart_lines {
            if *line.quantity() >= config.quantity_threshold {
                operations.push(CartOperation::ProductDiscountsAdd(
                    ProductDiscountsAddOperation {
                        selection_strategy: ProductDiscountSelectionStrategy::First,
                        candidates: vec![ProductDiscountCandidate {
                            targets: vec![ProductDiscountCandidateTarget::CartLine(CartLineTarget {
                                id: line.id().clone(),
                                quantity: None,
                            })],
                            message: Some(format!("{}% OFF - Buy {} or more", config.quantity_discount_percentage, config.quantity_threshold)),
                            value: ProductDiscountCandidateValue::Percentage(Percentage {
                                value: Decimal(config.quantity_discount_percentage),
                            }),
                            associated_discount_code: None,
                        }],
                    },
                ));
            }
        }
    }

    // Order-level discount: 10% off subtotal when spending $100+
    if has_order_discount_class && cart_subtotal.0 >= config.order_threshold_1 {
        operations.push(CartOperation::OrderDiscountsAdd(
            OrderDiscountsAddOperation {
                selection_strategy: OrderDiscountSelectionStrategy::First,
                candidates: vec![OrderDiscountCandidate {
                    targets: vec![OrderDiscountCandidateTarget::OrderSubtotal(
                        OrderSubtotalTarget {
                            excluded_cart_line_ids: vec![],
                        },
                    )],
                    message: Some(format!("{}% OFF ORDER - Spend ${}+", config.order_discount_percentage_1, config.order_threshold_1)),
                    value: OrderDiscountCandidateValue::Percentage(Percentage {
                        value: Decimal(config.order_discount_percentage_1),
                    }),
                    conditions: None,
                    associated_discount_code: None,
                }],
            },
        ));
    }

    Ok(CartLinesDiscountsGenerateRunResult { operations })
}
