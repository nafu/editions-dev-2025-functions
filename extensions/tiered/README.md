# Tiered Discount Campaign Function

A configurable Shopify Function that implements a tiered discount system with the following features:

## Features

1. **Quantity-based discounts**: 5% off items when buying 2+ of any item
2. **Order value discounts**: 10% off subtotal when spending $100+
3. **Free shipping**: Free shipping when spending $200+

## Configuration

The function reads configuration from a metafield with namespace "tiered" and key "config". The configuration is a JSON object with the following structure:

```json
{
  "quantity_threshold": 2,
  "quantity_discount_percentage": 5.0,
  "order_threshold_1": 100.0,
  "order_discount_percentage_1": 10.0,
  "free_shipping_threshold": 200.0
}
```

### Configuration Options

- `quantity_threshold`: Minimum quantity to trigger item discount (default: 2)
- `quantity_discount_percentage`: Percentage discount for qualifying items (default: 5.0)
- `order_threshold_1`: Minimum order value for order discount (default: 100.0)
- `order_discount_percentage_1`: Percentage discount for order subtotal (default: 10.0)
- `free_shipping_threshold`: Minimum order value for free shipping (default: 200.0)

## Usage

1. Deploy the function to your Shopify store
2. Create a discount with the appropriate discount classes:
   - `PRODUCT` for quantity-based discounts
   - `ORDER` for order value discounts  
   - `SHIPPING` for free shipping
3. Optionally add a metafield to the discount with namespace "tiered" and key "config" containing your JSON configuration

## Default Behavior

If no configuration metafield is provided, the function uses these defaults:
- 5% off items when buying 2 or more
- 10% off order subtotal when spending $100+
- Free shipping when spending $200+

## Building

```bash
cargo build --target=wasm32-wasip1 --release
```

The compiled WASM file will be available at `target/wasm32-wasip1/release/tiered.wasm`.

## Dependencies

- [Install Rust](https://www.rust-lang.org/tools/install)
  - On Windows, Rust requires the [Microsoft C++ Build Tools](https://docs.microsoft.com/en-us/windows/dev-environment/rust/setup). Be sure to select the _Desktop development with C++_ workload when installing them.
