import { contextBridge } from "electron";
import AWS from "aws-sdk";

contextBridge.exposeInMainWorld("api", {
  dynamodb_request: async function () {
    const credentials = new AWS.SharedIniFileCredentials();
    await credentials.getPromise();

    const result = await new AWS.DynamoDB.DocumentClient({
      region: "ap-southeast-2",
    })
      .scan({
        TableName: "cloud-chat-dev",
      })
      .promise();
    return JSON.stringify(result.Items, undefined, 2);
  },
});
