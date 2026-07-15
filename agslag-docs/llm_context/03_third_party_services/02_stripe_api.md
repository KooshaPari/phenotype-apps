# Stripe API Integration

## Overview
Stripe provides payment processing APIs for:
- Payment processing
- Subscription management
- Invoicing
- Payment links
- Fraud prevention

## Authentication
Stripe uses API keys for authentication:
- Test keys (pk_test_*, sk_test_*)
- Live keys (pk_live_*, sk_live_*)
- Restricted keys with limited permissions

## Core Resources

### Payments
- `POST /v1/payment_intents` - Create payment intent
- `GET /v1/payment_intents/{id}` - Retrieve payment intent
- `POST /v1/payment_intents/{id}/confirm` - Confirm payment

### Customers
- `POST /v1/customers` - Create customer
- `GET /v1/customers/{id}` - Retrieve customer
- `POST /v1/customers/{id}` - Update customer

### Products & Prices
- `POST /v1/products` - Create product
- `POST /v1/prices` - Create price
- `GET /v1/prices` - List prices

### Subscriptions
- `POST /v1/subscriptions` - Create subscription
- `GET /v1/subscriptions/{id}` - Retrieve subscription
- `POST /v1/subscriptions/{id}` - Update subscription

## Webhooks
- Configure in Stripe Dashboard
- Validate signature using `stripe.webhooks.constructEvent`
- Handle events like `payment_intent.succeeded`, `invoice.payment_failed`

## Testing
- Test mode with test API keys
- Test cards: 4242 4242 4242 4242 (success), 4000 0000 0000 0002 (declined)
- Test bank accounts, ACH, and other payment methods

## Error Handling
- HTTP status codes (400, 401, 402, 404, 429)
- Error types (card_error, validation_error, rate_limit_error)
- Error handling with try/catch

## Best Practices
- Store customer IDs, not card data
- Use idempotency keys for API requests
- Implement proper webhook handling
- Follow PCI compliance guidelines