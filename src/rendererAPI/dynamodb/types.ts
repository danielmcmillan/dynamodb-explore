export enum DynamoDBRequestType {
  SCAN,
  QUERY,
  PARTIQL,
}

export interface DynamoDBRequest {
  profile: string;
  region: string;
  options: DynamoDBScanOptions | DynamoDBQueryOptions | DynamoDBPartiQLOptions;
}

export interface DynamoDBScanOptions {
  type: DynamoDBRequestType.SCAN;
  tableName: string;
  expressionAttributeNames?: Record<string, string>;
  expressionAttributeValues?: Record<string, unknown>;
}

export interface DynamoDBQueryOptions {
  type: DynamoDBRequestType.QUERY;
  tableName: string;
  expressionAttributeNames?: Record<string, string>;
  expressionAttributeValues?: Record<string, unknown>;
  keyConditionExpression: string;
}

export interface DynamoDBPartiQLOptions {
  type: DynamoDBRequestType.PARTIQL;
  statement: string;
}

export type SendDynamoDBRequestFn = (
  request: DynamoDBRequest
) => Promise<Record<string, unknown>[]>;
