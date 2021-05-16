import { contextBridge } from "electron";
import AWS from "aws-sdk";

// contextBridge.exposeInMainWorld("api", {
//   dynamodb_request: async function (
//     profile,
//     region,
//     table_name,
//     expression_attribute_names,
//     expression_attribute_values,
//     query_input
//   ) {
//     console.log({
//       profile,
//       region,
//       table_name,
//       expression_attribute_names,
//       expression_attribute_values,
//       query_input,
//     });
//     const credentials = new AWS.SharedIniFileCredentials({ profile });
//     await credentials.getPromise();
//     const dc = new AWS.DynamoDB.DocumentClient({ region, credentials });

//     const parameters = {
//       TableName: table_name,
//       ExpressionAttributeNames: expression_attribute_names,
//       ExpressionAttributeValues: expression_attribute_values,
//       ...(query_input
//         ? {
//             KeyConditionExpression: query_input.key_condition_expression,
//           }
//         : undefined),
//     };
//     try {
//       const result = await (query_input
//         ? dc.query(parameters).promise()
//         : dc.scan(parameters).promise());
//       return (result.Items || []).map((item) => item);
//     } catch (err) {
//       console.error(err);
//     } finally {
//       return;
//     }
//   },
// });
