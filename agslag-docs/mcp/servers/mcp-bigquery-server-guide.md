# MCP BigQuery Server Guide

## Overview

The MCP BigQuery Server provides AI assistants with secure, controlled access to Google BigQuery datasets. This server enables AI models to query data, explore schemas, and analyze results from BigQuery tables, making it an essential tool for data analysis and business intelligence tasks.

## Features

- **SQL Query Execution**: Run SQL queries against BigQuery datasets
- **Schema Exploration**: Discover available datasets, tables, and their structures
- **Query Validation**: Verify SQL syntax before execution
- **Result Pagination**: Handle large result sets efficiently
- **Cost Estimation**: Preview query costs before execution
- **Access Controls**: Restrict access to specific datasets and tables
- **Query History**: Track and reuse previous queries
- **Data Visualization**: Generate basic visualizations from query results

## Installation

### Prerequisites

- Node.js 16 or higher
- Google Cloud account with BigQuery access
- Service account with appropriate permissions

### Using NPX (Recommended)

```bash
npx @modelcontextprotocol/server-bigquery
```

### Global Installation

```bash
npm install -g @modelcontextprotocol/server-bigquery
mcp-bigquery
```

### From Source

```bash
git clone https://github.com/modelcontextprotocol/server-bigquery.git
cd server-bigquery
npm install
npm start
```

## Configuration

### Authentication

The server supports multiple authentication methods:

1. **Service Account Key**:
   ```json
   {
     "auth": {
       "type": "service_account",
       "keyFile": "/path/to/service-account-key.json"
     }
   }
   ```

2. **Application Default Credentials**:
   ```json
   {
     "auth": {
       "type": "application_default"
     }
   }
   ```

3. **Explicit Credentials**:
   ```json
   {
     "auth": {
       "type": "explicit",
       "projectId": "your-project-id",
       "clientEmail": "your-client-email",
       "privateKey": "your-private-key"
     }
   }
   ```

### Access Controls

Restrict access to specific projects, datasets, and tables:

```json
{
  "accessControls": {
    "allowedProjects": ["project-1", "project-2"],
    "allowedDatasets": {
      "project-1": ["dataset-a", "dataset-b"],
      "project-2": ["*"]
    },
    "allowedTables": {
      "project-1.dataset-a": ["table-1", "table-2"],
      "project-1.dataset-b": ["*"],
      "project-2.*": ["*"]
    }
  }
}
```

### Query Limits

Set limits on query execution:

```json
{
  "queryLimits": {
    "maxBytes": 10737418240,  // 10 GB
    "maxRows": 10000,
    "timeoutMs": 30000,       // 30 seconds
    "maxConcurrentQueries": 5
  }
}
```

## Usage with Claude

### Basic Query Example

```
Please query our BigQuery database to find the top 10 products by revenue for Q1 2023.
```

Claude might respond:

```
I'll query the BigQuery database to find the top 10 products by revenue for Q1 2023.

<mcp:bigquery.execute_query>
<query>
SELECT 
  p.product_name,
  SUM(o.quantity * o.unit_price) AS revenue
FROM 
  `project-id.sales.orders` o
JOIN 
  `project-id.products.catalog` p ON o.product_id = p.product_id
WHERE 
  o.order_date BETWEEN '2023-01-01' AND '2023-03-31'
GROUP BY 
  p.product_name
ORDER BY 
  revenue DESC
LIMIT 10
</query>
</mcp:bigquery.execute_query>

Based on the query results, here are the top 10 products by revenue for Q1 2023:

1. Product A: $1,245,678
2. Product B: $987,654
3. Product C: $876,543
...
```

### Schema Exploration Example

```
What tables are available in our sales dataset?
```

Claude might respond:

```
Let me check what tables are available in the sales dataset.

<mcp:bigquery.list_tables>
<projectId>project-id</projectId>
<datasetId>sales</datasetId>
</mcp:bigquery.list_tables>

The sales dataset contains the following tables:
1. orders
2. customers
3. returns
4. promotions
5. sales_reps

Would you like me to describe the schema of any of these tables?
```

