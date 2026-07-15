# Shopify API Integration

## Overview
Shopify provides REST and GraphQL APIs for e-commerce functionality including:
- Product management
- Order processing
- Customer data
- Storefront customization
- Payment processing

## Authentication
Shopify uses OAuth 2.0 for authentication:
1. Register app in Shopify Partner Dashboard
2. Obtain API key and secret
3. Request access scopes
4. Implement OAuth flow
5. Store and refresh access tokens

## Key Endpoints

### Products
- `GET /admin/api/2025-04/products.json` - List products
- `POST /admin/api/2025-04/products.json` - Create product
- `PUT /admin/api/2025-04/products/{id}.json` - Update product

### Orders
- `GET /admin/api/2025-04/orders.json` - List orders
- `POST /admin/api/2025-04/orders/{id}/close.json` - Close order
- `POST /admin/api/2025-04/orders/{id}/cancel.json` - Cancel order

### Customers
- `GET /admin/api/2025-04/customers.json` - List customers
- `POST /admin/api/2025-04/customers.json` - Create customer
- `PUT /admin/api/2025-04/customers/{id}.json` - Update customer

## GraphQL API
GraphQL endpoint: `https://{shop}.myshopify.com/admin/api/2025-04/graphql.json`

Example query:
```graphql
query {
  products(first: 10) {
    edges {
      node {
        id
        title
        description
        variants(first: 5) {
          edges {
            node {
              id
              price
              inventoryQuantity
            }
          }
        }
      }
    }
  }
}
```

## Rate Limits
- REST API: 40 requests per minute (per IP)
- GraphQL: 1000 cost points per minute (queries have variable costs)

## Best Practices
- Use bulk operations for large datasets
- Implement webhook listeners for real-time updates
- Cache responses when appropriate
- Handle rate limiting with exponential backoff