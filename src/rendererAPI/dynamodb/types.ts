export interface DynamoDBRequest {
  profile: string;
  region: string;
  tableName: string;
  expressionAttributeNames?: Record<string, string>;
  expressionAttributeValues?: Record<string, any>;
  options: DynamoDBScanOptions | DynamoDBQueryOptions;
}

export enum DynamoDBRequestType {
  SCAN,
  QUERY,
}

export interface DynamoDBScanOptions {
  type: DynamoDBRequestType.SCAN;
}

export interface DynamoDBQueryOptions {
  type: DynamoDBRequestType.QUERY;
  keyConditionExpression: string;
}

export type SendDynamoDBRequestFn = (request: DynamoDBRequest) => Promise<object[]>;