## Available Tools

### Query Execution

- **execute_query**: Run a SQL query against BigQuery
  - Parameters:
    - `query`: SQL query string
    - `maxRows` (optional): Maximum number of rows to return
    - `timeoutMs` (optional): Query timeout in milliseconds
    - `dryRun` (optional): Estimate query cost without execution

### Schema Exploration

- **list_projects**: List available projects
- **list_datasets**: List datasets in a project
  - Parameters:
    - `projectId`: Project ID
- **list_tables**: List tables in a dataset
  - Parameters:
    - `projectId`: Project ID
    - `datasetId`: Dataset ID
- **get_table_schema**: Get schema for a specific table
  - Parameters:
    - `projectId`: Project ID
    - `datasetId`: Dataset ID
    - `tableId`: Table ID

### Query Management

- **get_query_history**: Retrieve recent query history
  - Parameters:
    - `limit` (optional): Maximum number of queries to return
- **get_query_job**: Get details about a specific query job
  - Parameters:
    - `jobId`: Query job ID

## Best Practices

1. **Use Query Parameters**: Avoid SQL injection by using parameterized queries
2. **Limit Result Size**: Specify row limits to avoid large result sets
3. **Estimate Costs**: Use dry runs for expensive queries
4. **Explore Schema First**: Understand data structure before querying
5. **Use Efficient Joins**: Optimize join conditions for better performance
6. **Apply Filters Early**: Filter data as early as possible in the query
7. **Cache Results**: Store query results for frequently used queries
8. **Monitor Usage**: Track query patterns and optimize common queries

## Troubleshooting

### Common Issues

1. **Authentication Failures**
   - Verify service account key is valid and has necessary permissions
   - Check if service account has BigQuery access

2. **Query Timeout**
   - Increase timeout limit in configuration
   - Optimize query to reduce execution time
   - Consider creating materialized views for complex queries

3. **Access Denied**
   - Verify access control configuration
   - Check if table/dataset is included in allowed resources
   - Ensure service account has appropriate IAM roles

4. **Resource Exhaustion**
   - Reduce query complexity
   - Add more specific filters
   - Consider using partitioned tables

### Logging

Enable detailed logging for troubleshooting:

```json
{
  "logging": {
    "level": "debug",
    "file": "/path/to/logfile.log"
  }
}
```

## Security Considerations

1. **Least Privilege**: Grant minimal permissions to service accounts
2. **Data Masking**: Consider views that mask sensitive columns
3. **Query Validation**: Implement additional validation for user-generated queries
4. **Audit Logging**: Enable Cloud Audit Logs for BigQuery operations
5. **Cost Controls**: Set billing alerts and quotas to prevent unexpected charges

## Advanced Configuration

### Custom Query Templates

Define reusable query templates:

```json
{
  "queryTemplates": {
    "monthlySales": "SELECT date_trunc(order_date, month) as month, SUM(amount) as total FROM `{project}.{dataset}.orders` WHERE order_date BETWEEN '{start_date}' AND '{end_date}' GROUP BY month ORDER BY month",
    "customerSegment": "SELECT customer_segment, COUNT(*) as count FROM `{project}.{dataset}.customers` GROUP BY customer_segment"
  }
}
```

### Result Formatting

Configure how results are formatted:

```json
{
  "resultFormatting": {
    "dateFormat": "YYYY-MM-DD",
    "numberFormat": {
      "currency": "USD",
      "decimal": ".",
      "thousands": ","
    },
    "nullDisplay": "N/A"
  }
}
```

## Integration with Other MCP Servers

The BigQuery server works well with other MCP servers:

1. **Filesystem**: Export query results to CSV/JSON files
2. **Visualization**: Generate charts from query results
3. **Memory**: Store query results for future reference
4. **GitHub**: Version control for SQL queries

## Conclusion

The MCP BigQuery Server provides a powerful interface for AI assistants to interact with Google BigQuery. By following the guidelines in this document, you can set up a secure, efficient connection between your AI assistant and your data warehouse, enabling sophisticated data analysis and business intelligence capabilities.
