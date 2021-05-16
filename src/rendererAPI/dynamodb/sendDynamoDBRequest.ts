import AWS from "aws-sdk";
import { DynamoDBRequestType, SendDynamoDBRequestFn } from "./types";

export const sendDynamoDBRequest: SendDynamoDBRequestFn = async function ({
  profile,
  region,
  tableName,
  expressionAttributeNames,
  expressionAttributeValues,
  options,
}) {
  const credentials = new AWS.SharedIniFileCredentials({ profile });
  await credentials.getPromise();
  const dc = new AWS.DynamoDB.DocumentClient({ region, credentials });

  const parameters = {
    TableName: tableName,
    ExpressionAttributeNames: expressionAttributeNames,
    ExpressionAttributeValues: expressionAttributeValues,
    ...(options.type === DynamoDBRequestType.QUERY
      ? {
          KeyConditionExpression: options.keyConditionExpression,
        }
      : undefined),
  };
  try {
    const result = await (options.type === DynamoDBRequestType.QUERY
      ? dc.query(parameters).promise()
      : dc.scan(parameters).promise());
    return result.Items || [];
  } catch (err) {
    console.error(err);
    throw err;
  }
};
