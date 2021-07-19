import AWS from "aws-sdk";
import { DynamoDBRequestType, SendDynamoDBRequestFn } from "./types";

export const sendDynamoDBRequest: SendDynamoDBRequestFn = async function ({
  profile,
  region,
  options,
}) {
  const credentials = new AWS.SharedIniFileCredentials({ profile });
  await credentials.getPromise();

  if (
    options.type === DynamoDBRequestType.SCAN ||
    options.type === DynamoDBRequestType.QUERY
  ) {
    const dc = new AWS.DynamoDB.DocumentClient({ region, credentials });
    const { tableName, expressionAttributeNames, expressionAttributeValues } =
      options;
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
  } else {
    const db = new AWS.DynamoDB({ region, credentials });
    const { statement } = options;
    try {
      const result = await db
        .executeStatement({
          Statement: statement,
        })
        .promise();
      return (result.Items || []).map((item) =>
        AWS.DynamoDB.Converter.unmarshall(item)
      );
    } catch (err) {
      console.error(err);
      throw err;
    }
  }
};
