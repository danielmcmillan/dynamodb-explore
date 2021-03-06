import { contextBridge } from "electron";
import AWS from "aws-sdk";

const results = {};

contextBridge.exposeInMainWorld("api", {
  doQuery: async function () {
    const credentials = new AWS.SharedIniFileCredentials();
    await credentials.getPromise();
    console.log(`Access key id: ${credentials.accessKeyId}`);

    const result = await new AWS.DynamoDB.DocumentClient({
      region: "ap-southeast-2",
    })
      .scan({
        TableName: "cloud-chat-dev",
      })
      .promise();
    results.query = JSON.stringify(result.Items, undefined, 2);
  },
  getResult: function () {
    return results.query;
  },
});
